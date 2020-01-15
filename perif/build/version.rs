use std::fs;
use last_git_commit::{LastGitCommit, Id};

pub fn get_version() {

    let commit = LastGitCommit::new(Some(".."), Some("master")).unwrap().id.short();
    let cargo_version = env!("CARGO_PKG_VERSION");

    let version = format!("{}-{}", cargo_version, commit);

    fs::write("out/version.txt", version).unwrap();

}
