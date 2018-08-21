use item::*;
use config::*;
use style::*;

use std::io;

struct Indent {
    pub regular_prefix: String,
    pub child_prefix: String,
    pub last_regular_prefix: String,
    pub last_child_prefix: String,
}

impl Indent {
    pub fn from_config(config: &PrintConfig) -> Indent {
        Self::from_chars(config.indent_size, &config.chars)
    }

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

fn print_item<T: TreeItem, W: io::Write>(
    item: &T,
    f: &mut W,
    prefix: String,
    child_prefix: String,
    config: &PrintConfig,
    chars: &Indent,
    leaf_style: &Style,
    level: u32,
) -> io::Result<()> {
    write!(f, "{}", config.paint_branch(prefix))?;
    item.write_self(f, leaf_style)?;
    writeln!(f, "")?;

    if level < config.max_depth {
        let children = item.children();
        if let Some((last_child, children)) = children.split_last() {
            let rp = child_prefix.clone() + &chars.regular_prefix;
            let cp = child_prefix.clone() + &chars.child_prefix;

            for c in children {
                print_item(
                    c,
                    f,
                    rp.clone(),
                    cp.clone(),
                    config,
                    chars,
                    leaf_style,
                    level + 1,
                )?;
            }

            let rp = child_prefix.clone() + &chars.last_regular_prefix;
            let cp = child_prefix.clone() + &chars.last_child_prefix;

            print_item(last_child, f, rp, cp, config, chars, leaf_style, level + 1)?;
        }
    }

    Ok(())
}

/// Print the tree `item` to standard output using default formatting
pub fn print_tree<T: TreeItem>(item: &T) -> io::Result<()> {
    print_tree_with(item, &PrintConfig::load())
}

/// Print the tree `item` to standard output using custom formatting
pub fn print_tree_with<T: TreeItem>(item: &T, config: &PrintConfig) -> io::Result<()> {
    let style = if config.should_style_output(true) {
        config.leaf_style.clone()
    } else {
        Style::default()
    };

    let chars = Indent::from_config(config);
    let out = io::stdout();
    let mut handle = out.lock();
    print_item(
        item,
        &mut handle,
        "".to_string(),
        "".to_string(),
        config,
        &chars,
        &style,
        0,
    )
}

/// Write the tree `item` to writer `f` using default formatting
pub fn write_tree<T: TreeItem, W: io::Write>(item: &T, mut f: W) -> io::Result<()> {
    write_tree_with(item, &mut f, &PrintConfig::load())
}

/// Write the tree `item` to writer `f` using custom formatting
pub fn write_tree_with<T: TreeItem, W: io::Write>(item: &T, mut f: W, config: &PrintConfig) -> io::Result<()> {
    let style = if config.should_style_output(true) {
        config.leaf_style.clone()
    } else {
        Style::default()
    };

    let chars = Indent::from_config(config);
    print_item(
        item,
        &mut f,
        "".to_string(),
        "".to_string(),
        config,
        &chars,
        &style,
        0,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use config::PrintConfig;

    #[test]
    fn indent_from_chars() {
        let indent = Indent::from_chars(4, &UTF_CHARS.into());
        assert_eq!(indent.regular_prefix, "├── ");
        assert_eq!(indent.last_regular_prefix, "└── ");
        assert_eq!(indent.child_prefix, "│   ");
        assert_eq!(indent.last_child_prefix, "    ");
    }

    #[test]
    fn indent_from_chars_ascii() {
        let indent = Indent::from_chars(6, &ASCII_CHARS_TICK.into());
        assert_eq!(indent.regular_prefix, "|---- ");
        assert_eq!(indent.last_regular_prefix, "`---- ");
        assert_eq!(indent.child_prefix, "|     ");
        assert_eq!(indent.last_child_prefix, "      ");
    }

    #[test]
    fn indent_from_config() {
        let config = {
            let mut config = PrintConfig::default();
            config.indent_size = 3;
            config.chars = UTF_CHARS.into();
            config
        };
        let indent = Indent::from_config(&config);
        assert_eq!(indent.regular_prefix, "├─ ");
        assert_eq!(indent.last_regular_prefix, "└─ ");
        assert_eq!(indent.child_prefix, "│  ");
        assert_eq!(indent.last_child_prefix, "   ");
    }
}
