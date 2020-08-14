use item::*;
use print_config::*;
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
        Self::from_characters(config.indent, &config.characters)
    }

    pub fn from_characters(indent_size: usize, characters: &IndentChars) -> Indent {
        let n = if indent_size > 2 { indent_size - 2 } else { 0 };

        let right_pad = characters.right.repeat(n);
        let empty_pad = characters.empty.repeat(n);

        Indent {
            regular_prefix: format!("{}{} ", characters.down_and_right, right_pad),
            child_prefix: format!("{}{} ", characters.down, empty_pad),
            last_regular_prefix: format!("{}{} ", characters.turn_right, right_pad),
            last_child_prefix: format!("{}{} ", characters.empty, empty_pad),
        }
    }
}

fn print_item<T: TreeItem, W: io::Write>(
    item: &T,
    f: &mut W,
    prefix: String,
    child_prefix: String,
    config: &PrintConfig,
    characters: &Indent,
    branch_style: &Style,
    leaf_style: &Style,
    level: u32,
) -> io::Result<()> {
    write!(f, "{}", branch_style.paint(prefix))?;
    item.write_self(f, leaf_style)?;
    writeln!(f, "")?;

    if level < config.depth {
        let children = item.children();
        if let Some((last_child, children)) = children.split_last() {
            let rp = child_prefix.clone() + &characters.regular_prefix;
            let cp = child_prefix.clone() + &characters.child_prefix;

            for c in children {
                print_item(
                    c,
                    f,
                    rp.clone(),
                    cp.clone(),
                    config,
                    characters,
                    branch_style,
                    leaf_style,
                    level + 1,
                )?;
            }

            let rp = child_prefix.clone() + &characters.last_regular_prefix;
            let cp = child_prefix.clone() + &characters.last_child_prefix;

            print_item(
                last_child,
                f,
                rp,
                cp,
                config,
                characters,
                branch_style,
                leaf_style,
                level + 1,
            )?;
        }
    }

    Ok(())
}

/// Print the tree `item` to standard output using default formatting
pub fn print_tree<T: TreeItem>(item: &T) -> io::Result<()> {
    print_tree_with(item, &PrintConfig::from_env())
}

/// Print the tree `item` to standard output using custom formatting
pub fn print_tree_with<T: TreeItem>(item: &T, config: &PrintConfig) -> io::Result<()> {
    let (branch_style, leaf_style) = if config.should_style_output(OutputKind::Stdout) {
        (config.branch.clone(), config.leaf.clone())
    } else {
        (Style::default(), Style::default())
    };

    let characters = Indent::from_config(config);
    let out = io::stdout();
    let mut handle = out.lock();
    print_item(
        item,
        &mut handle,
        "".to_string(),
        "".to_string(),
        config,
        &characters,
        &branch_style,
        &leaf_style,
        0,
    )
}

/// Write the tree `item` to writer `f` using default formatting
pub fn write_tree<T: TreeItem, W: io::Write>(item: &T, mut f: W) -> io::Result<()> {
    write_tree_with(item, &mut f, &PrintConfig::from_env())
}

/// Write the tree `item` to writer `f` using custom formatting
pub fn write_tree_with<T: TreeItem, W: io::Write>(item: &T, mut f: W, config: &PrintConfig) -> io::Result<()> {
    let (branch_style, leaf_style) = if config.should_style_output(OutputKind::Unknown) {
        (config.branch.clone(), config.leaf.clone())
    } else {
        (Style::default(), Style::default())
    };

    let characters = Indent::from_config(config);
    print_item(
        item,
        &mut f,
        "".to_string(),
        "".to_string(),
        config,
        &characters,
        &branch_style,
        &leaf_style,
        0,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use print_config::PrintConfig;

    #[test]
    fn indent_from_characters() {
        let indent = Indent::from_characters(4, &UTF_CHARS.into());
        assert_eq!(indent.regular_prefix, "├── ");
        assert_eq!(indent.last_regular_prefix, "└── ");
        assert_eq!(indent.child_prefix, "│   ");
        assert_eq!(indent.last_child_prefix, "    ");
    }

    #[test]
    fn indent_from_characters_ascii() {
        let indent = Indent::from_characters(6, &ASCII_CHARS_TICK.into());
        assert_eq!(indent.regular_prefix, "|---- ");
        assert_eq!(indent.last_regular_prefix, "`---- ");
        assert_eq!(indent.child_prefix, "|     ");
        assert_eq!(indent.last_child_prefix, "      ");
    }

    #[test]
    fn indent_from_config() {
        let config = {
            let mut config = PrintConfig::default();
            config.indent = 3;
            config.characters = UTF_CHARS.into();
            config
        };
        let indent = Indent::from_config(&config);
        assert_eq!(indent.regular_prefix, "├─ ");
        assert_eq!(indent.last_regular_prefix, "└─ ");
        assert_eq!(indent.child_prefix, "│  ");
        assert_eq!(indent.last_child_prefix, "   ");
    }
}
