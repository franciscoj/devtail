pub mod cmd;
pub mod entry;
pub mod log;
pub mod parser;
pub mod tail;

#[derive(Debug, PartialEq)]
// TODO: Map HTTP states from 100..=199
pub enum HttpStatus {
    /// HTTP States from `200..=299`
    Success(u16),
    /// HTTP States from `300..=399`
    Redirect(u16),
    /// HTTP States from `400..=499`
    ClientError(u16),
    /// HTTP States from `500..=599`
    ServerError(u16),
    /// To have the same "ducktype" as the rest of variants it has an u16 attached but it is set to
    /// 0 by default.
    ///
    /// In case it receives an error < 200 or > 600 the value comes here.
    Unknown(u16),
}
