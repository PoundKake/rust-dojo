use assert_cmd::prelude::*; // Add methods on command
use std::process::Command; // run programs

#[test]
fn file_does_not_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("grrs")?;

    cmd.arg("foobar").arg("test/file/doesnt/exist");
    cmd.assert()
        .failure()
        .stderr(predicates::str::contains("could not read file"));

   Ok(()) 
}
