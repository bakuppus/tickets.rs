use super::util;
use serde::de::Error;
use serde::ser::SerializeSeq;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;
use std::fmt;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Snowflake(pub u64);

impl Snowflake {
    pub fn serialize_to_int<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_u64(self.0)
    }

    pub fn serialize_vec_to_ints<S: Serializer>(
        vec: &[Snowflake],
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        let mut seq = serializer.serialize_seq(Some(vec.len()))?;

        for snowflake in vec {
            seq.serialize_element(&snowflake)?;
        }

        seq.end()
    }

    pub fn serialize_option_to_int<S: Serializer>(
        op: &Option<Snowflake>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        match op {
            Some(s) => s.serialize_to_int(serializer),
            None => serializer.serialize_none(),
        }
    }
}

impl Serialize for Snowflake {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.0.to_string())
    }
}

impl<'de> Deserialize<'de> for Snowflake {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value: Value = Deserialize::deserialize(deserializer)?;

        if let Some(i) = value.as_u64() {
            return Ok(Snowflake(i));
        }

        if let Some(s) = value.as_str() {
            return Ok(Snowflake(s.parse().map_err(Error::custom)?));
        }

        Err(Error::invalid_type(
            util::to_unexpected(value),
            &"a string or u64",
        ))
    }
}

impl fmt::Display for Snowflake {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Snowflake {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Snowflake(s.parse()?))
    }
}

impl From<u64> for Snowflake {
    fn from(x: u64) -> Self {
        Snowflake(x)
    }
}

/*impl<'r> sqlx::Decode<'r, Postgres> for Snowflake {
    fn decode(value: PgValueRef<'_>) -> Result<Self, BoxDynError> {
        let i = i64::decode(value)?;
        Ok(Snowflake(i as u64))
    }
}

impl<'q> sqlx::Encode<'q, Postgres> for Snowflake {
    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> IsNull {
        buf.extend(&(self.0 as i64).to_le_bytes());
        IsNull::No
    }
}*/
