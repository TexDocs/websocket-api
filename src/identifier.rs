pub const PROTOCOL_VERSION: &'static str = env!("CARGO_PKG_VERSION");

// --- Structure ---
// 0bXXXXX___ = Group ID e.g. Handshake/Project/File
// 0b_____111 = Request
// 0b_____0XX = Response
// 0b_____1XX = Error (exclusive 0b_____111)

// Handshake
pub const HANDSHAKE: u8 = 0b00000_111;
pub const HANDSHAKE_ACK: u8 = 0b00000_000;
pub const HANDSHAKE_ERR: u8 = 0b00000_100;

// Project
pub const PROJECT_REQUEST: u8 = 0b00001_111;
pub const PROJECT: u8 = 0b00001_000;
pub const FILE_TREE: u8 = 0b00001_001;
pub const PROJECT_REQUEST_ERR: u8 = 0b00001_100;

// File
pub const FILE_REQUEST: u8 = 0b00010_111;
pub const FILE: u8 = 0b00010_000;
pub const FILE_REQUEST_ERR: u8 = 0b00010_100;

// User management
pub const USER_JOINED: u8 = 0b00011_000;
pub const USER_LEFT: u8 = 0b00011_001;
