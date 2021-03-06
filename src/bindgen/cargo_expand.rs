use std::env;
use std::path::Path;
use std::process::Command;
use std::str::from_utf8;

type ExpandResult = Result<String, String>;

pub fn expand(manifest_path: &Path, crate_name: &str) -> ExpandResult {
    let cargo = env::var("CARGO").unwrap_or_else(|_| String::from("cargo"));
    let mut cmd = Command::new(cargo);
    cmd.arg("rustc");
    cmd.arg("--manifest-path");
    cmd.arg(manifest_path);
    cmd.arg("--all-features");
    cmd.arg("-p");
    cmd.arg(crate_name);
    cmd.arg("--");
    cmd.arg("-Z");
    cmd.arg("unstable-options");
    cmd.arg("--pretty=expanded");
    let output = cmd.output().unwrap();

    let src = from_utf8(&output.stdout).unwrap().to_owned();
    let error = from_utf8(&output.stderr).unwrap().to_owned();

    if src.len() == 0 {
        Err(error)
    } else {
        Ok(src)
    }
}
