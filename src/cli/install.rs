use clap::Args;

#[derive(Args)]
pub struct Install {
    pub name: String,
    pub version: String,
}

impl Install {
    pub async fn run(&self) {
        println!("Installing {} version {}", self.name, self.version);
    }
}
