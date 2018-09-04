extern crate ptree;
extern crate tempfile;
#[macro_use]
extern crate lazy_static;

use std::env;
use std::io::Write;
use std::sync::Mutex;

lazy_static! {
    static ref ENV_MUTEX: Mutex<()> = Mutex::new(());
}

#[test]
fn test_chars_by_string_ascii() {
    let _g = ENV_MUTEX.lock().unwrap();

    let mut f = tempfile::Builder::new()
        .suffix(".toml")
        .tempfile().unwrap();
    writeln!(f, "chars = \"ascii\"").unwrap();

    env::set_var("PTREE_CONFIG", f.path());
    let config = ptree::PrintConfig::from_env();
    assert_eq!(config.chars, ptree::print_config::ASCII_CHARS_TICK.into());
}

#[test]
fn test_chars_by_string_utf() {
    let _g = ENV_MUTEX.lock().unwrap();

    let mut f = tempfile::Builder::new()
        .suffix(".toml")
        .tempfile().unwrap();
    writeln!(f, "chars = \"utf\"").unwrap();

    env::set_var("PTREE_CONFIG", f.path());
    let config = ptree::PrintConfig::from_env();
    assert_eq!(config.chars, ptree::print_config::UTF_CHARS.into());
}

#[test]
fn test_chars_by_string_double() {
    let _g = ENV_MUTEX.lock().unwrap();

    let mut f = tempfile::Builder::new()
        .suffix(".toml")
        .tempfile().unwrap();
    writeln!(f, "chars = \"utf-double\"").unwrap();

    env::set_var("PTREE_CONFIG", f.path());
    let config = ptree::PrintConfig::from_env();
    assert_eq!(config.chars, ptree::print_config::UTF_CHARS_DOUBLE.into());
}

#[test]
fn test_chars_by_struct() {
    let mut f = tempfile::Builder::new()
        .suffix(".toml")
        .tempfile().unwrap();
    writeln!(f, "\
                [chars]\n\
                down_and_right = \"|\"\n\
                down = \"|\"\n\
                turn_right = \"`\"\n\
                right = \"-\"\n\
                empty = \" \"\
                ").unwrap();

    env::set_var("PTREE_CONFIG", f.path());
    let config = ptree::PrintConfig::from_env();
    assert_eq!(config.chars.down_and_right, "|");
    assert_eq!(config.chars.down, "|");
    assert_eq!(config.chars.turn_right, "`");
    assert_eq!(config.chars.right, "-");
    assert_eq!(config.chars.empty, " ");
}
