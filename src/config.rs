#[cfg(feature = "ansi")]
use ansi_term::Style;
#[cfg(feature = "ansi")]
use isatty::stdout_isatty;

use std::fmt::Display;

#[derive(Clone, Debug, PartialEq)]
pub struct PrintConfig<'a> {
    pub max_depth: u32,
    pub indent_size: usize,
    pub chars: IndentChars<'a>,
    #[cfg(feature = "ansi")]
    pub branch_style: Style,
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
    pub fn create_indent_chars(&self) -> Indent {
        Indent::from_chars(self.indent_size, &self.chars)
    }

    pub fn paint_branch(&self, input: impl Display) -> impl Display {
        #[cfg(feature = "ansi")]
        return self.branch_style.paint(input.to_string());

        #[cfg(not(feature = "ansi"))]
        return input;
    }
    pub fn paint_leaf(&self, input: impl Display) -> impl Display {
        #[cfg(feature = "ansi")]
        return self.leaf_style.paint(input.to_string());

        #[cfg(not(feature = "ansi"))]
        return input;
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IndentChars<'a> {
    pub down_and_right: &'a str,
    pub down: &'a str,
    pub turn_right: &'a str,
    pub right: &'a str,
    pub empty: &'a str,
}

pub const ASCII_CHARS_PLUS: IndentChars<'static> = IndentChars {
    down_and_right: "+",
    down: "|",
    turn_right: "+",
    right: "-",
    empty: " ",
};

pub const ASCII_CHARS_TICK: IndentChars<'static> = IndentChars {
    down_and_right: "+",
    down: "|",
    turn_right: "`",
    right: "-",
    empty: " ",
};

pub const UTF_CHARS: IndentChars<'static> = IndentChars {
    down_and_right: "├",
    down: "│",
    turn_right: "└",
    right: "─",
    empty: " ",
};

pub const UTF_CHARS_DOUBLE: IndentChars<'static> = IndentChars {
    down_and_right: "╠",
    down: "║",
    turn_right: "╚",
    right: "═",
    empty: " ",
};

pub const UTF_CHARS_BOLD: IndentChars<'static> = IndentChars {
    down_and_right: "┣",
    down: "┃",
    turn_right: "┗",
    right: "━",
    empty: " ",
};

pub const UTF_CHARS_DASHED: IndentChars<'static> = IndentChars {
    down_and_right: "├",
    down: "┆",
    turn_right: "└",
    right: "╌",
    empty: " ",
};

pub struct Indent {
    pub regular_prefix: String,
    pub child_prefix: String,
    pub last_regular_prefix: String,
    pub last_child_prefix: String,
}

impl Indent {
    pub fn from_chars(indent_size: usize, chars: &IndentChars) -> Indent {
        let n = if indent_size > 2 { indent_size - 2 } else { 0 };

        let right_pad = chars.right.repeat(n);
        let empty_pad = chars.empty.repeat(n);

        Indent {
            regular_prefix: format!("{}{} ", chars.down_and_right, right_pad),
            child_prefix: format!("{}{} ", chars.down, empty_pad),
            last_regular_prefix: format!("{}{} ", chars.turn_right, right_pad),
            last_child_prefix: format!("{}{} ", chars.empty, empty_pad),
        }
    }
}
