use std::fmt::Display;

#[cfg(feature = "ansi")]
use ansi_term;
use serde::{Deserialize, Serialize};
#[cfg(feature = "ansi")]
use tint;

///
/// Terminal output style
///
#[derive(Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Style {
    /// The style's foreground colour, if it has one.
    pub foreground: Option<Color>,

    /// The style's background colour, if it has one.
    pub background: Option<Color>,

    /// Whether this style is bold.
    pub bold: bool,

    /// Whether this style is dimmed.
    pub dimmed: bool,

    /// Whether this style is italic.
    pub italic: bool,

    /// Whether this style is underlined.
    pub underline: bool,

    /// Whether this style is blinking.
    pub blink: bool,

    /// Whether this style has reverse colours.
    pub reverse: bool,

    /// Whether this style is hidden.
    pub hidden: bool,

    /// Whether this style is struckthrough.
    pub strikethrough: bool,
}

/// A colour is one specific type of ANSI escape code, and can refer
/// to either the foreground or background colour.
///
/// These use the standard numeric sequences.
/// See <http://invisible-island.net/xterm/ctlseqs/ctlseqs.html>
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged, rename_all = "lowercase")]
pub enum Color {
    /// Color #0 (foreground code `30`, background code `40`).
    ///
    /// This is not necessarily the background colour, and using it as one may
    /// render the text hard to read on terminals with dark backgrounds.
    Black,

    /// Color #1 (foreground code `31`, background code `41`).
    Red,

    /// Color #2 (foreground code `32`, background code `42`).
    Green,

    /// Color #3 (foreground code `33`, background code `43`).
    Yellow,

    /// Color #4 (foreground code `34`, background code `44`).
    Blue,

    /// Color #5 (foreground code `35`, background code `45`).
    Purple,

    /// Color #6 (foreground code `36`, background code `46`).
    Cyan,

    /// Color #7 (foreground code `37`, background code `47`).
    ///
    /// As above, this is not necessarily the foreground colour, and may be
    /// hard to read on terminals with light backgrounds.
    White,

    /// A colour number from 0 to 255, for use in 256-colour terminal
    /// environments.
    ///
    /// - Colors 0 to 7 are the `Black` to `White` variants respectively.
    ///   These colours can usually be changed in the terminal emulator.
    /// - Colors 8 to 15 are brighter versions of the eight colours above.
    ///   These can also usually be changed in the terminal emulator, or it
    ///   could be configured to use the original colours and show the text in
    ///   bold instead. It varies depending on the program.
    /// - Colors 16 to 231 contain several palettes of bright colours,
    ///   arranged in six squares measuring six by six each.
    /// - Colors 232 to 255 are shades of grey from black to white.
    ///
    /// It might make more sense to look at a [colour chart][cc].
    ///
    /// [cc]: https://upload.wikimedia.org/wikipedia/commons/1/15/Xterm_256color_chart.svg
    Fixed(u8),

    /// A 24-bit RGB color, as specified by ISO-8613-3.
    RGB(u8, u8, u8),

    /// A named color, as supported by `tint`.
    Named(String),
}

impl Default for Color {
    fn default() -> Color {
        Color::Black
    }
}

impl Color {
    fn to_ansi_color(&self) -> ansi_term::Color {
        match self {
            Color::Black => ansi_term::Color::Black,
            Color::Red => ansi_term::Color::Red,
            Color::Green => ansi_term::Color::Green,
            Color::Yellow => ansi_term::Color::Yellow,
            Color::Blue => ansi_term::Color::Blue,
            Color::Purple => ansi_term::Color::Purple,
            Color::Cyan => ansi_term::Color::Cyan,
            Color::White => ansi_term::Color::White,
            Color::Fixed(f) => ansi_term::Color::Fixed(*f),
            Color::RGB(r, g, b) => ansi_term::Color::RGB(*r, *g, *b),
            Color::Named(n) => match &n[..] {
                // ANSI color names still take precedence over HTML and CSS colors,
                // because only ANSI colors can be dimmed.
                "black" => ansi_term::Color::Black,
                "red" => ansi_term::Color::Red,
                "green" => ansi_term::Color::Green,
                "yellow" => ansi_term::Color::Yellow,
                "blue" => ansi_term::Color::Blue,
                "purple" => ansi_term::Color::Purple,
                "cyan" => ansi_term::Color::Cyan,
                "white" => ansi_term::Color::White,
                n => {
                    let c = tint::Color::from(n);
                    let (r, g, b) = c.to_rgb255();
                    ansi_term::Color::RGB(r, g, b)
                }
            },
        }
    }
}

