pub const PROTOCOL_VERSION: &'static str = env!("CARGO_PKG_VERSION");

// Handshake
pub const HANDSHAKE: u8             = 0b00000_111;
pub const HANDSHAKE_ACK: u8         = 0b00000_000;
pub const HANDSHAKE_ERR: u8         = 0b00000_100;

// Project
pub const PROJECT_REQUEST: u8       = 0b00001_111;
pub const PROJECT: u8               = 0b00001_000;
pub const FILE_TREE: u8             = 0b00001_001;
pub const PROJECT_REQUEST_ERR: u8   = 0b00001_100;

// File
pub const FILE_REQUEST: u8          = 0b00010_111;
pub const FILE: u8                  = 0b00010_000;
pub const FILE_REQUEST_ERR: u8      = 0b00010_100;
