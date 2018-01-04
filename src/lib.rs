#![feature(try_from)]
#![feature(underscore_lifetimes)]

#[cfg(feature = "wasm")]
#[macro_use]
extern crate stdweb;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate rmp_serde as rmps;
extern crate uuid;

mod serialize;
pub mod identifier;
pub mod project;

// fn parse_msg(data: Vec<u8>) -> Value {
//     let id = data.pop().unwrap();
//
//     match id {
//         identifier::PROJECT => deserialize::<Project>(&data),
//         identifier::HANDSHAKE_ACK => deserialize::<HandshakeAcknowledgement>(&data),
//         identifier::HANDSHAKE_ERR => deserialize::<HandshakeError>(&data),
//         _ => Null.try_into().unwrap()
//     }
// }

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
