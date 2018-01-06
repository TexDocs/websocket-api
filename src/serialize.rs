use rmps::{from_slice, to_vec};
use rmps::decode::Error;
use serde::{Deserialize, Serialize};
#[cfg(feature = "wasm")]
use stdweb::unstable::TryInto;
#[cfg(feature = "wasm")]
use stdweb::Value;
#[cfg(feature = "wasm")]
use std::fmt::Debug;

pub fn serialize<T: Serialize>(to_serialize: &T, identifier: u8) -> Vec<u8> {
    let mut vec = to_vec(to_serialize).unwrap();
    vec.push(identifier);
    vec
}

pub fn deserialize<'a, T: Deserialize<'a>>(data: &'a Vec<u8>) -> Result<T, Error> {
    from_slice(data)
}

#[cfg(feature = "wasm")]
pub fn deserialize_to_js<'a, T: Deserialize<'a> + TryInto<Value>>(
    data: &'a Vec<u8>,
    identifier: String,
) -> Value
where
    T::Error: Debug,
{
    let deserialized: T = deserialize(data).unwrap();
    let value: Value = deserialized.try_into().unwrap();
    vec![identifier.try_into().unwrap(), value]
        .try_into()
        .unwrap()
}
