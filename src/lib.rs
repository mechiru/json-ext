use serde::{de, ser, Deserialize, Serialize};
use serde_json::{Map, Value};

use std::convert::{From, Into};

/// This object is a placeholder for a custom json.
#[derive(Serialize, Deserialize, Default, Debug, PartialEq, Clone)]
pub struct Ext(Map<String, Value>);

impl Ext {
    /// Convert the value to an `Ext` object.
    pub fn try_from<T>(value: T) -> serde_json::Result<Self>
    where
        T: Serialize,
    {
        match serde_json::to_value(value)? {
            Value::Object(m) => Ok(Self(m)),
            _ => Err(ser::Error::custom(
                "must pass a value that can be converted to `serde_json::Map`",
            )),
        }
    }

    /// Converts to a specified type.
    pub fn try_into<T>(self) -> serde_json::Result<T>
    where
        T: de::DeserializeOwned,
    {
        serde_json::from_value(Value::Object(self.0))
    }
}

impl From<Map<String, Value>> for Ext {
    fn from(value: Map<String, Value>) -> Self {
        Self(value)
    }
}

impl Into<Map<String, Value>> for Ext {
    fn into(self) -> Map<String, Value> {
        self.0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn from_and_into() {
        let m = Map::new();
        let e: Ext = m.into();
        let _: Map<String, Value> = e.into();
    }

    #[test]
    fn try_from_and_try_into() -> serde_json::Result<()> {
        assert!(Ext::try_from(serde_json::json!(true)).is_err());

        let ext = Ext::try_from(serde_json::json!({"a": 1}))?;
        assert_eq!(
            ext,
            Ext({
                let mut m = Map::new();
                m.insert("a".into(), Value::Number(1.into()));
                m
            }),
        );
        assert_eq!(ext.try_into::<Value>()?, serde_json::json!({"a":  1}));

        #[derive(Serialize, Deserialize, Debug, PartialEq)]
        struct Object {
            f1: bool,
            f2: i32,
        }
        let ext = Ext::try_from(Object { f1: true, f2: 123 })?;
        assert_eq!(
            ext,
            Ext({
                let mut m = Map::new();
                m.insert("f1".into(), Value::Bool(true));
                m.insert("f2".into(), Value::Number(123.into()));
                m
            })
        );
        assert_eq!(ext.try_into::<Object>()?, Object { f1: true, f2: 123 });

        Ok(())
    }

    #[test]
    fn json() -> serde_json::Result<()> {
        let json = r#"{"a":1}"#;
        let ext: Ext = serde_json::from_str(json)?;
        assert_eq!(
            ext,
            Ext({
                let mut m = Map::new();
                m.insert("a".into(), Value::Number(1.into()));
                m
            })
        );
        assert_eq!(serde_json::to_string(&ext)?, json);

        #[derive(Serialize, Deserialize, Debug, PartialEq)]
        struct Object {
            ext: Ext,
        }
        let json = r#"{"ext":{"a":true}}"#;
        let obj: Object = serde_json::from_str(json)?;
        assert_eq!(
            obj,
            Object {
                ext: Ext({
                    let mut m = Map::new();
                    m.insert("a".into(), Value::Bool(true));
                    m
                })
            }
        );
        assert_eq!(serde_json::to_string(&obj)?, json);

        Ok(())
    }
}
