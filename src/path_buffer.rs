use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};
use std::{ffi::OsStr, fmt, path::PathBuf, rc::Rc};

/// A wrapper around a reference counted PathBuf for extended implementation
#[derive(Debug)]
pub struct PathBuffer(Rc<PathBuf>);

impl PathBuffer {
    pub fn to_str(&self) -> Option<&str> {
        self.0.to_str()
    }

    pub fn as_os_str(&self) -> &OsStr {
        self.0.as_os_str()
    }
}

impl From<&str> for PathBuffer {
    fn from(value: &str) -> Self {
        PathBuffer(Rc::new(PathBuf::from(value)))
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

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(PathBuffer::from(value))
    }
}

impl<'de> Deserialize<'de> for PathBuffer {
    fn deserialize<D>(deserializer: D) -> Result<PathBuffer, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(PathBufferVisitor)
    }
}

// TODO: Implement proper error handling for custom serializers
impl Serialize for PathBuffer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_str().unwrap_or("FAILED"))

        // match self.to_str() {
        //     Some(val) => serializer.serialize_str(val),
        //     None => SerdeError::Serialize("{:?}", *self.0),
        // }
    }
}
