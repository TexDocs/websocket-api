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

pub mod serialize;
pub mod identifier;
pub mod project;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
