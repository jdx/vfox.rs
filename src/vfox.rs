use std::path::PathBuf;
use crate::plugin::Plugin;

pub struct Vfox {
    pub runtime_version: String,
    pub plugin_dir: PathBuf,
}

impl Vfox {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn list_plugins() -> Vec<Plugin> {
        vec![]
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
