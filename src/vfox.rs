use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use itertools::Itertools;
use reqwest::Url;
use xx::file;

use crate::error::Result;
use crate::hooks::pre_install::PreInstall;
use crate::plugin::Plugin;
use crate::registry;

#[derive(Debug)]
pub struct Vfox {
    pub runtime_version: String,
    pub install_dir: PathBuf,
    pub plugin_dir: PathBuf,
    pub cache_dir: PathBuf,
    pub download_dir: PathBuf,
    pub temp_dir: PathBuf,
}

impl Vfox {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn list_available_sdks() -> &'static BTreeMap<String, Url> {
        registry::list_sdks()
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

    pub fn install_plugin(&self, sdk: &str) -> Result<()> {
        let plugin_dir = self.plugin_dir.join(sdk);
        if !plugin_dir.exists() {
            let url = registry::sdk_url(sdk).ok_or_else(|| format!("Unknown SDK: {sdk}"))?;
            xx::git::clone(url.as_ref(), &plugin_dir)?;
        }
        Ok(())
    }

    pub fn uninstall_plugin(&self, sdk: &str) -> Result<()> {
        let plugin_dir = self.plugin_dir.join(sdk);
        if plugin_dir.exists() {
            file::remove_dir_all(&plugin_dir)?;
        }
        Ok(())
    }

    pub async fn install<ID: AsRef<Path>>(
        &self,
        sdk: &str,
        version: &str,
        install_dir: ID,
    ) -> Result<()> {
        self.install_plugin(sdk)?;
        let sdk = self.get_sdk(sdk)?;
        let pre_install = sdk.pre_install(version).await?;
        let url = Url::from_str(pre_install.url.as_ref().unwrap())?;
        let file = self.download(&url, &sdk, version).await?;
        self.verify(&pre_install, &file)?;
        self.extract(&file, install_dir.as_ref())?;
        Ok(())
    }

    async fn download(&self, url: &Url, sdk: &Plugin, version: &str) -> Result<PathBuf> {
        info!("Downloading {url}");
        let filename = url
            .path_segments()
            .and_then(|s| s.last())
            .ok_or("No filename in URL")?;
        let file = self
            .download_dir
            .join(format!("{sdk}-{version}"))
            .join(filename);
        xx::http::download(url.clone(), &file).await?;
        Ok(file)
    }

    fn verify(&self, pre_install: &PreInstall, file: &Path) -> Result<()> {
        info!("Verifying {file:?} checksum");
        if let Some(sha256) = &pre_install.sha256 {
            xx::hash::ensure_checksum_sha256(file, sha256)?;
        }
        if let Some(sha512) = &pre_install.sha512 {
            xx::hash::ensure_checksum_sha512(file, sha512)?;
        }
        if let Some(_sha1) = &pre_install.sha1 {
            unimplemented!("sha1")
        }
        if let Some(_md5) = &pre_install.md5 {
            unimplemented!("md5")
        }
        Ok(())
    }

    fn extract(&self, file: &Path, install_dir: &Path) -> Result<()> {
        info!("Extracting {file:?} to {install_dir:?}");
        let filename = file.file_name().unwrap().to_string_lossy().to_string();
        let tmp = self.temp_dir.join(&filename);
        file::remove_dir_all(&tmp)?;
        file::remove_dir_all(install_dir)?;
        let move_to_install = || {
            let subdirs = file::ls(&tmp)?;
            if subdirs.len() == 1 {
                let subdir = subdirs.first().unwrap();
                file::mv(subdir, install_dir)?;
                file::remove_dir_all(&tmp)?;
            } else {
                file::mv(&tmp, install_dir)?;
            }
            Result::Ok(())
        };
        if filename.ends_with(".tar.gz") {
            xx::archive::untar_gz(file, &tmp)?;
            move_to_install()?;
        } else if filename.ends_with(".tar.xz") {
            xx::archive::untar_xz(file, &tmp)?;
            move_to_install()?;
        } else if filename.ends_with(".tar.bz2") {
            xx::archive::untar_bz2(file, &tmp)?;
            move_to_install()?;
        } else if filename.ends_with(".zip") {
            xx::archive::unzip(file, install_dir)?;
        } else {
            unimplemented!("Unsupported extension {file:?}");
        }
        Ok(())
    }
}

impl Default for Vfox {
    fn default() -> Self {
        Self {
            runtime_version: "1.0.0".to_string(),
            plugin_dir: home().join(".version-fox/plugin"),
            cache_dir: home().join(".version-fox/cache"),
            download_dir: home().join(".version-fox/downloads"),
            install_dir: home().join(".version-fox/installs"),
            temp_dir: home().join(".version-fox/temp"),
        }
    }
}

fn home() -> PathBuf {
    homedir::get_my_home()
        .ok()
        .flatten()
        .unwrap_or_else(|| PathBuf::from("/"))
}
