use assert_fs::prelude::*;
use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;
use semver::Version;
use std::path;
use cargo::{
    core::source::SourceId,
    util::config::{
        Config as CargoConfig
    },
    ops::read_package
};


#[test]
fn check_include_pre() -> Result<(), Box<dyn std::error::Error>> {
    let temp = assert_fs::TempDir::new()?;
    temp.copy_from("tests/fixtures/include-pre", &["*.toml", "*.rs"])?;

    let mut cmd = Command::cargo_bin("cargo-unleash")?;

    cmd
        .arg("--manifest-path")
        .arg(temp.path())
        .arg("check")
        .arg("--packages")
        .arg("crate_a")
        .arg("--include-pre-deps");
    cmd
        .assert()
        .success()
        .code(0);
    temp.close()?;
    Ok(())
}
