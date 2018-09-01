use item::TreeItem;
use style::Style;

use std::io;
use std::borrow::Cow;

use serde_value::Value;

fn value_to_string(v: &Value) -> String {
    match v {
        Value::Bool(b) => b.to_string(),
        Value::U8(u) => u.to_string(),
        Value::U16(u) => u.to_string(),
        Value::U32(u) => u.to_string(),
        Value::U64(u) => u.to_string(),
        Value::I8(i) => i.to_string(),
        Value::I16(i) => i.to_string(),
        Value::I32(i) => i.to_string(),
        Value::I64(i) => i.to_string(),
        Value::F32(f) => f.to_string(),
        Value::F64(f) => f.to_string(),
        Value::Char(c) => c.to_string(),
        Value::String(s) => s.clone(),
        Value::Option(Some(b)) => value_to_string(&*b),
        Value::Newtype(b) => value_to_string(&*b),
        _ => "".to_string(),
    }
}

impl TreeItem for Value {
    type Child = (String, Value);

    fn write_self<W: io::Write>(&self, f: &mut W, style: &Style) -> io::Result<()> {
        write!(f, "{}", style.paint(value_to_string(self)))
    }

    fn children(&self) -> Cow<[Self::Child]> {
        match self {
            Value::Seq(v) => Cow::from(
                v.iter()
                    .map(|v| ("".to_string(), v.clone()))
                    .collect::<Vec<_>>(),
            ),
            Value::Map(m) => {
                let v: Vec<_> = m.iter()
                    .map(|(k, v)| match v {
                        Value::Seq(_) => (value_to_string(k), v.clone()),
                        Value::Map(_) => (value_to_string(k), v.clone()),
                        _ => (
                            "".to_string(),
                            Value::String(format!("{} = {}", value_to_string(k), value_to_string(v))),
                        ),
                    })
                    .collect();
                Cow::from(v)
            }
            _ => Cow::from(vec![]),
        }
    }
}

impl TreeItem for (String, Value) {
    type Child = Self;

    fn write_self<W: io::Write>(&self, f: &mut W, style: &Style) -> io::Result<()> {
        if self.0.is_empty() {
            write!(f, "{}", style.paint(value_to_string(&self.1)))
        } else {
            write!(f, "{}", style.paint(&self.0))
        }
    }

    fn children(&self) -> Cow<[Self::Child]> {
        match &self.1 {
            Value::Seq(v) => Cow::from(
                v.iter()
                    .map(|v| ("".to_string(), v.clone()))
                    .collect::<Vec<_>>(),
            ),
            Value::Map(m) => {
                let v: Vec<_> = m.iter()
                    .map(|(k, v)| match v {
                        Value::Seq(_) => (value_to_string(k), v.clone()),
                        Value::Map(_) => (value_to_string(k), v.clone()),
                        _ => (
                            "".to_string(),
                            Value::String(format!("{} = {}", value_to_string(k), value_to_string(v))),
                        ),
                    })
                    .collect();
                Cow::from(v)
            }
            _ => Cow::from(vec![]),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use std::str::from_utf8;
    use super::*;

    use output::write_tree_with;
    use print_config::PrintConfig;

    use serde_any;

    #[test]
    fn toml_value_output() {
        let toml = "\
                    configuration = [\"toml\", \"yaml\", \"json\", \"environment\"]\n\
                    charsets = [\"utf\", \"ascii\"]\n\
                    \n\
                    default_depth = 3\n\
                    \n\
                    ";

        let value: Value = serde_any::from_str(toml, serde_any::Format::Toml).unwrap();
        let tree = ("toml".to_string(), value);

        let config = PrintConfig {
            indent: 4,
            leaf: Style::default(),
            branch: Style::default(),
            ..PrintConfig::default()
        };

        let mut cursor: Cursor<Vec<u8>> = Cursor::new(Vec::new());

        write_tree_with(&tree, &mut cursor, &config).unwrap();

        let data = cursor.into_inner();
        let expected = "\
                        toml\n\
                        ├── charsets\n\
                        │   ├── utf\n\
                        │   └── ascii\n\
                        ├── configuration\n\
                        │   ├── toml\n\
                        │   ├── yaml\n\
                        │   ├── json\n\
                        │   └── environment\n\
                        └── default_depth = 3\n\
                        ";
        assert_eq!(from_utf8(&data).unwrap(), expected);
    }
}