impl Style {
    ///
    /// Paints `input` according to this style.
    ///
    /// If the `"ansi"` feature is enabled, this function uses
    /// `ansi_term` to style text.
    ///
    /// If the `"ansi"` feature is disabled, this function
    /// always returns the output unchanged.
    ///
    pub fn paint(&self, input: impl Display) -> impl Display {
        #[cfg(feature = "ansi")]
        {
            let mut ansi_style = ansi_term::Style::new();

            ansi_style.foreground = self.foreground.as_ref().map(Color::to_ansi_color);
            ansi_style.background = self.background.as_ref().map(Color::to_ansi_color);

            ansi_style.is_bold = self.bold;
            ansi_style.is_dimmed = self.dimmed;
            ansi_style.is_italic = self.italic;
            ansi_style.is_underline = self.underline;

            ansi_style.paint(input.to_string())
        }

        #[cfg(not(feature = "ansi"))]
        return input;
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use ansi_term;
    use serde_any;

    #[derive(Deserialize)]
    pub struct Wrapper {
        color: Color,
    }

    fn toml_to_ansi(s: &str) -> ansi_term::Color {
        serde_any::from_str::<Wrapper>(&format!("color = {}", s), serde_any::Format::Toml)
            .unwrap()
            .color
            .to_ansi_color()
    }

    fn yaml_to_ansi(s: &str) -> ansi_term::Color {
        serde_any::from_str::<Wrapper>(&format!("color: {}", s), serde_any::Format::Yaml)
            .unwrap()
            .color
            .to_ansi_color()
    }

    #[test]
    fn color_from_toml() {
        assert_eq!(toml_to_ansi("\"red\""), ansi_term::Color::Red);
        assert_eq!(toml_to_ansi("\"green\""), ansi_term::Color::Green);
        assert_eq!(toml_to_ansi("10"), ansi_term::Color::Fixed(10));
        assert_eq!(toml_to_ansi("110"), ansi_term::Color::Fixed(110));
        assert_eq!(toml_to_ansi("[10, 20, 30]"), ansi_term::Color::RGB(10, 20, 30));
        assert_eq!(toml_to_ansi("\"maroon\""), ansi_term::Color::RGB(128, 0, 0));
        assert_eq!(toml_to_ansi("\"steelblue\""), ansi_term::Color::RGB(70, 130, 180));
        assert_eq!(toml_to_ansi("\"#4682B4\""), ansi_term::Color::RGB(70, 130, 180));
    }

    #[test]
    fn color_from_yaml() {
        assert_eq!(yaml_to_ansi("\"red\""), ansi_term::Color::Red);
        assert_eq!(yaml_to_ansi("\"green\""), ansi_term::Color::Green);
        assert_eq!(yaml_to_ansi("10"), ansi_term::Color::Fixed(10));
        assert_eq!(yaml_to_ansi("110"), ansi_term::Color::Fixed(110));
        assert_eq!(yaml_to_ansi("[10, 20, 30]"), ansi_term::Color::RGB(10, 20, 30));
        assert_eq!(yaml_to_ansi("\"maroon\""), ansi_term::Color::RGB(128, 0, 0));
        assert_eq!(yaml_to_ansi("\"steelblue\""), ansi_term::Color::RGB(70, 130, 180));
        assert_eq!(yaml_to_ansi("\"#4682B4\""), ansi_term::Color::RGB(70, 130, 180));
    }

    #[test]
    fn style_from_toml() {
        let toml = "foreground = \"#102030\"\nbackground = 3\ndimmed = true\nbold = true";
        let actual = serde_any::from_str::<Style>(toml, serde_any::Format::Toml).unwrap();
        let expected = Style {
            dimmed: true,
            bold: true,
            foreground: Some(Color::Named("#102030".to_string())),
            background: Some(Color::Fixed(3)),
            ..Style::default()
        };

        assert_eq!(actual, expected);
    }
}
