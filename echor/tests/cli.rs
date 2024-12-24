use assert_cmd::Command;
use std::fs;

#[test]
fn runs() {
  let mut cmd = Command::cargo_bin("echor").unwrap();
  cmd.arg("hello").assert().success();
}

#[test]
fn dies_no_args() {
  let mut cmd = Command::cargo_bin("echor").unwrap();
  cmd.assert()
      .failure()
      .stderr(predicates::str::contains("USAGE"));
}

#[test]
fn hello1() {
  let outfile = "./tests/expected/hello1.txt";
  let expected = fs::read_to_string(outfile).unwrap();

  let mut cmd = Command::cargo_bin("echor").unwrap();
  cmd.arg("Hello there").assert().success().stdout(expected);
}
