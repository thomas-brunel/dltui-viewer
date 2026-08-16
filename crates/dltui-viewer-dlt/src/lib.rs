pub mod dlt_file;

#[derive(Debug)]
pub enum Error {
    MissingFileExtension,
    UnsupportedExtension,
    MissingStorageTimestampError,
    Utf8DecodeError(std::str::Utf8Error),
    DecodeError(dlt_parse::error::VerboseDecodeError),
}

impl From<dlt_parse::error::VerboseDecodeError> for Error {
    fn from(value: dlt_parse::error::VerboseDecodeError) -> Self {
        Self::DecodeError(value)
    }
}

impl From<std::str::Utf8Error> for Error {
    fn from(value: std::str::Utf8Error) -> Self {
        Self::Utf8DecodeError(value)
    }
}
