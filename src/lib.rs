#![feature(try_from)]
#![feature(underscore_lifetimes)]

extern crate rmp_serde as rmps;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[cfg(feature = "wasm")]
#[macro_use]
extern crate stdweb;
extern crate uuid;

pub use serde::Deserialize;

pub mod serialize;
pub mod identifier;
pub mod handshake;
pub mod project;
pub mod user;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
