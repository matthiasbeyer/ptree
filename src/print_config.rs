//!
//! Output formatting is configured through the [`PrintConfig`] structure.
//!

#[cfg(feature = "conf")]
use config;
#[cfg(feature = "conf")]
use directories::BaseDirs;

#[cfg(feature = "ansi")]
use atty::Stream;

use style::Style;

use std::env;
use std::fmt::{self, Display};
use std::marker::PhantomData;
use std::str::FromStr;

use serde::{
    de::{self, Deserializer, MapAccess, Unexpected, Visitor},
    Deserialize, Serialize,
};

///
/// Configuration option controlling when output styling is used
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StyleWhen {
    /// Never style output
    Never,
    /// Always style output
    Always,
    /// Style output only when printing to a TTY
    Tty,
}

///
/// Structure controlling the print output formatting
///
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct PrintConfig {
    /// Maximum recursion depth when printing
    ///
    /// The default is infinity, i.e. there is no recursion limit.
    pub depth: u32,
    /// Indentation size. The default value is 3.
    pub indent: usize,
    /// Padding size. The default value is 1.
    pub padding: usize,
    /// Control when output is styled.
    ///
    /// The default value is [`StyleWhen::Tty`], meaning that ANSI styles are only used for printing to the standard
    /// output, and only when the standard output is a TTY.
    pub styled: StyleWhen,
    /// Characters used to print indentation lines or "branches" of the tree
    #[serde(deserialize_with = "string_or_struct")]
    pub characters: IndentChars,
    /// ANSI style used for printing the indentation lines ("branches")
    pub branch: Style,
    /// ANSI style used for printing the item text ("leaves")
    pub leaf: Style,
}

impl Default for PrintConfig {
    fn default() -> PrintConfig {
        PrintConfig {
            depth: u32::max_value(),
            indent: 3,
            padding: 1,
            characters: UTF_CHARS.into(),
            branch: Style {
                dimmed: true,
                ..Style::default()
            },
            leaf: Style::default(),
            styled: StyleWhen::Tty,
        }
    }
}

///
/// Enumeration of output kinds
///
/// Standard output is treated differently because we can query
/// whether it is a TTY or not.
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputKind {
    /// The program's standard output
    Stdout,
    /// The actual output is not known
    Unknown,
}

impl PrintConfig {

    /// Try to instantiate PrintConfig from environment
    ///
    /// Only available with feature "config"
    #[cfg(feature = "conf")]
    fn try_from_env() -> Option<PrintConfig> {
        let mut settings = config::Config::default();

        if let Ok(p) = env::var("PTREE_CONFIG") {
            settings.merge(config::File::with_name(&p)).ok()?;
        } else {
            let f = BaseDirs::new()?.config_dir().join("ptree");
            settings.merge(config::File::with_name(f.to_str()?)).ok()?;
        }

        settings
            .merge(config::Environment::with_prefix("PTREE").separator("_"))
            .ok()?;

        Some(settings.try_into().ok()?)
    }

    ///
    /// Load print configuration from a configuration file or environment variables
    ///
    /// ### Configuration files and variables
    ///
    /// If the `PTREE_CONFIG` environment variable is set, its value is used as the path to a file
    /// from which to read to configuration parameters.
    /// Otherwise, any file with a stem of `ptree` inside the directory returned by [`config_dir`]
    /// is used.
    ///
    /// Finally, environment variables may be used to override the values from the configuration file.
    /// For every field of the `PrintConfig` structure, the corresponding environment variable name
    /// is PTREE_<FIELD_NAME>, for example `PTREE_INDENT=4` sets the `indent` field to 4.
    /// Nested fields are supported; to set the branch foreground color use `PTREE_BRANCH_FOREGROUND=red`.
    ///
    /// ### Field values
    ///
    /// [`indent`] and [`depth`] accept non-negative integers.
    ///
    /// [`styled`] accepts either `"always"`, `"tty"` or `"never"`
    ///
    /// [`leaf`] and [`branch`] accept a `Style` structure.
    /// In a configuration file, this takes a form of a map.
    /// Using environment variables, each field has to be set separately.
    ///
    /// Color fields accept either an ANSI named color, a named web color, a hex code like "#33ffbb",
    /// an ANSI integer fixed color, or a [red, green, blue] triple of non-negative integers.
    ///
    /// Other `Style` fields are boolean parameters.
    /// In a configuration file, they are parsed according to the rules of the deserialization format.
    /// In an environment variables, `TRUE`, `ON` and `1` evaluate to `true`, and `FALSE`, `OFF` and `0`
    /// evaluate to `false`. Environment variable values are case insensitive.
    ///
    /// [`characters`] can be set to a string with a value of "utf", "ascii", "ascii-plus", "utf-bold", "utf-double"
    /// or "utf-dashed". Alternatively, it can be set to a structure with each of their fields set to the
    /// appropriate character.
    ///
    /// ### Configuration file example
    ///
    /// ```toml
    /// indent = 3
    /// depth = 100
    /// styled = "tty"
    ///
    /// [branch]
    /// foreground = "red"
    /// dimmed = true
    /// bold = false
    ///
    /// [leaf]
    /// foreground = "MediumSeaGreen"
    /// background = "#102018"
    /// ```
    ///
    /// ### Errors
    ///
    /// This function does not report errors.
    /// If anything goes wrong while loading the configuration parameters, a default `PrintConfig` is returned.
    #[cfg(feature = "conf")]
    pub fn from_env() -> PrintConfig {
        Self::try_from_env().unwrap_or_else(Default::default)
    }
    #[cfg(not(feature = "conf"))]
    pub fn from_env() -> PrintConfig {
        Default::default()
    }

