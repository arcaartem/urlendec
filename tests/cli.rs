use assert_cmd::Command;
use assert_fs::prelude::*; // Add methods on commands
use predicates::prelude::*;

#[test]
fn input_file_does_not_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("urlendec")?;

    cmd.arg("-i").arg("test/file/doesnt/exist");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("could not read file"));

    Ok(())
}

#[test]
fn input_file_exists() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("sample_input.txt")?;
    file.write_str("Hello, world!\nTest123\n")?;

    let mut cmd = Command::cargo_bin("urlendec")?;

    cmd.arg("-i").arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicate::eq("Hello%2C%20world%21\nTest123\n"));

    Ok(())
}

#[test]
fn input_from_stdin() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("urlendec")?;

    cmd.write_stdin("Hello, world!\nTest123\n");
    cmd.assert()
        .success()
        .stdout(predicate::eq("Hello%2C%20world%21\nTest123\n"));

    Ok(())
}

#[test]
fn input_from_string() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("urlendec")?;

    cmd.arg("-s").arg("Hello, world!");
    cmd.assert()
        .success()
        .stdout(predicate::eq("Hello%2C%20world%21\n"));

    Ok(())
}

#[test]
fn output_to_file() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = assert_fs::TempDir::new()?;

    let mut cmd = Command::cargo_bin("urlendec").unwrap();

    cmd.arg("-s")
        .arg("Hello, world!")
        .arg("-o")
        .arg(format!("{}/test_output.txt", temp_dir.path().display()))
        .assert()
        .success();

    temp_dir
        .child("test_output.txt")
        .assert(predicate::path::exists())
        .assert("Hello%2C%20world%21\n");

    Ok(())
}

#[test]
fn decode_from_string() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("urlendec")?;

    cmd.arg("-s").arg("Hello%2C%20world%21");
    cmd.arg("-d");
    cmd.assert()
        .success()
        .stdout(predicate::eq("Hello, world!\n"));

    Ok(())
}
