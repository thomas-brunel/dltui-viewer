pub mod dlt_file;

#[derive(Debug)]
pub enum Error {
    MissingFileExtension,
    UnsupportedExtension,
    MissingStorageTimestampError,
    Utf8DecodeError(std::str::Utf8Error),
    DecodeError(dlt_parse::error::VerboseDecodeError),
    FormatError(std::fmt::Error),
    IoError(std::io::Error),
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

impl From<std::fmt::Error> for Error {
    fn from(value: std::fmt::Error) -> Self {
        Self::FormatError(value)
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::IoError(value)
    }
}
