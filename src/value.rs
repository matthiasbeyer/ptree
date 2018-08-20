use item::TreeItem;
use config::PrintConfig;

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

    fn write_self<W: io::Write>(&self, f: &mut W, config: &PrintConfig) -> io::Result<()> {
        write!(f, "{}", config.paint_leaf(value_to_string(self)))
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
                            Value::String(format!(
                                "{} = {}",
                                value_to_string(k),
                                value_to_string(v)
                            )),
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

    fn write_self<W: io::Write>(&self, f: &mut W, config: &PrintConfig) -> io::Result<()> {
        if self.0.is_empty() {
            write!(f, "{}", config.paint_leaf(value_to_string(&self.1)))
        } else {
            write!(f, "{}", config.paint_leaf(&self.0))
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
                            Value::String(format!(
                                "{} = {}",
                                value_to_string(k),
                                value_to_string(v)
                            )),
                        ),
                    })
                    .collect();
                Cow::from(v)
            }
            _ => Cow::from(vec![]),
        }
    }
}
