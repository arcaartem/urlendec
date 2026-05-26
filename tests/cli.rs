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

    let mut cmd = Command::cargo_bin("urlendec")?;

    cmd.arg("-s")
        .arg("Hello, world!")
        .arg("-o")
        .arg(format!("{}/test_output.txt", temp_dir.path().display()))
        .assert()
        .success();

    temp_dir
        .child("test_output.txt")
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

#[test]
fn decode_invalid_percent_encoding_errors() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("urlendec")?;

    cmd.arg("-d").arg("-s").arg("%ZZ");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("invalid percent-encoding"));

    Ok(())
}

#[test]
fn unicode_encode() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("urlendec")?;

    cmd.arg("-s").arg("café");
    cmd.assert().success().stdout(predicate::eq("caf%C3%A9\n"));

    Ok(())
}

#[test]
fn empty_file_produces_empty_stdout() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("empty.txt")?;
    file.write_str("")?;

    let mut cmd = Command::cargo_bin("urlendec")?;

    cmd.arg("-i").arg(file.path());
    cmd.assert().success().stdout(predicate::eq(""));

    Ok(())
}

#[test]
fn decode_to_file() -> Result<(), Box<dyn std::error::Error>> {
    let out_file = assert_fs::NamedTempFile::new("decoded.txt")?;

    let mut cmd = Command::cargo_bin("urlendec")?;

    cmd.arg("-d")
        .arg("-s")
        .arg("Hello%2C%20world%21")
        .arg("-o")
        .arg(out_file.path())
        .assert()
        .success();

    out_file.assert("Hello, world!\n");

    Ok(())
}

#[test]
fn clap_rejects_string_and_file_together() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("urlendec")?;

    cmd.arg("-s").arg("foo").arg("-i").arg("bar.txt");
    cmd.assert().failure();

    Ok(())
}

#[test]
fn midstream_decode_error() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("mixed.txt")?;
    file.write_str("good\nbad%ZZ\n")?;

    let mut cmd = Command::cargo_bin("urlendec")?;

    cmd.arg("-d").arg("-i").arg(file.path());
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("invalid percent-encoding"));

    Ok(())
}
