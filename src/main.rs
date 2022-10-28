use cli::Action::*;
use profile::Profile;
use relative_path::Dir;
use std::{fs::File, io::copy, path::PathBuf};
use tar::Archive;

mod cli;
mod component;
mod dynamic_module_resolver;
mod error;
mod profile;
mod relative_path;

#[allow(unused_variables)]
fn main() -> main_error::MainResult {
    let action = cli::main()?;

    match action {
        // TODO: Add a [settings] table in profiles for storing universal per-profile constants to be changed from CLI
        AddHelper { profile, helper } => component::add(profile, Dir::Helpers, helper)?,
        AddModule { profile, module } => component::add(profile, Dir::Modules, module)?,
        AddTemplate { profile, template } => component::add(profile, Dir::Templates, template)?,
        ApplyProfile { profile } => Profile::open(profile)?.apply()?,
        Nothing => {}
        Restore => {
            let file = File::open(Dir::Config.as_base()?.join("backup.tar"))?;
            let mut archive = Archive::new(file);

            for file in archive.entries()? {
                let mut file = file?;
                let path = PathBuf::from("/").join(file.path()?);

                copy(&mut file, &mut File::options().write(true).open(path)?)?;
            }
        }
        RmHelper { profile, helper } => component::rm(profile, Dir::Helpers, helper)?,
        RmModule { profile, module } => component::rm(profile, Dir::Modules, module)?,
        RmTemplate { profile, template } => component::rm(profile, Dir::Templates, template)?,
    }

    Ok(())
}
