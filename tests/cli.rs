//! End-to-end tests of the `kvlite` binary: commands in on stdin, replies out
//! on stdout, the way a person or a script drives it.

use std::io::Write;
use std::process::{Command, Output, Stdio};

/// The reply for a key that is not in the store.
const NIL: &str = "(nil)";

/// Runs `kvlite` with `args`, writes `input` to its stdin, closes stdin and
/// waits for the process to exit.
fn kvlite(args: &[&str], input: &[u8]) -> Output {
    let mut child = Command::new(env!("CARGO_BIN_EXE_kvlite"))
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("kvlite starts");
    let mut stdin = child.stdin.take().expect("stdin is piped");
    stdin.write_all(input).expect("write stdin");
    drop(stdin); // EOF: the shell finishes and exits
    child.wait_with_output().expect("kvlite exits")
}

/// The reply lines kvlite printed on stdout.
fn replies(output: &Output) -> Vec<String> {
    String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(str::to_owned)
        .collect()
}

#[test]
fn put_get_del_get_round_trip() {
    let out = kvlite(&[], b"put a 1\nget a\ndel a\nget a\n");
    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert_eq!(stdout, "OK\n1\nOK\n(nil)\n");
    assert!(out.stderr.is_empty(), "no prompt when stdin is a pipe");
}

#[test]
fn malformed_command_prints_err_and_the_shell_carries_on() {
    let out = kvlite(&[], b"put a\nget a\n");
    assert!(out.status.success());
    let lines = replies(&out);
    assert_eq!(lines.len(), 2, "{lines:?}");
    assert!(lines[0].starts_with("ERR "), "{lines:?}");
    assert_eq!(lines[1], NIL);
}

#[test]
fn unknown_command_and_missing_keys() {
    let out = kvlite(&[], b"frobnicate x\nget nope\ndel nope\n");
    assert!(out.status.success());
    let lines = replies(&out);
    assert_eq!(lines.len(), 3, "{lines:?}");
    assert!(lines[0].starts_with("ERR "), "{lines:?}");
    assert_eq!(lines[1..], [NIL, NIL]);
}

#[test]
fn value_is_the_rest_of_the_line_and_blank_lines_are_skipped() {
    let out = kvlite(&[], b"put k hello,  big world\n\n   \nget k\n");
    assert!(out.status.success());
    assert_eq!(replies(&out), ["OK", "hello,  big world"]);
}

#[test]
fn invalid_utf8_line_prints_err_and_the_shell_carries_on() {
    let out = kvlite(&[], b"put a 1\nget \xff\nget a\n");
    assert!(out.status.success());
    let lines = replies(&out);
    assert_eq!(lines.len(), 3, "{lines:?}");
    assert_eq!(lines[0], "OK");
    assert!(lines[1].starts_with("ERR "), "{lines:?}");
    assert_eq!(lines[2], "1");
}

#[test]
fn help_command_prints_the_usage() {
    let out = kvlite(&[], b"help\n");
    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("put <key> <value>"), "{stdout}");
}

#[test]
fn help_flag_prints_usage_on_stdout_and_exits_0() {
    for flag in ["-h", "--help"] {
        let out = kvlite(&[flag], b"");
        assert_eq!(out.status.code(), Some(0), "{flag}");
        let stdout = String::from_utf8_lossy(&out.stdout);
        assert!(stdout.contains("Usage: kvlite"), "{flag}: {stdout}");
        assert!(out.stderr.is_empty(), "{flag}");
    }
}

#[test]
fn unknown_flag_exits_2_with_usage_on_stderr() {
    let out = kvlite(&["--bogus"], b"");
    assert_eq!(out.status.code(), Some(2));
    assert!(out.stdout.is_empty());
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(stderr.contains("--bogus"), "{stderr}");
    assert!(stderr.contains("Usage: kvlite"), "{stderr}");
}
