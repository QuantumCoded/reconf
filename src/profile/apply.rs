use super::Profile;
use crate::{error::Error, relative_path::*};
use std::fs::{write, File};
use std::{collections::HashMap, path::PathBuf};
use tar::Builder;

impl Profile {
    pub fn apply(&self) -> Result<(), Error> {
        let mut rendered_template_map = HashMap::new();

        // evaluate modules
        for module in &self.modules {
            self.engine
                .eval_file(module.to_path_buf())
                .map_err(|err| Error::RhaiModuleError(module.to_path_buf(), err.into()))?;
        }

        // generate templates
        for templating_data_ref in self.template_map.iter() {
            let path = templating_data_ref.key().to_path_buf();
            let (name, data) = templating_data_ref.value();

            rendered_template_map.insert(path, self.registry.render(name, data)?);
        }

        let mut backup = Builder::new(File::create(Dir::Config.as_base()?.join("backup.tar"))?);

        // backup files
        for (abs_path, _) in rendered_template_map.iter() {
            if !abs_path.exists() {
                continue;
            }

            let path = abs_path.components().skip(1).collect::<PathBuf>();

            backup.append_file(&path, &mut File::open(&abs_path)?)?;
        }

        // write config
        for (path, data) in rendered_template_map.into_iter() {
            match write(&path, data) {
                Err(err) => {
                    eprintln!("WARNING! Failed to write config file {:?}\n{}", path, err)
                }
                Ok(()) => {}
            };
        }

        Ok(())
    }
}
