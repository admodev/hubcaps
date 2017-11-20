extern crate env_logger;
extern crate hubcaps;
extern crate tokio_core;

use hubcaps::{Credentials, Github};
use hubcaps::hooks::{HookCreateOptions, WebHookContentType};
use std::env;
use tokio_core::reactor::Core;

fn main() {
    env_logger::init().unwrap();
    match env::var("GITHUB_TOKEN").ok() {
        Some(token) => {
            let mut core = Core::new().unwrap();
            let github = Github::new(
                format!("hubcaps/{}", env!("CARGO_PKG_VERSION")),
                Credentials::Token(token),
                &core.handle(),
            );
            let repo = github.repo("softprops", "hubcaps");
            let hook = core.run(
                repo.hooks().create(&HookCreateOptions::web()
                    .url("http://localhost:8080")
                    .content_type(WebHookContentType::Json)
                    .build()),
            );
            println!("{:#?}", hook);
            let hooks = repo.hooks();
            for hook in core.run(hooks.list()).unwrap() {
                println!("{:#?}", hook)
            }
        }
        _ => println!("example missing GITHUB_TOKEN"),
    }
}
