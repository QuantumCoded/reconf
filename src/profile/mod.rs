use dashmap::DashMap;
use handlebars::Handlebars;
use rhai::{Dynamic, Engine};
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};
use std::{path::PathBuf, sync::Arc};

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
