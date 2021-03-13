#[macro_use]
extern crate lazy_static;
extern crate ptree;
extern crate tempfile;

use std::env;
use std::io::Write;
use std::sync::Mutex;

lazy_static! {
    static ref ENV_MUTEX: Mutex<()> = Mutex::new(());
}

#[test]
#[cfg(feature = "conf")]
fn test_characters_by_string_ascii() {
    let _g = ENV_MUTEX.lock().unwrap();

    let mut f = tempfile::Builder::new().suffix(".toml").tempfile().unwrap();
    writeln!(f, "characters = \"ascii\"").unwrap();

    env::set_var("PTREE_CONFIG", f.path());
    let config = ptree::PrintConfig::from_env();
    assert_eq!(config.characters, ptree::print_config::ASCII_CHARS_TICK.into());
}

#[test]
#[cfg(feature = "conf")]
fn test_characters_by_string_utf() {
    let _g = ENV_MUTEX.lock().unwrap();

    let mut f = tempfile::Builder::new().suffix(".toml").tempfile().unwrap();
    writeln!(f, "characters = \"utf\"").unwrap();

    env::set_var("PTREE_CONFIG", f.path());
    let config = ptree::PrintConfig::from_env();
    assert_eq!(config.characters, ptree::print_config::UTF_CHARS.into());
}

#[test]
#[cfg(feature = "conf")]
fn test_characters_by_string_double() {
    let _g = ENV_MUTEX.lock().unwrap();

    let mut f = tempfile::Builder::new().suffix(".toml").tempfile().unwrap();
    writeln!(f, "characters = \"utf-double\"").unwrap();

    env::set_var("PTREE_CONFIG", f.path());
    let config = ptree::PrintConfig::from_env();
    assert_eq!(config.characters, ptree::print_config::UTF_CHARS_DOUBLE.into());
}

#[test]
#[cfg(feature = "conf")]
fn test_characters_by_struct() {
    let _g = ENV_MUTEX.lock().unwrap();

    let mut f = tempfile::Builder::new().suffix(".toml").tempfile().unwrap();
    writeln!(
        f,
        "\
         [characters]\n\
         down_and_right = \"|\"\n\
         down = \"|\"\n\
         turn_right = \"`\"\n\
         right = \"-\"\n\
         empty = \" \"\
         "
    )
    .unwrap();

    env::set_var("PTREE_CONFIG", f.path());
    let config = ptree::PrintConfig::from_env();
    assert_eq!(config.characters.down_and_right, "|");
    assert_eq!(config.characters.down, "|");
    assert_eq!(config.characters.turn_right, "`");
    assert_eq!(config.characters.right, "-");
    assert_eq!(config.characters.empty, " ");
}
