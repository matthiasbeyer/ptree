#[cfg(feature = "ansi")]
use ansi_term::Style;
#[cfg(feature = "ansi")]
use isatty::stdout_isatty;

use std::fmt::Display;

///
/// Structure controlling the print output formatting
///
#[derive(Clone, Debug, PartialEq)]
pub struct PrintConfig<'a> {
    /// Maximum recursion depth when printing
    ///
    /// The default is infinity, i.e. there is no recursion limit.
    pub max_depth: u32,
    /// Indentation size. The default value is 3.
    pub indent_size: usize,
    /// Characters used to print indentation lines or "branches" of the tree
    pub chars: IndentChars<'a>,
    /// ANSI style used for printing the indentation lines ("branches")
    #[cfg(feature = "ansi")]
    pub branch_style: Style,
    /// ANSI style used for printing the item text ("leaves")
    #[cfg(feature = "ansi")]
    pub leaf_style: Style,
}

impl<'a> Default for PrintConfig<'a> {
    fn default() -> PrintConfig<'a> {
        PrintConfig {
            max_depth: u32::max_value(),
            indent_size: 3,
            chars: UTF_CHARS,
            #[cfg(feature = "ansi")]
            branch_style: Style::new(),
            #[cfg(feature = "ansi")]
            leaf_style: Style::new(),
        }
    }
}

impl<'a> PrintConfig<'a> {
    ///
    /// Create a default `PrintConfig` for printing to standard output
    ///
    /// When printing to standard output, we check if the output is a TTY.
    /// If it is, and ANSI formatting is enabled, the branches will be dimmed by default.
    /// If the output is not a TTY, this is equivalent to `PrintConfig::default()`.
    ///
    pub fn for_stdout() -> PrintConfig<'a> {
        PrintConfig {
            max_depth: u32::max_value(),
            indent_size: 3,
            chars: UTF_CHARS,
            #[cfg(feature = "ansi")]
            branch_style: if stdout_isatty() {
                Style::new().dimmed()
            } else {
                Style::new()
            },
            #[cfg(feature = "ansi")]
            leaf_style: Style::new(),
        }
    }
}

impl<'a> PrintConfig<'a> {
    ///
    /// Formats `input` according to the branch style
    ///
    /// This function is a wrapper that is available even without the `"ansi"` feature.
    /// Without that feature it returns the input unchanged.
    ///
    pub fn paint_branch(&self, input: impl Display) -> impl Display {
        #[cfg(feature = "ansi")]
        return self.branch_style.paint(input.to_string());

        #[cfg(not(feature = "ansi"))]
        return input;
    }

    ///
    /// Formats `input` according to the leaf style
    ///
    /// This function is a wrapper that is available even without the `"ansi"` feature.
    /// Without that feature it returns the input unchanged.
    ///
    pub fn paint_leaf(&self, input: impl Display) -> impl Display {
        #[cfg(feature = "ansi")]
        return self.leaf_style.paint(input.to_string());

        #[cfg(not(feature = "ansi"))]
        return input;
    }
}

///
/// Set of characters use to draw indentation lines (branches)
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IndentChars<'a> {
    /// Character for pointing down and right (`├`).
    pub down_and_right: &'a str,
    /// Character for pointing straight down (`|`).
    pub down: &'a str,
    /// Character for turning from down to right (`└`).
    pub turn_right: &'a str,
    /// Character for pointing right (`─`).
    pub right: &'a str,
    /// Empty character (` `).
    pub empty: &'a str,
}

///
/// ASCII indentation characters, using a tick (`\``) for turning right
///
/// This is the character used in the Linux command `tree --charset=ascii`.
///
pub const ASCII_CHARS_TICK: IndentChars<'static> = IndentChars {
    down_and_right: "|",
    down: "|",
    turn_right: "`",
    right: "-",
    empty: " ",
};

///
/// ASCII indentation characters, using a plus (`+`) for turning right
///
pub const ASCII_CHARS_PLUS: IndentChars<'static> = IndentChars {
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
pub const UTF_CHARS: IndentChars<'static> = IndentChars {
    down_and_right: "├",
    down: "│",
    turn_right: "└",
    right: "─",
    empty: " ",
};

///
/// UTF-8 indentation characters, using double box-drawing characters
///
pub const UTF_CHARS_DOUBLE: IndentChars<'static> = IndentChars {
    down_and_right: "╠",
    down: "║",
    turn_right: "╚",
    right: "═",
    empty: " ",
};

///
/// UTF-8 indentation characters, using heavy box-drawing characters
///
pub const UTF_CHARS_BOLD: IndentChars<'static> = IndentChars {
    down_and_right: "┣",
    down: "┃",
    turn_right: "┗",
    right: "━",
    empty: " ",
};

///
/// UTF-8 indentation characters, using dashed box-drawing characters
///
pub const UTF_CHARS_DASHED: IndentChars<'static> = IndentChars {
    down_and_right: "├",
    down: "┆",
    turn_right: "└",
    right: "╌",
    empty: " ",
};
