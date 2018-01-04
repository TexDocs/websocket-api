use rmps::{ to_vec, from_slice };
use rmps::decode::Error;
use serde::{ Serialize, Deserialize };
use stdweb::unstable::TryInto;
use stdweb::Value;
use std::fmt::Debug;

pub fn serialize<T: Serialize>(to_serialize: &T, identifier: u8) -> Vec<u8> {
    let mut vec = to_vec(to_serialize).unwrap();
    vec.push(identifier);
    vec
}

pub fn deserialize<'a, T: Deserialize<'a> + TryInto<Value>>(data: &'a Vec<u8>) -> Result<T, Error> {
    from_slice(data)
}


pub fn deserialize_to_js<'a, T: Deserialize<'a> + TryInto<Value>>(data: &'a Vec<u8>) -> Value where T::Error: Debug {
    let deserialized: T = deserialize(data).unwrap();
    deserialized.try_into().unwrap()
}
