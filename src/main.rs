//! The `kvlite` binary: an interactive put/get/del shell over [`MemStore`].
//!
//! It reads one command per line from stdin and writes one reply per command
//! to stdout. Nothing is persisted yet, so every run starts with an empty store.

use std::env;
use std::io::{self, BufRead, IsTerminal, Write};
use std::process::ExitCode;

use kvlite::store::MemStore;

/// Printed by `--help` and by the `help` command. The version comes from
/// Cargo.toml, so a release-please bump never drifts from this text.
const USAGE: &str = concat!(
    "kvlite ",
    env!("CARGO_PKG_VERSION"),
    ": an in-memory key-value store (nothing is persisted yet)\n",
    "\n",
    "Usage: kvlite [-h | --help]\n",
    "\n",
    "Reads one command per line from stdin and prints one reply per command:\n",
    "  put <key> <value>   set key to value (the rest of the line) -> OK\n",
    "  get <key>           the value, or (nil) if the key is absent\n",
    "  del <key>           remove the key -> OK, or (nil) if it was absent\n",
    "  help                this help\n",
    "Blank lines are skipped; a bad command prints ERR <reason>. EOF (Ctrl-D) exits.\n",
);

const OK: &str = "OK";
const NIL: &str = "(nil)";

fn main() -> ExitCode {
    let mut wants_help = false;
    for arg in env::args_os().skip(1) {
        if arg == "-h" || arg == "--help" {
            wants_help = true;
        } else {
            eprintln!("kvlite: unexpected argument '{}'", arg.to_string_lossy());
            eprint!("{USAGE}");
            return ExitCode::from(2);
        }
    }
    if wants_help {
        print!("{USAGE}");
        return ExitCode::SUCCESS;
    }
    match shell() {
        Ok(()) => ExitCode::SUCCESS,
        // The reader went away (`kvlite < cmds | head -1`): nobody is left to tell.
        Err(e) if e.kind() == io::ErrorKind::BrokenPipe => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("kvlite: {e}");
            ExitCode::FAILURE
        }
    }
}

/// Reads commands from stdin until EOF and prints one reply line per command.
fn shell() -> io::Result<()> {
    let stdin = io::stdin();
    let interactive = stdin.is_terminal();
    let mut out = io::stdout().lock();
    let mut db = MemStore::default();
    prompt(interactive);
    for line in stdin.lock().lines() {
        let reply = match line {
            Ok(line) => execute(&mut db, &line),
            // `lines()` has already consumed the bad line, so reading carries on after it.
            Err(e) if e.kind() == io::ErrorKind::InvalidData => {
                Some("ERR the line is not valid UTF-8".to_owned())
            }
            Err(e) => return Err(e),
        };
        if let Some(reply) = reply {
            writeln!(out, "{reply}")?;
        }
        prompt(interactive);
    }
    if interactive {
        eprintln!(); // end the prompt line that Ctrl-D left open
    }
    Ok(())
}

/// Shows the prompt on stderr, and only to a person at a terminal, so that
/// stdout carries nothing but replies.
fn prompt(interactive: bool) {
    if interactive {
        eprint!("kvlite> ");
    }
}

/// Runs one input line against `db` and returns the reply to print, or `None`
/// for a blank line. A malformed or unknown command gets `ERR <reason>`.
fn execute(db: &mut MemStore, line: &str) -> Option<String> {
    let (command, args) = split_word(line);
    let (key, value) = split_word(args);
    // `get` and `del` take exactly one key; `put` takes a key and a value.
    let one_key = !key.is_empty() && value.is_empty();
    let reply = match command {
        "" => return None,
        "put" if !value.is_empty() => {
            db.put(key.to_owned(), value.to_owned());
            OK.to_owned()
        }
        "get" if one_key => db.get(key).unwrap_or_else(|| NIL.to_owned()),
        "del" if one_key => {
            if db.delete(key) {
                OK.to_owned()
            } else {
                NIL.to_owned()
            }
        }
        "help" if args.is_empty() => USAGE.trim_end().to_owned(),
        "put" => "ERR usage: put <key> <value>".to_owned(),
        "get" => "ERR usage: get <key>".to_owned(),
        "del" => "ERR usage: del <key>".to_owned(),
        "help" => "ERR usage: help".to_owned(),
        _ => format!("ERR unknown command '{command}' (try help)"),
    };
    Some(reply)
}

/// Splits off the first whitespace-separated word of `s`. Both parts come back
/// trimmed, so the rest of the line keeps only its inner spaces.
fn split_word(s: &str) -> (&str, &str) {
    let s = s.trim();
    match s.split_once(char::is_whitespace) {
        Some((word, rest)) => (word, rest.trim_start()),
        None => (s, ""),
    }
}