    ///
    /// Checks if output to a writer should be styled
    ///
    pub fn should_style_output(&self, output_kind: OutputKind) -> bool {
        if cfg!(feature = "ansi") {
            match (self.styled, output_kind) {
                (StyleWhen::Always, _) => true,
                #[cfg(feature = "ansi")]
                (StyleWhen::Tty, OutputKind::Stdout) => atty::is(Stream::Stdout),
                _ => false,
            }
        } else {
            false
        }
    }

    ///
    /// Formats `input` according to the branch style
    ///
    /// This function is a wrapper that is available even without the `"ansi"` feature.
    /// Without that feature it returns the input unchanged.
    ///
    pub fn paint_branch(&self, input: impl Display) -> impl Display {
        self.branch.paint(input)
    }

    ///
    /// Formats `input` according to the leaf style
    ///
    /// This function is a wrapper that is available even without the `"ansi"` feature.
    /// Without that feature it returns the input unchanged.
    ///
    pub fn paint_leaf(&self, input: impl Display) -> impl Display {
        self.leaf.paint(input)
    }
}

fn get_default_empty_string() -> String {
    " ".to_string()
}

///
/// Set of characters use to draw indentation lines (branches)
///
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct IndentChars {
    /// Character for pointing down and right (`├`).
    pub down_and_right: String,
    /// Character for pointing straight down (`|`).
    pub down: String,
    /// Character for turning from down to right (`└`).
    pub turn_right: String,
    /// Character for pointing right (`─`).
    pub right: String,
    /// Empty character (` `).
    #[serde(default = "get_default_empty_string")]
    pub empty: String,
}

impl From<StaticIndentChars> for IndentChars {
    fn from(s: StaticIndentChars) -> IndentChars {
        IndentChars {
            down_and_right: s.down_and_right.to_string(),
            down: s.down.to_string(),
            turn_right: s.turn_right.to_string(),
            right: s.right.to_string(),
            empty: s.empty.to_string(),
        }
    }
}

impl FromStr for IndentChars {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "utf" => Ok(UTF_CHARS.into()),
            "ascii" | "ascii-tick" => Ok(ASCII_CHARS_TICK.into()),
            "ascii-plus" => Ok(ASCII_CHARS_PLUS.into()),
            "utf-bold" => Ok(UTF_CHARS_BOLD.into()),
            "utf-dashed" => Ok(UTF_CHARS_DASHED.into()),
            "utf-double" => Ok(UTF_CHARS_DOUBLE.into()),
            _ => Err(()),
        }
    }
}

// Deserializes from either a struct or a string
//
// Taken from https://serde.rs/string-or-struct.html
fn string_or_struct<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: Deserialize<'de> + FromStr<Err = ()>,
    D: Deserializer<'de>,
{
    // This is a Visitor that forwards string types to T's `FromStr` impl and
    // forwards map types to T's `Deserialize` impl. The `PhantomData` is to
    // keep the compiler from complaining about T being an unused generic type
    // parameter. We need T in order to know the Value type for the Visitor
    // impl.
    struct StringOrStruct<T>(PhantomData<fn() -> T>);

    impl<'de, T> Visitor<'de> for StringOrStruct<T>
    where
        T: Deserialize<'de> + FromStr<Err = ()>,
    {
        type Value = T;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("string or map")
        }

        fn visit_str<E>(self, value: &str) -> Result<T, E>
        where
            E: de::Error,
        {
            FromStr::from_str(value).map_err(|_| {
                E::invalid_value(
                    Unexpected::Str(value),
                    &"'utf', 'ascii', 'ascii-plus', 'utf-double', 'utf-bold' or 'utf-dashed'",
                )
            })
        }

        fn visit_map<M>(self, visitor: M) -> Result<T, M::Error>
        where
            M: MapAccess<'de>,
        {
            // `MapAccessDeserializer` is a wrapper that turns a `MapAccess`
            // into a `Deserializer`, allowing it to be used as the input to T's
            // `Deserialize` implementation. T then deserializes itself using
            // the entries from the map visitor.
            Deserialize::deserialize(de::value::MapAccessDeserializer::new(visitor))
        }
    }

    deserializer.deserialize_any(StringOrStruct(PhantomData))
}

///
/// Set of characters use to draw indentation lines (branches)
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StaticIndentChars {
    /// Character for pointing down and right (`├`).
    pub down_and_right: &'static str,
    /// Character for pointing straight down (`|`).
    pub down: &'static str,
    /// Character for turning from down to right (`└`).
    pub turn_right: &'static str,
    /// Character for pointing right (`─`).
    pub right: &'static str,
    /// Empty character (` `).
    pub empty: &'static str,
}

