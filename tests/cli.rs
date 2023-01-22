use assert_cmd::Command;
use assert_fs::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions

#[test]
fn input_file_does_not_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("urlencode")?;

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

    let mut cmd = Command::cargo_bin("urlencode")?;

    cmd.arg("-i").arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Hello%2C%20world%21\nTest123\n"));

    Ok(())
}

#[test]
fn input_from_stdin() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("urlencode")?;

    cmd.write_stdin("Hello, world!\nTest123\n");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Hello%2C%20world%21\nTest123\n"));

    Ok(())
}

#[test]
fn input_from_string() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("urlencode")?;

    cmd.arg("-s").arg("Hello, world!");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Hello%2C%20world%21"));

    Ok(())
}

#[test]
fn output_to_file() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("sample_output.txt")?;

    let mut cmd = Command::cargo_bin("urlencode")?;

    cmd.arg("-s").arg("Hello, world!");
    cmd.arg("-o").arg(file.path());
    cmd.assert().success();

    file.assert("Hello%2C%20world%21");

    Ok(())
}

#[test]
fn decode_from_string() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("urlencode")?;

    cmd.arg("-s").arg("Hello%2C%20world%21");
    cmd.arg("-d");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Hello, world!"));

    Ok(())
}
