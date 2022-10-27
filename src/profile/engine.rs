use crate::dynamic_module_resolver::DynamicModuleResolver;
use crate::error::Error;
use dashmap::DashMap;
use rhai::{Array, Dynamic, Engine};
use std::{path::PathBuf, process::Command, sync::Arc};

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

pub fn build(
    modules: &[PathBuf],
) -> Result<(Engine, Arc<DashMap<PathBuf, (String, Dynamic)>>), Error> {
    let mut engine = Engine::new();
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
                // FIXME: Make this use better error
                panic!("attempt to template twice {:?}", &path);
            }

            template_map.insert(path, (name, data));
        }
    });

    // load module resolver
    engine.set_module_resolver(DynamicModuleResolver::new(&modules)?);

    Ok((engine, template_map))
}
