use crate::error::Error;
use crate::relative_path::{Dir, RelativePath};
use dashmap::DashMap;
use handlebars::Handlebars;
use rhai::{Dynamic, Engine};
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};
use std::{collections::BTreeMap, fs, path::PathBuf, sync::Arc};
use toml::Value;

mod apply;
mod engine;
mod open;

pub struct Profile {
    engine: Engine,
    registry: Handlebars<'static>,
    modules: Vec<PathBuf>,
    template_map: Arc<DashMap<PathBuf, (String, Dynamic)>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProfileData {
    #[serde(rename = "profile")]
    pub inner: ProfileDataInner,
    pub settings: BTreeMap<String, Value>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProfileDataInner {
    pub name: String,
    #[serde(default)]
    pub modules: Vec<PathBuf>,
    #[serde(default)]
    pub helpers: Vec<PathBuf>,
    #[serde(default)]
    pub templates: Vec<PathBuf>,
}

pub fn set_setting(profile: RelativePath, name: String, value: Value) -> Result<(), Error> {
    let path = profile.resolve(Dir::Profiles, "toml")?;
    let mut data: ProfileData = toml::from_str(&fs::read_to_string(&path)?)?;
    data.settings.insert(name, value);
    fs::write(path, toml::to_string(&data)?)?;
    Ok(())
}

pub fn rm_setting(profile: RelativePath, name: String) -> Result<(), Error> {
    let path = profile.resolve(Dir::Profiles, "toml")?;
    let mut data: ProfileData = toml::from_str(&fs::read_to_string(&path)?)?;
    data.settings
        .remove(&name)
        .ok_or(Error::SettingNotFound(name, path.to_owned()))?;
    fs::write(&path, toml::to_string(&data)?)?;
    Ok(())
}

impl Deref for ProfileData {
    type Target = ProfileDataInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for ProfileData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
