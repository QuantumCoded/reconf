use crate::error::Error;
use std::ffi::OsString;
use std::path::{Path, PathBuf};

#[derive(Copy, Clone)]
pub enum Dir {
    Helpers,
    Modules,
    Profiles,
    Templates,
    Config,
}

impl Dir {
    pub fn path_str(&self) -> &str {
        match self {
            Dir::Helpers => "helpers",
            Dir::Modules => "modules",
            Dir::Profiles => "profiles",
            Dir::Templates => "templates",
            Dir::Config => ".",
        }
    }

    pub fn ext_str(&self) -> &str {
        match self {
            Dir::Helpers => "rhai",
            Dir::Modules => "rhai",
            Dir::Profiles => "toml",
            Dir::Templates => "hbs",
            _ => "",
        }
    }

    pub fn component_str(&self) -> &str {
        match self {
            Dir::Helpers => "helper",
            Dir::Modules => "module",
            Dir::Profiles => "profile",
            Dir::Templates => "template",
            Dir::Config => "internal",
        }
    }

    pub fn as_base(&self) -> Result<PathBuf, Error> {
        let mut config = dirs::config_dir().ok_or(Error::NoConfigDir)?;

        config.push("reconf");
        config.push(self.path_str());

        Ok(config)
    }
}

pub struct RelativePath(PathBuf);

impl From<PathBuf> for RelativePath {
    fn from(path: PathBuf) -> Self {
        Self(path)
    }
}

impl RelativePath {
    pub fn resolve(&self, dir: Dir, ext: impl AsRef<str>) -> Result<PathBuf, Error> {
        // make path absolute
        let mut path = if self.0.is_absolute() {
            self.0.to_path_buf()
        } else {
            let mut dir = dir.as_base()?;
            dir.push(&self.0);
            dir
        };

        // if the path exists we're done
        if path.exists() {
            return Ok(path);
        }

        // if the path has the right extension it's not found
        if path.extension() == Some(&OsString::from(ext.as_ref())) {
            return Err(Error::NotFound(dir.component_str().to_string(), path));
        }

        // append the provided file extension and recheck
        let mut new_ext = OsString::new();

        if let Some(ext) = path.extension() {
            new_ext.push(ext);
            new_ext.push(".");
        }

        new_ext.push(ext.as_ref());
        path.set_extension(new_ext);

        // if the path exists we're done
        if path.exists() {
            return Ok(path);
        }

        Err(Error::NotFound(dir.component_str().to_string(), path))
    }

    pub fn path(&self) -> &Path {
        &self.0
    }
}
