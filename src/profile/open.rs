use super::{engine, Profile, ProfileData};
use crate::{error::Error, relative_path::*};
use handlebars::Handlebars;
use std::{fs::File, io::Read, path::PathBuf};

fn resolve_path_vec(vec: &[PathBuf], dir: Dir, ext: &str) -> Result<Vec<PathBuf>, Error> {
    Ok(vec
        .iter()
        .map(|path| -> Result<_, _> {
            Ok(RelativePath::from(path.to_path_buf()).resolve(dir, ext)?)
        })
        .collect::<Result<Vec<_>, Error>>()?)
}

impl Profile {
    pub fn open(path: RelativePath) -> Result<Profile, Error> {
        let mut buf = String::new();

        File::open(path.resolve(Dir::Profiles, "toml")?)?.read_to_string(&mut buf)?;

        let data: ProfileData = toml::from_str(&buf)?;
        let mut registry = Handlebars::new();
        let helpers = resolve_path_vec(&data.helpers, Dir::Helpers, "rhai")?;
        let modules = resolve_path_vec(&data.modules, Dir::Modules, "rhai")?;
        let templates = resolve_path_vec(&data.templates, Dir::Templates, "hbs")?;
        let (engine, template_map) = engine::build(&modules, data.settings)?;

        // load helpers into Registry
        for helper in &helpers {
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
        for template in &templates {
            registry.register_template_file(
                &template
                    .file_stem()
                    .ok_or(Error::NoEmptyFileName)?
                    .to_string_lossy()
                    .into_owned(),
                template,
            )?;
        }

        Ok(Profile {
            engine,
            registry,
            modules,
            template_map,
        })
    }
}
