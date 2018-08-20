use item::*;
use config::*;

use std::io;

fn print_item<T: TreeItem, W: io::Write>(
    item: &T,
    f: &mut W,
    prefix: String,
    child_prefix: String,
    config: &PrintConfig,
    chars: &Indent,
    level: u32,
) -> io::Result<()> {
    write!(f, "{}", config.paint_branch(prefix))?;
    item.write_self(f, config)?;
    writeln!(f, "")?;

    if level < config.max_depth {
        let children = item.children();
        if let Some((last_child, children)) = children.split_last() {
            let rp = child_prefix.clone() + &chars.regular_prefix;
            let cp = child_prefix.clone() + &chars.child_prefix;

            for c in children {
                print_item(c, f, rp.clone(), cp.clone(), config, chars, level + 1)?;
            }

            let rp = child_prefix.clone() + &chars.last_regular_prefix;
            let cp = child_prefix.clone() + &chars.last_child_prefix;

            print_item(last_child, f, rp, cp, config, chars, level + 1)?;
        }
    }

    Ok(())
}

pub fn print_tree<T: TreeItem>(item: &T) -> io::Result<()> {
    print_tree_with(item, &PrintConfig::for_stdout())
}

pub fn print_tree_with<T: TreeItem>(item: &T, config: &PrintConfig) -> io::Result<()> {
    let chars = config.create_indent_chars();
    let out = io::stdout();
    let mut handle = out.lock();
    print_item(
        item,
        &mut handle,
        "".to_string(),
        "".to_string(),
        config,
        &chars,
        0,
    )
}

pub fn write_tree<T: TreeItem, W: io::Write>(item: &T, mut f: W) -> io::Result<()> {
    write_tree_with(item, &mut f, &PrintConfig::default())
}

pub fn write_tree_with<T: TreeItem, W: io::Write>(
    item: &T,
    mut f: W,
    config: &PrintConfig,
) -> io::Result<()> {
    let chars = config.create_indent_chars();
    print_item(
        item,
        &mut f,
        "".to_string(),
        "".to_string(),
        config,
        &chars,
        0,
    )
}
