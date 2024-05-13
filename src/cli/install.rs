use vfox::{Vfox, VfoxResult};

#[derive(clap::Args)]
pub struct Install {
    pub sdk: String,
    pub version: String,
}

impl Install {
    pub async fn run(&self) -> VfoxResult<()> {
        let vfox = Vfox::new();
        let sdk = vfox.get_sdk(&self.sdk)?;
        info!("Installing {sdk} version {}", self.version);
        sdk.install(&self.version).await?;
        Ok(())
    }
}
