use crate::{Base58PublicKey, ProgramLogEntry};
use wasmium_errors::WasmiumError;

pub type PoseidonResult<T> = core::result::Result<T, PoseidonError>;

#[derive(Debug)]
pub enum PoseidonError {
    /// Error deserializing the TOML file to the specified data structure
    #[cfg(feature = "smol_async_io")]
    TomlDeserError(String),
    /// An I/O error has occured
    IoError(std::io::ErrorKind),
    /// A HTTP error occured
    HTTPError(MinreqErrors),
    /// Unable to decode public key from provided base58 String
    InvalidBase58ForPublickKey,
    /// Unable to convert the provided data into a `[u8; 32]`
    ErrorConvertingToU832,
    SerdeJsonDeser(String),
    ProgramIdNotFound,
    PublicKeyNotFound,
    PublicKeyNotFoundInMessageAccounts,
    AccountIndexNotFoundInMessageAccounts,
    MaxSeedLengthExceeded,
    IllegalOwner,
    BorshSerDeError(String),
    WasmiumErrors(WasmiumError),
    BincodeError(bincode::ErrorKind),
    ParsedRpcResponseError {
        jsonrpc: String,
        id: u8,
        json_error_code: i16,
        message: String,
        error: RpcResponseError,
        accounts: Vec<Base58PublicKey>,
        logs: Vec<ProgramLogEntry>,
    },
}

impl From<bincode::Error> for PoseidonError {
    fn from(error: bincode::Error) -> Self {
        PoseidonError::BincodeError(*error)
    }
}

impl From<WasmiumError> for PoseidonError {
    fn from(error: WasmiumError) -> Self {
        PoseidonError::WasmiumErrors(error)
    }
}

#[cfg(feature = "smol_async_io")]
impl From<toml::de::Error> for PoseidonError {
    fn from(error: toml::de::Error) -> Self {
        PoseidonError::TomlDeserError(format!("{:?}", error))
    }
}

impl From<std::io::Error> for PoseidonError {
    fn from(io_error: std::io::Error) -> Self {
        PoseidonError::IoError(io_error.kind())
    }
}

impl From<serde_json::Error> for PoseidonError {
    fn from(error: serde_json::Error) -> Self {
        PoseidonError::SerdeJsonDeser(error.to_string())
    }
}

impl From<minreq::Error> for PoseidonError {
    fn from(minreq_error: minreq::Error) -> Self {
        PoseidonError::HTTPError(match minreq_error {
            minreq::Error::InvalidUtf8InBody(utf8_error) => {
                MinreqErrors::InvalidUtf8InBody(utf8_error)
            }
            minreq::Error::RustlsCreateConnection(rustls_error) => {
                MinreqErrors::RustlsCreateConnection(rustls_error.to_string())
            }
            minreq::Error::IoError(io_error) => MinreqErrors::IoError(io_error.kind()),
            minreq::Error::MalformedChunkLength => MinreqErrors::MalformedChunkLength,
            minreq::Error::MalformedChunkEnd => MinreqErrors::MalformedChunkEnd,
            minreq::Error::MalformedContentLength => MinreqErrors::MalformedContentLength,
            minreq::Error::HeadersOverflow => MinreqErrors::HeadersOverflow,
            minreq::Error::StatusLineOverflow => MinreqErrors::StatusLineOverflow,
            minreq::Error::AddressNotFound => MinreqErrors::AddressNotFound,
            minreq::Error::RedirectLocationMissing => MinreqErrors::RedirectLocationMissing,
            minreq::Error::InfiniteRedirectionLoop => MinreqErrors::InfiniteRedirectionLoop,
            minreq::Error::TooManyRedirections => MinreqErrors::TooManyRedirections,
            minreq::Error::InvalidUtf8InResponse => MinreqErrors::InvalidUtf8InResponse,
            minreq::Error::PunycodeConversionFailed => MinreqErrors::PunycodeConversionFailed,
            minreq::Error::HttpsFeatureNotEnabled => MinreqErrors::HttpsFeatureNotEnabled,
            minreq::Error::PunycodeFeatureNotEnabled => MinreqErrors::PunycodeFeatureNotEnabled,
            minreq::Error::BadProxy => MinreqErrors::BadProxy,
            minreq::Error::BadProxyCreds => MinreqErrors::BadProxyCreds,
            minreq::Error::ProxyConnect => MinreqErrors::ProxyConnect,
            minreq::Error::InvalidProxyCreds => MinreqErrors::InvalidProxyCreds,
            minreq::Error::Other(other_error) => MinreqErrors::Other(other_error),
        })
    }
}

/// Errors from the minreq crate
/// Manual implementation provides Comparison and Clone operations
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum MinreqErrors {
    /// The response body contains invalid UTF-8, so the `as_str()`
    /// conversion failed.
    InvalidUtf8InBody(core::str::Utf8Error),
    /// Ran into a rustls error while creating the connection.
    RustlsCreateConnection(String),
    /// Ran into an IO problem while loading the response.
    IoError(std::io::ErrorKind),
    /// Couldn't parse the incoming chunk's length while receiving a
    /// response with the header `Transfer-Encoding: chunked`.
    MalformedChunkLength,
    /// The chunk did not end after reading the previously read amount
    /// of bytes.
    MalformedChunkEnd,
    /// Couldn't parse the `Content-Length` header's value as an
    /// `usize`.
    MalformedContentLength,
    /// The response contains headers whose total size surpasses
    HeadersOverflow,
    /// The response's status line length surpasses
    StatusLineOverflow,
    /// [ToSocketAddrs](std::net::ToSocketAddrs) did not resolve to an
    /// address.
    AddressNotFound,
    /// The response was a redirection, but the `Location` header is
    /// missing.
    RedirectLocationMissing,
    /// The response redirections caused an infinite redirection loop.
    InfiniteRedirectionLoop,
    /// Redirections, won't follow any more.
    TooManyRedirections,
    /// The response contained invalid UTF-8 where it should be valid
    /// (eg. headers), so the response cannot interpreted correctly.
    InvalidUtf8InResponse,
    /// The provided url contained a domain that has non-ASCII
    /// characters, and could not be converted into punycode. It is
    /// probably not an actual domain.
    PunycodeConversionFailed,
    /// Tried to send a secure request (ie. the url started with
    /// `https://`), but the crate's `https` feature was not enabled,
    /// and as such, a connection cannot be made.
    HttpsFeatureNotEnabled,
    /// The provided url contained a domain that has non-ASCII
    /// characters, but it could not be converted into punycode
    /// because the `punycode` feature was not enabled.
    PunycodeFeatureNotEnabled,
    /// The provided proxy information was not properly formatted.
    /// Supported proxy format is `[user:password@]host:port`.
    BadProxy,
    /// The provided credentials were rejected by the proxy server.
    BadProxyCreds,
    /// The provided proxy credentials were malformed.
    ProxyConnect,
    /// The provided credentials were rejected by the proxy server.
    InvalidProxyCreds,

    /// This is a special error case, one that should never be
    /// returned! Think of this as a cleaner alternative to calling
    /// `unreachable!()` inside the library. If you come across this,
    /// please open an issue in the minreq crate repository, and include the string inside this
    /// error, as it can be used to locate the problem.
    Other(&'static str),
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq)]
pub enum RpcResponseError {
    CreateAccountWithSeedError,
    Unspecified,
}
