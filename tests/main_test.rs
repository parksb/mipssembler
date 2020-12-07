use assert_cmd::prelude::*;
use std::io::{Read, Write};
use std::process::Command;
use tempfile::NamedTempFile;

mod fixtures;

const BIN_NAME: &str = "mipssembler";

#[test]
fn test_main_case_1() {
    use fixtures::{INPUT_CASE_1, OUTPUT_CASE_1};

    let mut input_file = NamedTempFile::new().unwrap();
    input_file.write_all(INPUT_CASE_1.as_bytes()).unwrap();

    let mut output_file = NamedTempFile::new().unwrap();

    Command::cargo_bin(BIN_NAME)
        .unwrap()
        .args(&[input_file.path(), output_file.path()])
        .assert()
        .success();

    let mut actual = String::new();
    output_file.read_to_string(&mut actual).unwrap();

    assert_eq!(actual, OUTPUT_CASE_1);
}

#[test]
fn test_main_case_2() {
    use fixtures::{INPUT_CASE_2, OUTPUT_CASE_2};

    let mut input_file = NamedTempFile::new().unwrap();
    input_file.write_all(INPUT_CASE_2.as_bytes()).unwrap();

    let mut output_file = NamedTempFile::new().unwrap();

    Command::cargo_bin(BIN_NAME)
        .unwrap()
        .args(&[input_file.path(), output_file.path()])
        .assert()
        .success();

    let mut actual = String::new();
    output_file.read_to_string(&mut actual).unwrap();

    assert_eq!(actual, OUTPUT_CASE_2);
}

#[test]
fn test_main_case_3() {
    use fixtures::{INPUT_CASE_3, OUTPUT_CASE_3};

    let mut input_file = NamedTempFile::new().unwrap();
    input_file.write_all(INPUT_CASE_3.as_bytes()).unwrap();

    let mut output_file = NamedTempFile::new().unwrap();

    Command::cargo_bin(BIN_NAME)
        .unwrap()
        .args(&[input_file.path(), output_file.path()])
        .assert()
        .success();

    let mut actual = String::new();
    output_file.read_to_string(&mut actual).unwrap();

    assert_eq!(actual, OUTPUT_CASE_3);
}

#[test]
fn test_main_case_4() {
    use fixtures::{INPUT_CASE_4, OUTPUT_CASE_4};

    let mut input_file = NamedTempFile::new().unwrap();
    input_file.write_all(INPUT_CASE_4.as_bytes()).unwrap();

    let mut output_file = NamedTempFile::new().unwrap();

    Command::cargo_bin(BIN_NAME)
        .unwrap()
        .args(&[input_file.path(), output_file.path()])
        .assert()
        .success();

    let mut actual = String::new();
    output_file.read_to_string(&mut actual).unwrap();

    assert_eq!(actual, OUTPUT_CASE_4);
}
