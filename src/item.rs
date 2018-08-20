use config::PrintConfig;

use std::io;
use std::borrow::Cow;

pub trait TreeItem: Clone {
    type Child: TreeItem;

    fn write_self<W: io::Write>(&self, f: &mut W, config: &PrintConfig) -> io::Result<()>;
    fn children(&self) -> Cow<[Self::Child]>;
}

#[derive(Clone, Debug)]
pub struct StringItem {
    pub text: String,
    pub children: Vec<StringItem>,
}

impl TreeItem for StringItem {
    type Child = Self;

    fn write_self<W: io::Write>(&self, f: &mut W, config: &PrintConfig) -> io::Result<()> {
        write!(f, "{}", config.paint_leaf(self.text.clone()))
    }

    fn children(&self) -> Cow<[Self::Child]> {
        Cow::from(&self.children[..])
    }
}
