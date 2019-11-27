use {
    super::{StrIndex, StrRange},
    core::fmt,
    serde::{
        de::{Deserialize, Deserializer, Error, MapAccess, SeqAccess, Visitor},
        ser::{Serialize, SerializeStruct, Serializer},
    },
};

impl Serialize for StrIndex {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_newtype_struct("StrIndex", &self.raw)
    }
}

impl<'de> Deserialize<'de> for StrIndex {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_newtype_struct("StrIndex", StrIndexVisitor)
    }
}

impl Serialize for StrRange {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("StrRange", 2)?;
        s.serialize_field("start", &self.start)?;
        s.serialize_field("end", &self.end)?;
        s.end()
    }
}

impl<'de> Deserialize<'de> for StrRange {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_struct("StrRange", STR_RANGE_FIELDS, StrRangeVisitor)
    }
}

struct StrIndexVisitor;

impl<'de> Visitor<'de> for StrIndexVisitor {
    type Value = StrIndex;

    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "u32")
    }

    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(StrIndex::from(v))
    }

    fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_u32(self)
    }
}

struct StrRangeVisitor;

impl<'de> Visitor<'de> for StrRangeVisitor {
    type Value = StrRange;

    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "struct StrRange")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let start: StrIndex = match seq.next_element()? {
            Some(it) => it,
            None => return Err(Error::invalid_length(0, &self)),
        };
        let end: StrIndex = match seq.next_element()? {
            Some(it) => it,
            None => return Err(Error::invalid_length(1, &self)),
        };
        let range = StrRange { start, end }; // construct manually to bypass ordering assert!
        if start > end {
            Err(Error::custom(format_args!(
                "invalid string range {}",
                range
            )))
        } else {
            Ok(range)
        }
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut start: Option<StrIndex> = None;
        let mut end: Option<StrIndex> = None;
        while let Some(key) = map.next_key()? {
            match key {
                StrRangeField::Start => {
                    if start.is_some() {
                        return Err(Error::duplicate_field("start"));
                    } else {
                        start = Some(map.next_value()?)
                    }
                }
                StrRangeField::End => {
                    if end.is_some() {
                        return Err(Error::duplicate_field("end"));
                    } else {
                        end = Some(map.next_value()?)
                    }
                }
            }
        }
        let start = match start {
            Some(it) => it,
            None => return Err(Error::missing_field("start")),
        };
        let end = match end {
            Some(it) => it,
            None => return Err(Error::missing_field("end")),
        };
        let range = StrRange { start, end }; // construct manually to bypass ordering assert!
        if start > end {
            Err(Error::custom(format_args!(
                "invalid string range {}",
                range
            )))
        } else {
            Ok(range)
        }
    }
}

const STR_RANGE_FIELDS: &[&str] = &["start", "end"];

enum StrRangeField {
    Start,
    End,
}

impl<'de> Deserialize<'de> for StrRangeField {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_identifier(StrRangeFieldVisitor)
    }
}

struct StrRangeFieldVisitor;

impl<'de> Visitor<'de> for StrRangeFieldVisitor {
    type Value = StrRangeField;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("`start` or `end`")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        match value {
            "start" => Ok(StrRangeField::Start),
            "end" => Ok(StrRangeField::End),
            _ => Err(Error::unknown_field(value, STR_RANGE_FIELDS)),
        }
    }

    fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
    where
        E: Error,
    {
        match value {
            b"start" => Ok(StrRangeField::Start),
            b"end" => Ok(StrRangeField::End),
            _ => {
                let value = serde::export::from_utf8_lossy(value);
                Err(Error::unknown_field(&value, STR_RANGE_FIELDS))
            }
        }
    }
}
