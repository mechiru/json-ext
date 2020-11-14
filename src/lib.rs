//! This library provides an extension object for [serde_json](crates.io/crates/serde_json) crate.

use serde::{de, ser};
use serde_json::value::RawValue;

/// Represents a custom JSON object.
#[derive(Debug, Clone)]
pub struct Ext<'a>(&'a RawValue);

impl<'a> Ext<'a> {
    /// Convert a `&RawValue` to an `Ext` object.
    pub fn try_from<'b>(raw: &'b RawValue) -> serde_json::Result<Self>
    where
        'b: 'a,
    {
        check_raw(raw).map(Self).map_err(ser::Error::custom)
    }

    /// Converts to a specified type.
    pub fn try_into<'b, T>(&'a self) -> serde_json::Result<T>
    where
        T: serde::Deserialize<'b>,
        'a: 'b,
    {
        serde_json::from_str(self.0.get())
    }
}

impl<'a> PartialEq for Ext<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.0.get() == other.0.get()
    }
}

impl<'a> ser::Serialize for Ext<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'de: 'a, 'a> de::Deserialize<'de> for Ext<'a> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let raw: &'a RawValue = de::Deserialize::deserialize(deserializer)?;
        check_raw(raw).map(Self).map_err(de::Error::custom)
    }
}

fn check_raw(raw: &RawValue) -> Result<&RawValue, &'static str> {
    match raw.get().as_bytes().get(0) {
        Some(b'{') | Some(b'n') => Ok(raw),
        _ => Err("invalid value: expected null or object"),
    }
}

#[cfg(test)]
mod test {
    use serde::{Deserialize, Serialize};
    use serde_json::{from_str, value::to_raw_value, Map, Value};

    use super::*;

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct Object<'a> {
        #[serde(borrow)]
        ext: Ext<'a>,
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct Extension<'a> {
        f1: &'a str,
        f2: i32,
    }

    #[test]
    fn try_from_and_get() -> serde_json::Result<()> {
        let object = Value::Object({
            let mut m = Map::new();
            m.insert("f1".into(), Value::String("abc".into()));
            m.insert("f2".into(), Value::Number(123.into()));
            m
        });
        let raw = to_raw_value(&object)?;
        let ext = Ext::try_from(raw.as_ref())?;
        let ext2 = from_str::<Extension>(ext.0.get())?;
        assert_eq!(ext2, Extension { f1: "abc", f2: 123 });

        Ok(())
    }

    #[test]
    fn json() -> serde_json::Result<()> {
        assert!(from_str::<Object>(r#"{"ext":null}"#).is_ok());
        assert!(from_str::<Object>(r#"{"ext":true}"#).is_err());
        assert!(from_str::<Object>(r#"{"ext":1}"#).is_err());
        assert!(from_str::<Object>(r#"{"ext":"1"}"#).is_err());
        assert!(from_str::<Object>(r#"{"ext":[1]}"#).is_err());
        assert!(from_str::<Object>(r#"{"ext":{}}"#).is_ok());

        let s1 = r#"{"ext": 	
null}"#;
        let o1 = from_str::<Object>(s1)?;
        assert_eq!(
            o1,
            Object {
                ext: Ext::try_from(to_raw_value(&Value::Null)?.as_ref())?,
            }
        );
        assert_eq!(serde_json::to_string(&o1)?, r#"{"ext":null}"#);

        let s2 = r#"{"ext": 	
{}
 	}"#;
        let o2 = serde_json::from_str::<Object>(s2)?;
        assert_eq!(
            o2,
            Object {
                ext: Ext::try_from(to_raw_value(&Value::Object(Default::default()))?.as_ref())?,
            }
        );
        assert_eq!(serde_json::to_string(&o2)?, r#"{"ext":{}}"#);

        Ok(())
    }
}
