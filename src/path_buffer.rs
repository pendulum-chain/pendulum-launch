use crate::error::{Error, Result};
use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};
use std::{
    ffi::OsStr,
    fmt,
    path::{Path, PathBuf},
};

/// A wrapper around PathBuf for extended impl
#[derive(Debug)]
pub struct PathBuffer(PathBuf);

impl PathBuffer {
    pub fn to_str(&self) -> Option<&str> {
        self.0.to_str()
    }

    pub fn to_string(&self) -> Result<String> {
        match self.to_str() {
            Some(str) => Ok(str.to_owned()),
            None => Err(Error::InvalidPath),
        }
    }

    pub fn as_os_str(&self) -> &OsStr {
        self.0.as_os_str()
    }

    pub fn join<P: AsRef<Path>>(&self, path: P) -> Self {
        Self::from(self.0.join(path))
    }

    pub fn maybe_from(value: Option<&str>) -> Option<Self> {
        value.map(Self::from)
    }
}

impl From<&str> for PathBuffer {
    fn from(value: &str) -> Self {
        PathBuffer(PathBuf::from(value))
    }
}

impl From<PathBuf> for PathBuffer {
    fn from(path: PathBuf) -> Self {
        PathBuffer(path)
    }
}

impl AsRef<PathBuf> for PathBuffer {
    fn as_ref(&self) -> &PathBuf {
        &self.0
    }
}

struct PathBufferVisitor;

impl<'de> Visitor<'de> for PathBufferVisitor {
    type Value = PathBuffer;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("a valid path string")
    }

    fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(PathBuffer::from(value))
    }
}

impl<'de> Deserialize<'de> for PathBuffer {
    fn deserialize<D>(deserializer: D) -> std::result::Result<PathBuffer, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(PathBufferVisitor)
    }
}

impl Serialize for PathBuffer {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_str().unwrap())
    }
}
