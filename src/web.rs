#![feature(try_from)]

#[macro_use]
extern crate stdweb;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate rmp_serde as rmps;
extern crate uuid;

mod project;

use stdweb::unstable::TryInto;
use stdweb::{Value, Null};
use project::*;
use uuid::Uuid;

fn request_project(id: Value) -> Value {
    let id: String = id.try_into().unwrap();
    let parsed_id = Uuid::parse_str(&id).unwrap();

    ProjectRequest::new(parsed_id).try_into().unwrap()

    // let mut data = rmps::to_vec(&Project::mock()).unwrap();
    // data.push(PROJECT_IDENTIFIER);
    // data.try_into().unwrap()
}

fn parse_msg(msg: Value) -> Value {
    let mut data: Vec<u8> = msg.try_into().unwrap();
    let id = data.pop().unwrap();

    match id {
        PROJECT_IDENTIFIER => {
            let prj: Project = rmps::from_slice(&data).unwrap();
            prj.try_into().unwrap()
        },
        // PROJECT_REQUEST_IDENTIFIER => {
        //
        // },
        _ => Null.try_into().unwrap()
    }
}

fn main() {
    stdweb::initialize();

    js! {
        Module.exports.requestProject = @{request_project};
        Module.exports.parseMessage = @{parse_msg};
    }
}
