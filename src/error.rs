use std::path::PathBuf;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("could not find config directory with XDG lookup")]
    NoConfigDir,

    #[error("file names must not be empty")]
    NoEmptyFileName,

    #[error("could not find {0} {1:?}")]
    NotFound(String, PathBuf),

    #[error("{0} {1:?} is not in profile {2:?}")]
    NotInProfile(String, PathBuf, PathBuf),

    #[error("{0} {1:?} is used already used in profile {2:?}")]
    UsedProfVal(String, PathBuf, PathBuf),

    #[error("{0} {1:?} is used twice in profile {2:?}")]
    DupeProfVal(String, PathBuf, PathBuf),

    #[error("attempted to template {0:?} after already templating")]
    TemplateTwice(PathBuf),

    #[error("toml deserialize error")]
    TomlDeError(#[from] toml::de::Error),

    #[error("toml serialize error")]
    TomlSeError(#[from] toml::ser::Error),

    #[error("io error")]
    IoError(#[from] std::io::Error),

    #[error("handlebars template error")]
    TemplateError(#[from] handlebars::TemplateError),

    #[error("handlebars render error")]
    RenderError(#[from] handlebars::RenderError),

    #[error("handlebars script helper error\ncaused by: {0}")]
    ScriptHelperError(Box<dyn std::error::Error>),

    #[error("error in rhai module {0:?}\n{1}")]
    RhaiModuleError(PathBuf, Box<dyn std::error::Error>),

    #[error("there is no setting with the name {0:?} in the profile {1:?}")]
    SettingNotFound(String, PathBuf),
}
