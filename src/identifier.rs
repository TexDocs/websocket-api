pub const PROTOCOL_VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub const HANDSHAKE: u8         = 0;
pub const HANDSHAKE_ACK: u8     = 1;
pub const HANDSHAKE_ERR: u8     = 2;
pub const PROJECT_REQUEST: u8   = 3;
pub const PROJECT: u8           = 4;
