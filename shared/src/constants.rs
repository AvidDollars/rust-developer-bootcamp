pub const IMAGES_FOLDER: &str = "./client/images/";
pub const FILES_FOLDER: &str = "./client/files/";
pub const LOGS_FOLDER: &str = "./client/logs/";

pub const DEFAULT_HOST: [u8; 4] = [127, 0, 0, 1];
pub const DEFAULT_PORT: u16 = 11111;

// Buffer on client's side for receiving messages from server
pub const CLIENT_MSG_BUFFER_SIZE: usize = 65_536;
