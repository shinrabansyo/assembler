#[derive(Debug)]
pub struct Data {
    pub label: Option<String>,
    pub command: Command,
}

#[derive(Debug)]
pub enum Command {
    Byte1(u8),
    Byte2(u16),
    Byte4(u32),
    Byte6(u64),
    Char(char),
    String(String),
}

impl Command {
    pub fn len(&self) -> usize {
        match self {
            Command::Byte1(_) => 1,
            Command::Byte2(_) => 2,
            Command::Byte4(_) => 4,
            Command::Byte6(_) => 6,
            Command::Char(_) => 1,
            Command::String(s) => s.as_bytes().len() + 1,
        }
    }
}
