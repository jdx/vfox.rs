use crate::config::Config;
use mlua::{UserData, UserDataFields};

use crate::error::Result;
use crate::Plugin;

impl Plugin {
    #[tokio::main(flavor = "current_thread")]
    #[allow(clippy::needless_return)]
    pub async fn post_install(&self, ctx: PostInstallContext) -> Result<()> {
        self.post_install_async(ctx).await
    }

    pub async fn post_install_async(&self, ctx: PostInstallContext) -> Result<()> {
        debug!("[vfox:{}] post_install", &self.name);
        self.exec_async(chunk! {
            require "hooks/post_install"
            PLUGIN:PostInstall($ctx)
        })
        .await
    }
}

pub struct PostInstallContext {
    pub args: Vec<String>,
    pub root_path: String,
}

impl UserData for PostInstallContext {
    fn add_fields<F: UserDataFields<Self>>(fields: &mut F) {
        let config = Config::get();
        fields.add_field_method_get("rootPath", |_, t| Ok(t.root_path.clone()));
        fields.add_field_method_get("runtimeVersion", move |_, _| {
            Ok(config.runtime_version.clone())
        });
    }
}

#[cfg(test)]
mod tests {
    use crate::Plugin;

    use super::*;

    #[test]
    fn dummy() {
        let p = Plugin::test("dummy");
        let ctx = PostInstallContext {
            args: vec![],
            root_path: "/tmp".to_string(),
        };
        p.post_install(ctx).unwrap();
    }
}
