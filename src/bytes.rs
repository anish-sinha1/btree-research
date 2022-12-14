use serde::Serialize;
use serde::de::{DeserializeOwned};

pub fn encode<T>(item: T) -> Option<Vec<u8>>
    where
        T: Sized + Serialize,
{
    let encoded = bincode::serialize(&item);
    match encoded {
        Ok(encoded) => Some(encoded),
        Err(_) => None,
    }
}

pub fn decode<T>(bytes: Vec<u8>) -> Option<T>
    where
        T: Sized + Serialize + DeserializeOwned,
{
    let decoded = bincode::deserialize(&bytes[..]);
    match decoded {
        Ok(decoded) => Some(decoded),
        Err(_) => None,
    }
}