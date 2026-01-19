pub const LOGIN: u8 = 0x01;
pub const ALERT: u8 = 0x02;
pub const PING: u8 = 0x03;

pub enum Message {
    Login(String),
    Alert { code: u8, severity: u8 },
    Ping,
}
