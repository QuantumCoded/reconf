use crate::{error::Error, profile::ProfileData, relative_path::*};
use std::fs::{write, File};
use std::{collections::HashSet, io::Read};

pub fn add(profile: RelativePath, dir: Dir, component: RelativePath) -> Result<(), Error> {
    let mut buf = String::new();
    let path = profile.resolve(Dir::Profiles, "toml")?;
    let mut used = HashSet::new();

    File::open(&path)?.read_to_string(&mut buf)?;

    let mut data: ProfileData = toml::from_str(&buf)?;
    let components = match dir {
        Dir::Helpers => &mut data.helpers,
        Dir::Modules => &mut data.modules,
        Dir::Templates => &mut data.templates,
        _ => unreachable!(),
    };

    for component in components.iter() {
        let component = RelativePath::from(component.to_path_buf()).resolve(dir, dir.ext_str())?;

        if used.contains(&component) {
            return Err(
                Error::DupeProfVal(dir.component_str().to_string(), component, path).into(),
            );
        }

        used.insert(component);
    }

    let component_path = component.resolve(dir, dir.ext_str())?;

    if used.contains(&component_path) {
        return Err(
            Error::UsedProfVal(dir.component_str().to_string(), component_path, path).into(),
        );
    }

    components.push(component.path().to_path_buf());

    write(path, toml::to_string(&data)?)?;

    Ok(())
}

pub fn rm(profile: RelativePath, dir: Dir, component: RelativePath) -> Result<(), Error> {
    let mut buf = String::new();
    let path = profile.resolve(Dir::Profiles, "toml")?;
    let mut used = HashSet::new();

    File::open(&path)?.read_to_string(&mut buf)?;

    let mut data: ProfileData = toml::from_str(&buf)?;
    let components = match dir {
        Dir::Helpers => &mut data.helpers,
        Dir::Modules => &mut data.modules,
        Dir::Templates => &mut data.templates,
        _ => unreachable!(),
    };

    let component_path = component.resolve_unchecked(dir, dir.ext_str())?;
    let mut target_component = None;

    for (idx, prof_component) in components.iter().enumerate() {
        let prof_component_path = RelativePath::from(prof_component.to_path_buf())
            .resolve_unchecked(dir, dir.ext_str())?;

        if prof_component_path == component_path {
            target_component = Some(idx);
        }

        if used.contains(&prof_component_path) {
            return Err(Error::DupeProfVal(
                dir.component_str().to_string(),
                prof_component_path,
                path,
            )
            .into());
        }

        used.insert(prof_component_path);
    }

    components.remove(target_component.ok_or(Error::NotInProfile(
        dir.component_str().to_string(),
        component_path,
        path.clone(),
    ))?);

    write(path, toml::to_string(&data)?)?;

    Ok(())
}
