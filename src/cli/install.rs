use std::env;
use clap::Args;
use crate::Vfox;

#[derive(Args)]
pub struct Install {
    pub name: String,
    pub version: String,
}

impl Install {
    pub async fn run(&self) {
        // env::temp_dir().push("vfox").create_dir_all().await.unwrap();
        println!("Installing {} version {}", self.name, self.version);
    }
}
