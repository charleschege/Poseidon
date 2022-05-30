use borsh::{BorshDeserialize, BorshSerialize};
use core::fmt;
use serde::{Deserialize, Serialize};

pub type PoseidonResult<T> = Result<T, PoseidonError>;

#[derive(Debug)]
pub enum PoseidonError {
    Io(std::io::ErrorKind),
    /// Errors from the minreq crate
    Http(Minreq),
    /// Errors from the bincode crate
    BincodeError(bincode::ErrorKind),
    /// Errors encountered when using serde_json crate to deserialize
    SerdeJsonDeser(String),
    /// The maximum length of the `seed` provided has been exceeded
    /// as indicated by `[MAX_SEED_LEN]`
    MaxSeedLengthExceeded,
    /// The `owner` public key of the PDA provided is the same as the
    /// `[PDA_MARKER]` address. This is not allowed.
    IllegalOwner,
    /// The error that occured hasn't been encountered before
    UnspecifiedError,
    /// The string provided is not valid for the Base58 format.
    InvalidBase58ForPublicKey,
    /// Unable to convert a `slice` to an array of 32 bytes (`[u8; 32]`).
    ErrorConvertingToU832,
    /// The program ID was not found in the provided instruction
    ProgramIdNotFound,
    /// The public key was not found in the accounts found in the `Message`
    PublicKeyNotFoundInMessageAccounts,
    /// The account index was not found in the `Accounts`
    AccountIndexNotFoundInMessageAccounts,
    /// Error decoding string as Base58 format
    Bs58Decode(bs58::decode::Error),
    /// Error encoding to base58 format
    Bs58Encode(bs58::encode::Error),
    /// The transaction was not found in the Cluster
    TransactionNotFoundInCluster,
}

impl std::error::Error for PoseidonError {}

impl fmt::Display for PoseidonError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<bs58::encode::Error> for PoseidonError {
    fn from(error: bs58::encode::Error) -> Self {
        PoseidonError::Bs58Encode(error)
    }
}

impl From<bs58::decode::Error> for PoseidonError {
    fn from(error: bs58::decode::Error) -> Self {
        PoseidonError::Bs58Decode(error)
    }
}

impl From<bincode::Error> for PoseidonError {
    fn from(error: bincode::Error) -> Self {
        PoseidonError::BincodeError(*error)
    }
}

impl From<std::io::Error> for PoseidonError {
    fn from(io_error: std::io::Error) -> Self {
        PoseidonError::Io(io_error.kind())
    }
}

impl From<serde_json::Error> for PoseidonError {
    fn from(error: serde_json::Error) -> Self {
        PoseidonError::SerdeJsonDeser(error.to_string())
    }
}

impl From<minreq::Error> for PoseidonError {
    fn from(minreq_error: minreq::Error) -> Self {
        PoseidonError::Http(match minreq_error {
            minreq::Error::InvalidUtf8InBody(utf8_error) => Minreq::InvalidUtf8InBody(utf8_error),
            minreq::Error::RustlsCreateConnection(rustls_error) => {
                Minreq::RustlsCreateConnection(rustls_error.to_string())
            }
            minreq::Error::IoError(io_error) => Minreq::Io(io_error.kind()),
            minreq::Error::MalformedChunkLength => Minreq::MalformedChunkLength,
            minreq::Error::MalformedChunkEnd => Minreq::MalformedChunkEnd,
            minreq::Error::MalformedContentLength => Minreq::MalformedContentLength,
            minreq::Error::HeadersOverflow => Minreq::HeadersOverflow,
            minreq::Error::StatusLineOverflow => Minreq::StatusLineOverflow,
            minreq::Error::AddressNotFound => Minreq::AddressNotFound,
            minreq::Error::RedirectLocationMissing => Minreq::RedirectLocationMissing,
            minreq::Error::InfiniteRedirectionLoop => Minreq::InfiniteRedirectionLoop,
            minreq::Error::TooManyRedirections => Minreq::TooManyRedirections,
            minreq::Error::InvalidUtf8InResponse => Minreq::InvalidUtf8InResponse,
            minreq::Error::PunycodeConversionFailed => Minreq::PunycodeConversionFailed,
            minreq::Error::HttpsFeatureNotEnabled => Minreq::HttpsFeatureNotEnabled,
            minreq::Error::PunycodeFeatureNotEnabled => Minreq::PunycodeFeatureNotEnabled,
            minreq::Error::BadProxy => Minreq::BadProxy,
            minreq::Error::BadProxyCreds => Minreq::BadProxyCreds,
            minreq::Error::ProxyConnect => Minreq::ProxyConnect,
            minreq::Error::InvalidProxyCreds => Minreq::InvalidProxyCreds,
            minreq::Error::Other(other_error) => Minreq::Other(other_error),
        })
    }
}

/// Errors from the minreq crate
/// Manual implementation provides Comparison and Clone operations
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Minreq {
    /// The response body contains invalid UTF-8, so the `as_str()`
    /// conversion failed.
    InvalidUtf8InBody(core::str::Utf8Error),
    /// Ran into a rustls error while creating the connection.
    RustlsCreateConnection(String),
    /// Ran into an IO problem while loading the response.
    Io(std::io::ErrorKind),
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

#[derive(
    Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize, PartialEq, PartialOrd, Clone,
)]
pub struct RpcResponseJsonError {
    jsonrpc: String,
    error: JsonRpcError,
    id: u8,
}

#[derive(
    Debug, Serialize, Deserialize, BorshSerialize, BorshDeserialize, PartialEq, PartialOrd, Clone,
)]
pub struct JsonRpcError {
    code: i16,
    message: String,
}
