use crate::dynamic_module_resolver::DynamicModuleResolver;
use crate::{error::Error, relative_path::*};
use dashmap::DashMap;
use handlebars::Handlebars;
use rhai::{Dynamic, Engine};
use serde::{Deserialize, Serialize};
use std::fs::{write, File};
use std::{collections::HashMap, io::Read, path::PathBuf, process::Command, sync::Arc};
use tar::Builder;

#[derive(Serialize, Deserialize, Debug)]
pub struct Profile {
    #[serde(rename = "profile")]
    pub inner: ProfileData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProfileData {
    pub name: String,
    #[serde(default)]
    pub modules: Vec<PathBuf>,
    #[serde(default)]
    pub helpers: Vec<PathBuf>,
    #[serde(default)]
    pub templates: Vec<PathBuf>,
}

pub fn apply(profile: RelativePath) -> Result<(), Error> {
    let mut engine = Engine::new();
    let mut registry = Handlebars::new();
    let templates_map = Arc::new(DashMap::new());
    let mut buf = String::new();
    let profile = profile.resolve(Dir::Profiles, "toml")?;

    File::open(&profile)?.read_to_string(&mut buf)?;

    let prof: Profile = toml::from_str(&buf)?;

    engine.register_fn(
        "command",
        |program: String, args: Option<Vec<String>>| match Command::new(program)
            .args(args.unwrap_or_default())
            .output()
        {
            _ => {}
        },
    );

    engine.register_fn("template", {
        let template = Arc::clone(&templates_map);
        move |path: String, name: String, data: Dynamic| {
            let path = PathBuf::from(path);

            assert!(
                path.is_absolute(),
                "templating to relative paths is unsupported {:?}",
                &path
            );

            if template.contains_key(&path) {
                // FIXME: Make this use better error
                panic!("attempt to template twice {:?}", &path);
            }

            template.insert(path, (name, data));
        }
    });

    let modules = prof
        .inner
        .modules
        .into_iter()
        .map(|path| -> Result<_, _> { Ok(RelativePath::from(path).resolve(Dir::Modules, "rhai")?) })
        .collect::<Result<Vec<_>, Error>>()?;

    let helpers = prof
        .inner
        .helpers
        .into_iter()
        .map(|path| -> Result<_, _> { Ok(RelativePath::from(path).resolve(Dir::Helpers, "rhai")?) })
        .collect::<Result<Vec<_>, Error>>()?;

    let templates = prof
        .inner
        .templates
        .into_iter()
        .map(|path| -> Result<_, _> {
            Ok(RelativePath::from(path).resolve(Dir::Templates, "hbs")?)
        })
        .collect::<Result<Vec<_>, Error>>()?;

    // load module resolver
    engine.set_module_resolver(DynamicModuleResolver::new(&modules)?);

    // load helpers into Registry
    for helper in helpers {
        registry
            .register_script_helper_file(
                &helper
                    .file_stem()
                    .ok_or(Error::NoEmptyFileName)?
                    .to_string_lossy()
                    .into_owned(),
                helper,
            )
            .map_err(|err| Error::ScriptHelperError(err.into()))?;
    }

    // load templates into Registry
    for template in templates {
        registry.register_template_file(
            &template
                .file_stem()
                .ok_or(Error::NoEmptyFileName)?
                .to_string_lossy()
                .into_owned(),
            template,
        )?;
    }

    // evaluate modules
    for module in modules {
        engine
            .eval_file(module.clone())
            .map_err(|err| Error::RhaiModuleError(module, err.into()))?;
    }

    let mut template_map = HashMap::new();

    // generate templates
    for template_data_ref in templates_map.iter() {
        let path = template_data_ref.key().to_path_buf();
        let (name, data) = template_data_ref.value();

        template_map.insert(path, registry.render(name, data)?);
    }

    let file = File::create(Dir::Config.as_base()?.join("backup.tar"))?;
    let mut backup = Builder::new(file);

    // backup files
    for (abs_path, _) in template_map.iter() {
        if !abs_path.exists() {
            continue;
        }

        let path = abs_path.components().skip(1).collect::<PathBuf>();

        backup.append_file(&path, &mut File::open(&abs_path)?)?;
    }

    // write config
    for (path, data) in template_map.into_iter() {
        match write(&path, data) {
            Err(err) => {
                println!("WARNING! Failed to write config file {:?}\n{}", path, err)
            }
            Ok(()) => {}
        };
    }

    Ok(())
}
