use std::process::Command;  // Run programs
use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions



const SEGMENT_KSUID: &str = "0ujtsYcgvSTl8PAuAdqWYSMnLOv";
#[test]
fn deserialize_segment_compat_ksuid() {
    let mut cmd = Command::main_binary().unwrap();
    cmd.arg("inspect").arg(SEGMENT_KSUID);
    cmd.assert().success().stdout(predicate::str::contains("String: 0ujtsYcgvSTl8PAuAdqWYSMnLOv")).stdout(predicate::str::contains("Timestamp: 107608047"));
}

#[test]
fn create_new_ksuid() {
    let mut cmd = Command::main_binary().unwrap();
    cmd.arg("create");
    cmd.assert().success().stdout(predicate::str::contains("String: 0ujtsYcgvSTl8PAuAdqWYSMnLOv").not());
}
