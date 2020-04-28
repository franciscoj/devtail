pub mod tail;
pub mod log;
pub mod parser;
pub mod entry;

#[derive(Debug, PartialEq)]
pub enum HttpStatus {
    Success(u16),
    Redirect(u16),
    ClientError(u16),
    ServerError(u16),
    Unknown(u16)
}

