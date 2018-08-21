use style::Style;

use std::io;
use std::borrow::Cow;

///
/// Main trait for exposing a tree structure to `ptree`
///
pub trait TreeItem: Clone {
    ///
    /// The type of this item's child items
    ///
    /// This is usually Self, but may be any type that itself implements TreeItem.
    ///
    type Child: TreeItem;

    ///
    /// Write the item's own contents (without children) to `f`
    ///
    /// The function returns an `io::Result<()>`, so calls to `f.write()` and `write!`
    /// can be chained with `?`.
    ///
    /// The provided `style` may be used for formatting hints.
    /// Usually, everything printed should be run through `Style::paint()`.
    /// However, this is not enforced, and custom implementations may choose to format
    /// only parts of the output, apply its own formatting in combination with the provided
    /// config, or ignore formatting altogether.
    ///
    fn write_self<W: io::Write>(&self, f: &mut W, style: &Style) -> io::Result<()>;

    ///
    /// Retrieve a list of this item's children
    ///
    /// If the items contains no children (it is a leaf item), this method returns an empty list.
    ///
    fn children(&self) -> Cow<[Self::Child]>;
}

///
/// A simple concrete implementation of `TreeItem` using `String`s
///
/// While a tree of `StringItem`s can be constructed directly,
/// it is usually easier to use a `TreeBuilder`.
///
#[derive(Clone, Debug)]
pub struct StringItem {
    /// The item's own text, to be returned by `write_self`
    pub text: String,
    /// The list of item's children
    pub children: Vec<StringItem>,
}

impl TreeItem for StringItem {
    type Child = Self;

    fn write_self<W: io::Write>(&self, f: &mut W, style: &Style) -> io::Result<()> {
        write!(f, "{}", style.paint(&self.text))
    }

    fn children(&self) -> Cow<[Self::Child]> {
        Cow::from(&self.children[..])
    }
}