///
/// ASCII indentation characters, using a tick (`\``) for turning right
///
/// This is the character used in the Linux command `tree --charset=ascii`.
///
pub const ASCII_CHARS_TICK: StaticIndentChars = StaticIndentChars {
    down_and_right: "|",
    down: "|",
    turn_right: "`",
    right: "-",
    empty: " ",
};

///
/// ASCII indentation characters, using a plus (`+`) for turning right
///
pub const ASCII_CHARS_PLUS: StaticIndentChars = StaticIndentChars {
    down_and_right: "+",
    down: "|",
    turn_right: "+",
    right: "-",
    empty: " ",
};

///
/// UTF-8 indentation characters, using regular box-drawing characters
///
/// This is the character used in the Linux command `tree`.
///
pub const UTF_CHARS: StaticIndentChars = StaticIndentChars {
    down_and_right: "├",
    down: "│",
    turn_right: "└",
    right: "─",
    empty: " ",
};

///
/// UTF-8 indentation characters, using double box-drawing characters
///
pub const UTF_CHARS_DOUBLE: StaticIndentChars = StaticIndentChars {
    down_and_right: "╠",
    down: "║",
    turn_right: "╚",
    right: "═",
    empty: " ",
};

///
/// UTF-8 indentation characters, using heavy box-drawing characters
///
pub const UTF_CHARS_BOLD: StaticIndentChars = StaticIndentChars {
    down_and_right: "┣",
    down: "┃",
    turn_right: "┗",
    right: "━",
    empty: " ",
};

///
/// UTF-8 indentation characters, using dashed box-drawing characters
///
pub const UTF_CHARS_DASHED: StaticIndentChars = StaticIndentChars {
    down_and_right: "├",
    down: "┆",
    turn_right: "└",
    right: "╌",
    empty: " ",
};

#[cfg(test)]
mod tests {
    use super::*;
    use style::Color;

    use std::env;
    use std::fs::{self, File};
    use std::io::Write;
    use std::sync::Mutex;

    lazy_static! {
        static ref ENV_MUTEX: Mutex<()> = Mutex::new(());
    }

    #[cfg(feature = "conf")]
    fn load_config_from_path(path: &str) -> PrintConfig {
        env::set_var("PTREE_CONFIG", path);
        let config = PrintConfig::from_env();
        env::remove_var("PTREE_CONFIG");

        config
    }

    #[test]
    #[cfg(feature = "conf")]
    fn load_yaml_config_file() {
        let _g = ENV_MUTEX.lock().unwrap();
        let path = "ptree.yaml";
        {
            let mut f = File::create(path).unwrap();
            writeln!(f, "indent: 7\nbranch:\n  foreground: maroon").unwrap();
        }

        let config = load_config_from_path(path);
        assert_eq!(config.indent, 7);
        assert_eq!(config.branch.foreground, Some(Color::Named("maroon".to_string())));
        assert_eq!(config.branch.background, None);

        fs::remove_file(path).unwrap();
    }

    #[test]
    #[cfg(feature = "conf")]
    fn load_toml_config_file() {
        let _g = ENV_MUTEX.lock().unwrap();
        let path = "ptree.toml";
        {
            let mut f = File::create(path).unwrap();
            writeln!(
                f,
                "indent = 5\n[leaf]\nforeground = \"green\"\nbackground = \"steelblue\"\n"
            )
            .unwrap();
        }

        let config = load_config_from_path(path);
        assert_eq!(config.indent, 5);
        assert_eq!(config.leaf.foreground, Some(Color::Named("green".to_string())));
        assert_eq!(config.leaf.background, Some(Color::Named("steelblue".to_string())));
        assert_eq!(config.branch.foreground, None);
        assert_eq!(config.branch.background, None);

        fs::remove_file(path).unwrap();
    }

    #[test]
    #[cfg(feature = "conf")]
    fn load_env() {
        let _g = ENV_MUTEX.lock().unwrap();
        let path = "ptree.toml";
        {
            let mut f = File::create(path).unwrap();
            writeln!(f, "indent = 5\n[leaf]\nforeground = \"green\"\n").unwrap();
        }

        env::set_var("PTREE_LEAF_BACKGROUND", "steelblue");
        env::set_var("PTREE_LEAF_BOLD", "true");
        env::set_var("PTREE_DEPTH", "4");

        let config = load_config_from_path(path);
        assert_eq!(config.indent, 5);
        assert_eq!(config.depth, 4);
        assert_eq!(config.leaf.foreground, Some(Color::Named("green".to_string())));
        assert_eq!(config.leaf.background, Some(Color::Named("steelblue".to_string())));
        assert_eq!(config.leaf.bold, true);
        assert_eq!(config.branch.foreground, None);
        assert_eq!(config.branch.background, None);

        env::remove_var("PTREE_LEAF_BACKGROUND");
        env::remove_var("PTREE_LEAF_BOLD");
        env::remove_var("PTREE_DEPTH");

        fs::remove_file(path).unwrap();
    }
}
