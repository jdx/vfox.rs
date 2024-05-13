use std::path::PathBuf;

use itertools::Itertools;

use crate::error::Result;
use crate::plugin::Plugin;

pub struct Vfox {
    pub runtime_version: String,
    pub plugin_dir: PathBuf,
}

impl Vfox {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn list_sdks(&self) -> Result<Vec<Plugin>> {
        if !self.plugin_dir.exists() {
            return Ok(Default::default());
        }
        let plugins = xx::file::ls(&self.plugin_dir)?;
        plugins
            .into_iter()
            .filter_map(|p| {
                p.file_name()
                    .and_then(|f| f.to_str())
                    .map(|s| s.to_string())
            })
            .sorted()
            .map(|name| self.get_sdk(&name))
            .collect()
    }

    pub fn get_sdk(&self, name: &str) -> Result<Plugin> {
        Plugin::from_dir(&self.plugin_dir.join(name))
    }
}

impl Default for Vfox {
    fn default() -> Self {
        Self {
            runtime_version: "1.0.0".to_string(),
            plugin_dir: home().join(".version-fox/plugin"),
        }
    }
}

fn home() -> PathBuf {
    homedir::get_my_home()
        .ok()
        .flatten()
        .unwrap_or_else(|| PathBuf::from("/"))
}
