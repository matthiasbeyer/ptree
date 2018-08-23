extern crate ptree;
extern crate serde_any;
extern crate serde_value;

#[macro_use]
extern crate structopt;

use structopt::StructOpt;

use std::path::PathBuf;
use std::fs::File;
use ptree::style::{Color, Style};
use ptree::print_config;

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
    character_set: Option<ptree::IndentChars>,

    #[structopt(short = "i", long = "indent")]
    indent: Option<usize>,
}

fn chars_from_str(s: &&str) -> ptree::IndentChars {
    match &s.to_lowercase()[..] {
        "ascii" | "ascii-plus" => print_config::ASCII_CHARS_PLUS.into(),
        "ascii-tick" => print_config::ASCII_CHARS_TICK.into(),
        "utf" => print_config::UTF_CHARS.into(),
        "utf-bold" => print_config::UTF_CHARS_BOLD.into(),
        "utf-double" => print_config::UTF_CHARS_DOUBLE.into(),
        _ => print_config::UTF_CHARS.into(),
    }
}

fn style_from_str(s: &&str) -> Style {
    let mut style = Style::default();

    for i in s.split(",") {
        match &i.to_lowercase()[..] {
            "black" => style.foreground = Some(Color::Black),
            "on_black" => style.background = Some(Color::Black),
            "red" => style.foreground = Some(Color::Red),
            "on_red" => style.background = Some(Color::Red),
            "green" => style.foreground = Some(Color::Green),
            "on_green" => style.background = Some(Color::Green),
            "yellow" => style.foreground = Some(Color::Yellow),
            "on_yellow" => style.background = Some(Color::Yellow),
            "blue" => style.foreground = Some(Color::Blue),
            "on_blue" => style.background = Some(Color::Blue),
            "purple" => style.foreground = Some(Color::Purple),
            "on_purple" => style.background = Some(Color::Purple),
            "cyan" => style.foreground = Some(Color::Cyan),
            "on_cyan" => style.background = Some(Color::Cyan),
            "white" => style.foreground = Some(Color::White),
            "on_white" => style.background = Some(Color::White),
            "bold" => style.bold = true,
            "dimmed" => style.dimmed = true,
            "italic" => style.italic = true,
            "underline" => style.underline = true,
            "blink" => style.blink = true,
            "reverse" => style.reverse = true,
            "hidden" => style.hidden = true,
            "strikethrough" => style.strikethrough = true,
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
