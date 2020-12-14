use serde::Serializer;
use std::fmt;

/// serde from <T> which impled `Display` to string
pub fn serde_to_str<T, S>(value: &T, serializer: S) -> std::result::Result<S::Ok, S::Error>
where
    T: fmt::Display,
    S: Serializer,
{
    serializer.serialize_str(&*value.to_string())
}
