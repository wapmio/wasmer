#[cfg(unix)]
mod unix;

#[cfg(windows)]
mod windows;

#[cfg(unix)]
pub use self::unix::*;

#[cfg(windows)]
pub use self::windows::*;

#[cfg(feature = "cache")]
use serde::{
    de::{self, SeqAccess, Visitor},
    ser::SerializeStruct,
    Deserialize, Deserializer, Serialize, Serializer,
};
#[cfg(feature = "cache")]
use serde_bytes::Bytes;
#[cfg(feature = "cache")]
use std::fmt;

#[cfg(feature = "cache")]
impl Serialize for Memory {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        assert!(self.protection().is_readable());

        let mut state = serializer.serialize_struct("Memory", 2)?;
        state.serialize_field("protection", &self.protection())?;
        state.serialize_field("data", &Bytes::new(unsafe { self.as_slice() }))?;
        state.end()
    }
}

#[cfg(feature = "cache")]
impl<'de> Deserialize<'de> for Memory {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MemoryVisitor;

        impl<'de> Visitor<'de> for MemoryVisitor {
            type Value = Memory;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "struct Memory")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Memory, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let original_protection = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;

                let bytes: Bytes = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;

                #[cfg(not(target_os = "windows"))]
                let mut memory = Memory::with_size_protect(bytes.len(), Protect::ReadWrite)
                    .expect("Could not create a memory");

                #[cfg(target_os = "windows")]
                let mut memory = Memory::with_size(bytes.len()).expect("Could not create a memory");

                unsafe {
                    memory.as_slice_mut().copy_from_slice(&*bytes);

                    if memory.protection() != original_protection {
                        memory
                            .protect(.., original_protection)
                            .expect("Could not protect memory as its original protection");
                    }
                }

                Ok(memory)
            }
        }

        deserializer.deserialize_struct("Memory", &["protection", "data"], MemoryVisitor)
    }
}
