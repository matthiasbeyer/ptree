extern crate ansi_term;
extern crate ptree;
extern crate serde_any;
extern crate serde_value;

#[macro_use]
extern crate structopt;

use structopt::StructOpt;

use std::path::PathBuf;
use std::fs::File;
use ansi_term::{Color, Style};
use ptree::config;

#[derive(Clone, Debug, StructOpt)]
struct Opt {
    #[structopt(name = "FILE", parse(from_os_str))]
    file: PathBuf,

    #[structopt(short = "d", long = "depth")]
    depth: Option<u32>,

    #[structopt(short = "o", long = "output", parse(from_os_str))]
    output: Option<PathBuf>,

    #[structopt(short = "l", long = "leaf-style", parse(from_str = "style_from_str"))]
    leaf_style: Option<Style>,

    #[structopt(short = "b", long = "branch-style", parse(from_str = "style_from_str"))]
    branch_style: Option<Style>,

    #[structopt(short = "c", long = "character-set", parse(from_str = "chars_from_str"))]
    character_set: Option<ptree::IndentChars<'static>>,

    #[structopt(short = "i", long = "indent")]
    indent: Option<usize>,
}

fn chars_from_str(s: &&str) -> ptree::IndentChars<'static> {
    match &s.to_lowercase()[..] {
        "ascii" | "ascii-plus" => config::ASCII_CHARS_PLUS,
        "ascii-tick" => config::ASCII_CHARS_TICK,
        "utf" => config::UTF_CHARS,
        "utf-bold" => config::UTF_CHARS_BOLD,
        "utf-double" => config::UTF_CHARS_DOUBLE,
        _ => config::UTF_CHARS,
    }
}

fn style_from_str(s: &&str) -> Style {
    let mut style = Style::new();

    for i in s.split(",") {
        match &i.to_lowercase()[..] {
            "black" => style = style.fg(Color::Black),
            "on_black" => style = style.on(Color::Black),
            "red" => style = style.fg(Color::Red),
            "on_red" => style = style.on(Color::Red),
            "green" => style = style.fg(Color::Green),
            "on_green" => style = style.on(Color::Green),
            "yellow" => style = style.fg(Color::Yellow),
            "on_yellow" => style = style.on(Color::Yellow),
            "blue" => style = style.fg(Color::Blue),
            "on_blue" => style = style.on(Color::Blue),
            "purple" => style = style.fg(Color::Purple),
            "on_purple" => style = style.on(Color::Purple),
            "cyan" => style = style.fg(Color::Cyan),
            "on_cyan" => style = style.on(Color::Cyan),
            "white" => style = style.fg(Color::White),
            "on_white" => style = style.on(Color::White),
            "bold" => style = style.bold(),
            "dimmed" => style = style.dimmed(),
            "italic" => style = style.italic(),
            "underline" => style = style.underline(),
            "blink" => style = style.blink(),
            "reverse" => style = style.reverse(),
            "hidden" => style = style.hidden(),
            "strikethrough" => style = style.strikethrough(),
            _ => {}
        }
    }

    style
}

fn main() {
    let opt = Opt::from_args();

    let value: serde_value::Value = serde_any::from_file(&opt.file).expect("Error loading file");
    let config = {
        let mut config = if opt.output.is_some() {
            ptree::PrintConfig::default()
        } else {
            ptree::PrintConfig::for_stdout()
        };
        if let Some(d) = opt.depth {
            config.max_depth = d;
        }
        if let Some(b) = opt.branch_style {
            config.branch_style = b;
        }
        if let Some(l) = opt.leaf_style {
            config.leaf_style = l;
        }
        if let Some(i) = opt.indent {
            config.indent_size = i;
        }
        if let Some(c) = opt.character_set {
            config.chars = c;
        }
        config
    };

    let tree = (opt.file.display().to_string(), value);

    if let Some(output) = opt.output {
        let mut out = File::create(output).expect("Cannot create output file");
        ptree::write_tree_with(&tree, &mut out, &config).expect("Cannot write tree to file");
    } else {
        ptree::print_tree_with(&tree, &config).expect("Cannot write tree to standard output");
    }
}
