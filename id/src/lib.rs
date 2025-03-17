use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod reexports;

pub trait Id<'de>: From<Uuid> + Serialize + Deserialize<'de> + std::fmt::Display {
    const PREFIX: &'static str;

    fn get(self) -> Uuid;
}

pub const UUID_STRING_LENGTH: usize = 36;
pub const SEPARATOR: char = '-';
pub const SEPARATOR_LENGTH: usize = SEPARATOR.len_utf8();

#[macro_export]
macro_rules! define_id {
    ($name:ident, $prefix:expr) => {
        define_id!($name, $prefix, sql_grimoire_id);
    };
    ($name:ident, $prefix:expr, $id_crate:ident) => {
        use $id_crate::reexports::*;

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub struct $name(uuid::Uuid);

        impl<'de> $id_crate::Id<'de> for $name {
            const PREFIX: &'static str = $prefix;

            fn get(self) -> uuid::Uuid {
                self.0
            }
        }

        impl From<uuid::Uuid> for $name {
            fn from(id: uuid::Uuid) -> Self {
                Self(id)
            }
        }

        impl serde::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                use std::fmt::Write;

                use $id_crate::Id;

                let mut buf = String::with_capacity(
                    Self::PREFIX.len()
                        + $id_crate::UUID_STRING_LENGTH
                        + $id_crate::SEPARATOR_LENGTH,
                );
                buf.push_str(Self::PREFIX);
                buf.push($id_crate::SEPARATOR);
                write!(buf, "{}", self.0).unwrap();
                serializer.serialize_str(&buf)
            }
        }

        impl<'de> serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                use $id_crate::Id;

                let s = String::deserialize(deserializer)?;

                if !s.starts_with(Self::PREFIX)
                    || s.as_bytes().get(Self::PREFIX.len()) != Some(&b'-')
                {
                    return Err(serde::de::Error::custom("invalid ID prefix"));
                }

                let uuid = uuid::Uuid::parse_str(&s[Self::PREFIX.len() + 1..])
                    .map_err(serde::de::Error::custom)?;

                Ok(Self(uuid))
            }
        }

        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                use $id_crate::Id;

                write!(f, "{}-{}", Self::PREFIX, self.0)
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    define_id!(TestId, "test", crate);

    #[test]
    fn test_id() {
        use serde_test::{Token, assert_tokens};

        let id = uuid::Uuid::nil();
        let test_id = TestId::from(id);

        const EXPECTED_STRING: &str = "test-00000000-0000-0000-0000-000000000000";
        assert_eq!(
            EXPECTED_STRING.len(),
            UUID_STRING_LENGTH + SEPARATOR_LENGTH + TestId::PREFIX.len()
        );

        assert_tokens(&test_id, &[Token::String(EXPECTED_STRING)]);
    }
}
