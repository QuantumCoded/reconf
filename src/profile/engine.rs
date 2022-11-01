use crate::dynamic_module_resolver::DynamicModuleResolver;
use crate::error::Error;
use dashmap::DashMap;
use rhai::{Array, Dynamic, Engine, Module};
use std::{collections::BTreeMap, path::PathBuf, process::Command, sync::Arc};
use toml::Value;

fn run_command(program: String, args: Option<Array>) {
    match Command::new(program)
        .args(
            args.unwrap_or_default()
                .into_iter()
                .map(|arg| arg.to_string())
                .collect::<Vec<_>>(),
        )
        .status()
    {
        _ => {}
    }
}

fn parse_value(value: Value) -> Option<Dynamic> {
    match value.type_str() {
        "string" => value.as_str().map(|s| Dynamic::from(s.to_owned())),
        "integer" => value.as_integer().map(Dynamic::from_int),
        "float" => value.as_float().map(Dynamic::from_float),
        "boolean" => value.as_bool().map(Dynamic::from_bool),
        "datetime" => {
            todo!("parsing TOML datetimes is currently not supported, feel free to make a PR")
        }
        "array" => value.as_array().map(|vec| -> Option<Dynamic> {
            Some(Dynamic::from_array(
                vec.into_iter()
                    .map(|value| parse_value(value.to_owned()))
                    .collect::<Option<Vec<_>>>()?,
            ))
        })?,
        "table" => value.as_table().map(|map| -> Option<Dynamic> {
            Some(Dynamic::from_map(
                map.to_owned()
                    .into_iter()
                    .map(|(key, value)| parse_value(value).map(|value| (key.into(), value)))
                    .collect::<Option<BTreeMap<_, _>>>()?,
            ))
        })?,
        _ => unreachable!(),
    }
}

pub fn build(
    modules: &[PathBuf],
    settings: BTreeMap<String, Value>,
) -> Result<(Engine, Arc<DashMap<PathBuf, (String, Dynamic)>>), Error> {
    let mut engine = Engine::new();
    let mut settings_mod = Module::new();

    let template_map = Arc::new(DashMap::new());

    engine.register_fn("command", |program: String| {
        run_command(program, None);
    });

    engine.register_fn("command", |program: String, args: Array| {
        run_command(program, Some(args));
    });

    engine.register_fn("template", {
        let template_map = Arc::clone(&template_map);

        move |path: String, name: String, data: Dynamic| {
            let path = PathBuf::from(path);

            assert!(
                path.is_absolute(),
                "templating to relative paths is unsupported {:?}",
                &path
            );

            if template_map.contains_key(&path) {
                eprintln!("{}", Error::TemplateTwice(path));
                panic!("templating twice creates race condition");
            }

            template_map.insert(path, (name, data));
        }
    });

    engine.register_fn("setting", {
        let settings = settings.clone();

        move |name: String| match settings
            .get(&name)
            .map(|value| parse_value(value.to_owned()))
        {
            Some(Some(setting)) => setting,
            _ => Dynamic::UNIT,
        }
    });

    for (name, value) in settings {
        match parse_value(value) {
            Some(value) => settings_mod.set_var(name, value),
            None => continue,
        };
    }

    engine.register_static_module("settings", Arc::new(settings_mod));

    // load module resolver
    engine.set_module_resolver(DynamicModuleResolver::new(&modules)?);

    Ok((engine, template_map))
}
