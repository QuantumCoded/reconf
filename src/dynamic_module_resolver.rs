use crate::error::Error;
use rhai::{Engine, EvalAltResult, Module, ModuleResolver, Position, Scope, Shared};
use std::{collections::HashMap, path::PathBuf, sync::Arc};

pub struct DynamicModuleResolver(HashMap<String, PathBuf>);

impl DynamicModuleResolver {
    pub fn new(modules: &[PathBuf]) -> Result<Self, Error> {
        let mut map = HashMap::new();

        for module in modules {
            map.insert(
                module
                    .file_stem()
                    .ok_or(Error::NoEmptyFileName)?
                    .to_string_lossy()
                    .into_owned(),
                module.to_path_buf(),
            );
        }

        Ok(Self(map))
    }
}

impl ModuleResolver for DynamicModuleResolver {
    fn resolve(
        &self,
        engine: &Engine,
        _source: Option<&str>,
        path: &str,
        pos: Position,
    ) -> Result<Shared<Module>, Box<EvalAltResult>> {
        let ast = engine.compile_file(
            self.0
                .get(path)
                .ok_or(Box::new(EvalAltResult::ErrorModuleNotFound(
                    path.to_string(),
                    pos,
                )))?
                .to_path_buf(),
        )?;

        Ok(Arc::new(Module::eval_ast_as_new(
            Scope::new(),
            &ast,
            &engine,
        )?))
    }
}
