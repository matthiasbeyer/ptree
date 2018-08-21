use directories::BaseDirs;
use serde_any::{from_file, from_file_stem};

#[cfg(feature = "ansi")]
use isatty::stdout_isatty;

use style::Style;

use std::fmt::Display;
use std::env;

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
    pub max_depth: u32,
    /// Indentation size. The default value is 3.
    pub indent_size: usize,
    /// Control when output is styled
    pub style_when: StyleWhen,
    /// Characters used to print indentation lines or "branches" of the tree
    pub chars: IndentChars,
    /// ANSI style used for printing the indentation lines ("branches")
    pub branch_style: Style,
    /// ANSI style used for printing the item text ("leaves")
    pub leaf_style: Style,
}

impl Default for PrintConfig {
    fn default() -> PrintConfig {
        PrintConfig {
            max_depth: u32::max_value(),
            indent_size: 3,
            chars: UTF_CHARS.into(),
            branch_style: Style {
                dimmed: true,
                ..Style::default()
            },
            leaf_style: Style::default(),
            style_when: StyleWhen::Tty,
        }
    }
}

impl PrintConfig {
    ///
    /// Create a default `PrintConfig` for printing to standard output
    ///
    /// When printing to standard output, we check if the output is a TTY.
    /// If it is, and ANSI formatting is enabled, the branches will be dimmed by default.
    /// If the output is not a TTY, this is equivalent to `PrintConfig::default()`.
    ///
    pub fn for_stdout() -> PrintConfig {
        Default::default()
    }

    fn load_from_config_file() -> Option<PrintConfig> {
        if let Ok(p) = env::var("PTREE_CONFIG") {
            from_file(p).ok()
        } else {
            from_file_stem(BaseDirs::new()?.config_dir().join("ptree")).ok()
        }
    }

    ///
    /// Load print configuration from a configuration file
    ///
    pub fn load() -> PrintConfig {
        Self::load_from_config_file().unwrap_or_else(Default::default)
    }

    ///
    /// Checks if output to a writer should be styled
    ///
    pub fn should_style_output(&self, output_is_stdout: bool) -> bool {
        if cfg!(feature = "ansi") {
            match self.style_when {
                StyleWhen::Always => true,
                #[cfg(feature = "ansi")]
                StyleWhen::Tty => output_is_stdout && stdout_isatty(),
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
        self.branch_style.paint(input)
    }

    ///
    /// Formats `input` according to the leaf style
    ///
    /// This function is a wrapper that is available even without the `"ansi"` feature.
    /// Without that feature it returns the input unchanged.
    ///
    pub fn paint_leaf(&self, input: impl Display) -> impl Display {
        self.leaf_style.paint(input)
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
