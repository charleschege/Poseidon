use crate::{Base58PublicKey, PoseidonValue, ProgramLogEntry};
use serde::{Deserialize, Serialize};
use serde_json::value::Value;
use std::collections::HashMap;
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
    Json(json::JsonError),
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
    ParsedRpcResponseError(RpcResponseError),
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RpcResponseError {
    jsonrpc: String,
    id: u8,
    error_code: i16,
    message: String,
    accounts: Vec<Base58PublicKey>,
    logs: Vec<ProgramLogEntry>,
    instruction_error: u32,
    custom_errors: HashMap<String, PoseidonValue>,
    units_consumed: u64,
}

impl RpcResponseError {
    pub fn new() -> Self {
        RpcResponseError {
            jsonrpc: String::default(),
            id: u8::default(),
            error_code: i16::default(),
            message: String::default(),
            accounts: Vec::default(),
            logs: Vec::default(),
            instruction_error: u32::default(),
            custom_errors: HashMap::default(),
            units_consumed: u64::default(),
        }
    }
    pub fn transform(&mut self, error: RpcErrorHTTP) -> &mut RpcResponseError {
        let accounts = match error.error.data.accounts {
            Some(collection) => collection,
            None => Vec::default(),
        };

        let instruction_error = match &error.error.data.err.instruction_error[0] {
            Value::Number(value) => match value.as_u64() {
                Some(value) => value as u32,
                None => 0, //TODO Log All Errors as part of telemetry
            },
            _ => 0,
        };

        let raw_custom_error = &error.error.data.err.instruction_error[1];

        match raw_custom_error {
            Value::Object(values) => {
                values.iter().for_each(|(key, value)| {
                    let poseidon_value = match value {
                        Value::Number(inner_value) => match inner_value.as_u64() {
                            None => PoseidonValue::InvalidValue(value.to_string()),
                            Some(data) => PoseidonValue::Number(data as u8),
                        },

                        Value::String(inner_value) => PoseidonValue::String(inner_value.to_owned()),

                        _ => PoseidonValue::InvalidValue(value.to_string()),
                    };
                    self.custom_errors.insert(key.to_owned(), poseidon_value);
                });
            }
            _ => {
                self.custom_errors.insert(
                    "InvalidCustomError".to_owned(),
                    PoseidonValue::InvalidValue(raw_custom_error.to_string()),
                );
            }
        }

        self.jsonrpc = error.jsonrpc;
        self.id = error.id;
        self.error_code = error.error.code;
        self.message = error.error.message;
        self.accounts = accounts;
        self.logs = error.error.data.logs;
        self.instruction_error = instruction_error;
        self
    }
}

impl From<RpcResponseError> for PoseidonError {
    fn from(error: RpcResponseError) -> Self {
        PoseidonError::ParsedRpcResponseError(error)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct RpcErrorHTTP {
    jsonrpc: String,
    id: u8,
    error: RpcReponseError,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RpcReponseError {
    code: i16,
    message: String,
    data: RpcErrorData,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RpcErrorData {
    accounts: Option<Vec<String>>,
    logs: Vec<String>,
    #[serde(rename = "unitsConsumed")]
    units_consumed: u64,
    err: InstructionError,
}

#[derive(Debug, Clone, Deserialize, Default, Serialize)]

pub struct InstructionError {
    #[serde(rename = "InstructionError")]
    instruction_error: Vec<Value>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialOrd, Ord, PartialEq, Serialize)]
pub enum ProgramError {
    /// Allows on-chain programs to implement program-specific error types and see them returned
    /// by the Solana runtime. A program-specific error may be any type that is represented as
    /// or serialized to a u32 integer.
    Custom(u32),
    /// The arguments provided to a program instruction where invalid
    InvalidArgument,
    /// An instruction's data contents was invalid
    InvalidInstructionData,
    /// An account's data contents was invalid
    InvalidAccountData,
    /// An account's data was too small
    AccountDataTooSmall,
    /// An account's balance was too small to complete the instruction
    InsufficientFunds,
    /// The account did not have the expected program id
    IncorrectProgramId,
    /// A signature was required but not found
    MissingRequiredSignature,
    /// An initialize instruction was sent to an account that has already been initialized
    AccountAlreadyInitialized,
    /// An attempt to operate on an account that hasn't been initialized
    UninitializedAccount,
    /// The instruction expected additional account keys
    NotEnoughAccountKeys,
    /// Failed to borrow a reference to account data, already borrowed
    AccountBorrowFailed,
    /// Length of the seed is too long for address generation
    MaxSeedLengthExceeded,
    /// Provided seeds do not result in a valid address
    InvalidSeeds,
    /// IO Error
    BorshIoError(String),
    /// An account does not have enough lamports to be rent-exempt
    AccountNotRentExempt,
    /// Unsupported sysvar
    UnsupportedSysvar,
    /// Provided owner is not allowed
    IllegalOwner,
    /// Requested account data allocation exceeded the accounts data budget
    AccountsDataBudgetExceeded,
    /// Could not parse the custom error into a `u32`
    ErrorParsingCusomErrorToU32(String),
    /// The error could not be parsed properly
    UnspecifiedError,
}

impl From<&str> for ProgramError {
    fn from(error: &str) -> Self {
        match error {
            "InvalidArgument" => ProgramError::InvalidArgument,
            "InvalidInstructionData" => ProgramError::InvalidInstructionData,
            "InvalidAccountData" => ProgramError::InvalidAccountData,
            "AccountDataTooSmall" => ProgramError::AccountDataTooSmall,
            "InsufficientFunds" => ProgramError::InsufficientFunds,
            "IncorrectProgramId" => ProgramError::IncorrectProgramId,
            "MissingRequiredSignature" => ProgramError::MissingRequiredSignature,
            "AccountAlreadyInitialized" => ProgramError::AccountAlreadyInitialized,
            "UninitializedAccount" => ProgramError::UninitializedAccount,
            "NotEnoughAccountKeys" => ProgramError::NotEnoughAccountKeys,
            "AccountBorrowFailed" => ProgramError::AccountBorrowFailed,
            "MaxSeedLengthExceeded" => ProgramError::MaxSeedLengthExceeded,
            "InvalidSeeds" => ProgramError::InvalidSeeds,
            "AccountNotRentExempt" => ProgramError::AccountNotRentExempt,
            "UnsupportedSysvar" => ProgramError::UnsupportedSysvar,
            "IllegalOwner" => ProgramError::IllegalOwner,
            "AccountsDataBudgetExceeded" => ProgramError::AccountsDataBudgetExceeded,
            _error if error.contains("Custom(") => {
                let chunks = error.split("(").collect::<Vec<&str>>();

                let error_code = chunks[1].replace(")", "");
                let error_code = error_code.trim_start_matches("0x");

                match error_code.parse::<u32>() {
                    Ok(custom_code) => ProgramError::Custom(custom_code),
                    Err(error) => ProgramError::ErrorParsingCusomErrorToU32(error.to_string()),
                }
            }
            _error if error.contains("BorshIoError") => {
                // FIXME Check if escaped string is a problem
                let chunks = error.split("(").collect::<Vec<&str>>();

                let error_code = chunks[1].replace(")", "");
                let error_code = error_code.replace("\"", "");

                ProgramError::BorshIoError(error_code)
            }
            _ => ProgramError::UnspecifiedError,
        }
    }
}

impl Default for ProgramError {
    fn default() -> Self {
        ProgramError::UnspecifiedError
    }
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

impl From<json::JsonError> for PoseidonError {
    fn from(error: json::JsonError) -> Self {
        PoseidonError::Json(error)
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
