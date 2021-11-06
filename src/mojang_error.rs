#[derive(Debug, Clone, Copy)]
pub enum MojangError {
    NoNameOrUUID,

    RequestError,
    ParseError,
}
