use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use itertools::Itertools;
use reqwest::Url;
use xx::file;

use crate::error::Result;
use crate::hooks::available::AvailableVersion;
use crate::hooks::env_keys::{EnvKey, EnvKeysContext};
use crate::hooks::parse_legacy_file::ParseLegacyFileResponse;
use crate::hooks::pre_install::PreInstall;
use crate::metadata::Metadata;
use crate::plugin::Plugin;
use crate::registry;
use crate::sdk_info::SdkInfo;

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

    pub async fn list_available_versions(&self, sdk: &str) -> Result<Vec<AvailableVersion>> {
        let sdk = self.get_sdk(sdk)?;
        sdk.available_async().await
    }

    pub fn list_installed_versions(&self, sdk: &str) -> Result<Vec<SdkInfo>> {
        let path = self.install_dir.join(sdk);
        if !path.exists() {
            return Ok(Default::default());
        }
        let versions = xx::file::ls(&path)?;
        Ok(versions
            .into_iter()
            .filter_map(|p| {
                p.file_name()
                    .and_then(|f| f.to_str())
                    .map(|s| s.to_string())
            })
            .sorted()
            .map(|version| SdkInfo::new(sdk.to_string(), version.to_string(), path.join(&version)))
            .collect())
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

    pub fn install_plugin(&self, sdk: &str) -> Result<Plugin> {
        let plugin_dir = self.plugin_dir.join(sdk);
        if !plugin_dir.exists() {
            let url = registry::sdk_url(sdk).ok_or_else(|| format!("Unknown SDK: {sdk}"))?;
            return self.install_plugin_from_url(url);
        }
        Plugin::from_dir(&plugin_dir)
    }

    pub fn install_plugin_from_url(&self, url: &Url) -> Result<Plugin> {
        let sdk = url
            .path_segments()
            .and_then(|s| {
                let filename = s.last().unwrap();
                filename
                    .strip_prefix("vfox-")
                    .map(|s| s.to_string())
                    .or_else(|| Some(filename.to_string()))
            })
            .ok_or("No filename in URL")?;
        let plugin_dir = self.plugin_dir.join(&sdk);
        if !plugin_dir.exists() {
            debug!("Installing plugin {sdk}");
            xx::git::clone(url.as_ref(), &plugin_dir)?;
        }
        Plugin::from_dir(&plugin_dir)
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

    pub fn uninstall(&self, sdk: &str, version: &str) -> Result<()> {
        let path = self.install_dir.join(sdk).join(version);
        file::remove_dir_all(&path)?;
        Ok(())
    }

    pub async fn metadata(&self, sdk: &str) -> Result<Metadata> {
        self.get_sdk(sdk)?.get_metadata()
    }

    pub async fn env_keys(&self, sdk: &str, version: &str) -> Result<Vec<EnvKey>> {
        debug!("Getting env keys for {sdk} version {version}");
        let plugin = self.get_sdk(sdk)?;
        let path = self.install_dir.join(sdk).join(version);
        let sdk_info = SdkInfo::new(sdk.to_string(), version.to_string(), path.clone());
        let ctx = EnvKeysContext {
            args: vec![],
            version: version.to_string(),
            path,
            sdk_info: BTreeMap::from([(sdk.to_string(), sdk_info.clone())]),
            main: sdk_info,
        };
        plugin.env_keys(ctx).await
    }

    pub async fn parse_legacy_file(
        &self,
        sdk: &str,
        file: &Path,
    ) -> Result<ParseLegacyFileResponse> {
        let sdk = self.get_sdk(sdk)?;
        sdk.parse_legacy_file(file).await
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
            if subdirs.len() == 1 && subdirs.first().unwrap().is_dir() {
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
            xx::archive::unzip(file, &tmp)?;
            move_to_install()?;
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
    homedir::my_home()
        .ok()
        .flatten()
        .unwrap_or_else(|| PathBuf::from("/"))
}

#[cfg(test)]
mod tests {
    use super::*;

    impl Vfox {
        pub fn test() -> Self {
            Self {
                runtime_version: "1.0.0".to_string(),
                plugin_dir: PathBuf::from("plugins"),
                cache_dir: PathBuf::from("test/cache"),
                download_dir: PathBuf::from("test/downloads"),
                install_dir: PathBuf::from("test/installs"),
                temp_dir: PathBuf::from("test/temp"),
            }
        }
    }

    #[tokio::test]
    async fn test_env_keys() {
        let vfox = Vfox::test();
        vfox.install_plugin("nodejs").unwrap();
        let keys = vfox.env_keys("nodejs", "20.0.0").await.unwrap();
        let output = format!("{:?}", keys).replace(
            &vfox.install_dir.to_string_lossy().to_string(),
            "<INSTALL_DIR>",
        );
        assert_snapshot!(output);
    }

    #[tokio::test]
    async fn test_install_plugin() {
        let vfox = Vfox::test();
        vfox.uninstall_plugin("nodejs").unwrap();
        assert!(!vfox.plugin_dir.join("nodejs").exists());
        vfox.install_plugin("nodejs").unwrap();
        assert!(vfox.plugin_dir.join("nodejs").exists());
    }

    #[tokio::test]
    async fn test_install() {
        let vfox = Vfox::test();
        let install_dir = vfox.install_dir.join("nodejs").join("20.0.0");
        vfox.install("nodejs", "20.0.0", &install_dir)
            .await
            .unwrap();
        assert!(vfox
            .install_dir
            .join("nodejs")
            .join("20.0.0")
            .join("bin")
            .join("node")
            .exists());
        vfox.uninstall_plugin("nodejs").unwrap();
        assert!(!vfox.plugin_dir.join("nodejs").exists());
        vfox.uninstall("nodejs", "20.0.0").unwrap();
        assert!(!vfox.install_dir.join("nodejs").join("20.0.0").exists());
        file::remove_dir_all(vfox.plugin_dir.join("nodejs")).unwrap();
        file::remove_dir_all(vfox.install_dir).unwrap();
        file::remove_dir_all(vfox.download_dir).unwrap();
    }

    #[tokio::test]
    async fn test_install_cmake() {
        let vfox = Vfox::test();
        vfox.install_plugin("cmake").unwrap();
        let install_dir = vfox.install_dir.join("cmake").join("3.21.0");
        vfox.install("cmake", "3.21.0", &install_dir).await.unwrap();
        assert!(vfox
            .install_dir
            .join("cmake")
            .join("3.21.0")
            .join("bin")
            .join("cmake")
            .exists());
        vfox.uninstall_plugin("cmake").unwrap();
        assert!(!vfox.plugin_dir.join("cmake").exists());
        vfox.uninstall("cmake", "3.21.0").unwrap();
        assert!(!vfox.install_dir.join("cmake").join("3.21.0").exists());
        file::remove_dir_all(vfox.plugin_dir.join("cmake")).unwrap();
        file::remove_dir_all(vfox.install_dir).unwrap();
        file::remove_dir_all(vfox.download_dir).unwrap();
    }

    #[tokio::test]
    async fn test_metadata() {
        let vfox = Vfox::test();
        let metadata = vfox.metadata("nodejs").await.unwrap();
        let out = format!("{:?}", metadata);
        assert_snapshot!(out);
    }
}
