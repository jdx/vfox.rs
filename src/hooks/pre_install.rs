use mlua::prelude::LuaError;
use mlua::{FromLua, Lua, Value};

use crate::error::Result;
use crate::Plugin;

impl Plugin {
    pub async fn pre_install(&self, version: &str) -> Result<PreInstall> {
        debug!("[vfox:{}] pre_install", &self.name);
        let ctx = self.context(Some(version.to_string()))?;
        let pre_install = self
            .eval_async(chunk! {
                require "hooks/pre_install"
                return PLUGIN:PreInstall($ctx)
            })
            .await?;

        Ok(pre_install)
    }
}

#[derive(Debug)]
pub struct PreInstall {
    pub version: String,
    pub url: Option<String>,
    pub note: Option<String>,
    pub sha256: Option<String>,
    pub md5: Option<String>,
    pub sha1: Option<String>,
    pub sha512: Option<String>,
    // pub addition: Option<Table<'lua>>,
}

impl<'lua> FromLua<'lua> for PreInstall {
    fn from_lua(value: Value<'lua>, _: &'lua Lua) -> std::result::Result<Self, LuaError> {
        match value {
            Value::Table(table) => {
                if !table.contains_key("version")? {
                    return Err(LuaError::FromLuaConversionError {
                        from: "table",
                        to: "PreInstall",
                        message: Some("no version returned from vfox plugin".to_string()),
                    });
                }
                Ok(PreInstall {
                    version: table.get::<_, String>("version")?,
                    url: table.get::<_, Option<String>>("url")?,
                    note: table.get::<_, Option<String>>("note")?,
                    sha256: table.get::<_, Option<String>>("sha256")?,
                    md5: table.get::<_, Option<String>>("md5")?,
                    sha1: table.get::<_, Option<String>>("sha1")?,
                    sha512: table.get::<_, Option<String>>("sha512")?,
                    // addition,
                })
            }
            _ => panic!("Expected table"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::hooks::pre_install::PreInstall;
    use crate::runtime::Runtime;
    use crate::Plugin;
    use std::string::ToString;
    use tokio::test;

    #[test]
    async fn dummy() {
        let pre_install = run("dummy", "1.0.1").await;
        assert_debug_snapshot!(pre_install);
    }

    #[test]
    async fn nodejs() {
        Runtime::set_os("linux".to_string());
        Runtime::set_arch("x64".to_string());
        let pre_install = run("nodejs", "20.0.0").await;
        assert_debug_snapshot!(pre_install);

        Runtime::set_os("macos".to_string());
        Runtime::set_arch("arm64".to_string());
        let pre_install = run("nodejs", "20.1.0").await;
        assert_debug_snapshot!(pre_install);

        Runtime::set_os("windows".to_string());
        Runtime::set_arch("x64".to_string());
        let pre_install = run("nodejs", "20.3.0").await;
        assert_debug_snapshot!(pre_install);

        Runtime::reset();
    }

    async fn run(plugin: &str, v: &str) -> PreInstall {
        let p = Plugin::test(plugin);
        p.pre_install(v).await.unwrap()
    }
}
