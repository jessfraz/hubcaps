use std::env;

use tokio::runtime::Runtime;

use hubcaps::{Credentials, Github, Result};

fn main() -> Result<()> {
    pretty_env_logger::init();
    match env::var("GITHUB_TOKEN").ok() {
        Some(token) => {
            let mut rt = Runtime::new()?;
            let github = Github::new(
                concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")),
                Credentials::Token(token),
            )?;

            let repo = github.repo("softprops", "hubcaps");

            let forked = rt.block_on(repo.forks().create())?;

            println!("Forked repository to {}", forked.full_name);

            Ok(())
        }
        _ => Err("example missing GITHUB_TOKEN".into()),
    }
}
