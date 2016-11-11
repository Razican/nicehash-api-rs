//! [nicehash.com](https://www.nicehash.com/) API implementation for Rust.

#[macro_use]
extern crate lazy_static;
extern crate hyper;
extern crate semver;
extern crate serde_json;
extern crate serde;

use std::ops::Deref;

use semver::Version;
use serde_json::value::Value;
use serde_json::de;

pub mod error;
pub mod types;
mod public;
mod private;

use error::{Result, Error};

/// Order fee, in BTCs.
pub const ORDER_FEE: f64 = 0.0001;
/// Service fee, in percent of order expenditure.
pub const SERVICE_FEE: f64 = 0.03;

const API_URL: &'static str = "https://www.nicehash.com/api";

/// [nicehash.com](https://www.nicehash.com/) API client.
#[derive(Debug)]
pub struct Client {
    inner: hyper::Client,
    remote_version: Version,
}

impl Client {
    /// Creates a new API client with default settings.
    ///
    /// It will also call the API to check that the connection works and to store the remote API
    /// version.
    pub fn new() -> Result<Client> {
        Client::from_hyper_client(hyper::Client::new())
    }

    /// Creates a new API client from a Hyper client.
    pub fn from_hyper_client(hyper_client: hyper::Client) -> Result<Client> {
        let version = Client::remote_version(&hyper_client)?;
        Ok(Client {
            inner: hyper_client,
            remote_version: version,
        })
    }

    /// Gets the version of the remote API.
    ///
    /// This only returns the version stored in the client, it will not update the version if remote
    /// API is updated while the client has been created.
    pub fn get_api_version(&self) -> &Version {
        &self.remote_version
    }

    /// Updates the API version of this `Client`.
    ///
    /// This will check the current version of the remote server and store it in the `Client`
    /// object for later use.
    pub fn update_api_version(&mut self) -> Result<&Version> {
        let version = Client::remote_version(&self.inner)?;
        self.remote_version = version;
        Ok(&self.remote_version)
    }

    /// Gets the version of the remote API.
    fn remote_version(hyper_client: &hyper::Client) -> Result<Version> {
        let response = hyper_client.get(API_URL).send()?;
        let response_json: Value = de::from_reader(response)?;
        if let Some(&Value::String(ref version)) =
               response_json.find_path(&["result", "api_version"]) {
            Ok(Version::parse(version)?)
        } else {
            Err(Error::Api("the api returned an invalid response for the version request"
                .to_owned()))
        }
    }
}

impl Deref for Client {
    type Target = hyper::Client;

    fn deref(&self) -> &hyper::Client {
        &self.inner
    }
}

pub fn calculate_withdrawal_fee(amount: f64) -> f64 {
    if amount > 0.5 { amount * 0.001 } else { 0.0005 }
}

/// Enum representing the location of the servers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Location {
    Europe = 0,
    USA = 1,
}

impl Location {
    /// Gets an `&str` representing the `u64` of the server location.
    fn as_str(&self) -> &str {
        match *self {
            Location::Europe => "0",
            Location::USA => "1",
        }
    }
}

/// Enum representing all algorithms in nicehash.com.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderType {
    Standard = 0,
    Fixed = 1,
}

impl OrderType {
    /// Creates an `OrderType` from a `u64`.
    fn from_u64(val: u64) -> Result<OrderType> {
        match val {
            0 => Ok(OrderType::Standard),
            1 => Ok(OrderType::Fixed),
            t => Err(Error::ParseOrderType(format!("invalid order type {}", t))),
        }
    }
}

/// Enum representing all algorithms in nicehash.com.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Algorithm {
    Scrypt = 0,
    SHA256 = 1,
    ScryptNf = 2,
    X11 = 3,
    X13 = 4,
    Keccak = 5,
    X15 = 6,
    Nist5 = 7,
    NeoScrypt = 8,
    Lyra2RE = 9,
    WhirlpoolX = 10,
    Qubit = 11,
    Quark = 12,
    Axiom = 13,
    Lyra2REv2 = 14,
    ScryptJaneNf16 = 15,
    Blake256r8 = 16,
    Blake256r14 = 17,
    Blake256r8vnl = 18,
    Hodl = 19,
    DaggerHashimoto = 20,
    Decred = 21,
    CryptoNight = 22,
    Lbry = 23,
    Equihash = 24,
}

impl Algorithm {
    /// Creates an `Algorithm` from a `u64`.
    fn from_u64(alg: u64) -> Result<Algorithm> {
        match alg {
            0 => Ok(Algorithm::Scrypt),
            1 => Ok(Algorithm::SHA256),
            2 => Ok(Algorithm::ScryptNf),
            3 => Ok(Algorithm::X11),
            4 => Ok(Algorithm::X13),
            5 => Ok(Algorithm::Keccak),
            6 => Ok(Algorithm::X15),
            7 => Ok(Algorithm::Nist5),
            8 => Ok(Algorithm::NeoScrypt),
            9 => Ok(Algorithm::Lyra2RE),
            10 => Ok(Algorithm::WhirlpoolX),
            11 => Ok(Algorithm::Qubit),
            12 => Ok(Algorithm::Quark),
            13 => Ok(Algorithm::Axiom),
            14 => Ok(Algorithm::Lyra2REv2),
            15 => Ok(Algorithm::ScryptJaneNf16),
            16 => Ok(Algorithm::Blake256r8),
            17 => Ok(Algorithm::Blake256r14),
            18 => Ok(Algorithm::Blake256r8vnl),
            19 => Ok(Algorithm::Hodl),
            20 => Ok(Algorithm::DaggerHashimoto),
            21 => Ok(Algorithm::Decred),
            22 => Ok(Algorithm::CryptoNight),
            23 => Ok(Algorithm::Lbry),
            24 => Ok(Algorithm::Equihash),
            a => Err(Error::ParseAlgorithm(format!("invalid algorithm number {}", a))),
        }
    }

    /// Gets an `&str` representing the `u64` of the algorithm.
    fn as_str(&self) -> &str {
        match *self {
            Algorithm::Scrypt => "0",
            Algorithm::SHA256 => "1",
            Algorithm::ScryptNf => "2",
            Algorithm::X11 => "3",
            Algorithm::X13 => "4",
            Algorithm::Keccak => "5",
            Algorithm::X15 => "6",
            Algorithm::Nist5 => "7",
            Algorithm::NeoScrypt => "8",
            Algorithm::Lyra2RE => "9",
            Algorithm::WhirlpoolX => "10",
            Algorithm::Qubit => "11",
            Algorithm::Quark => "12",
            Algorithm::Axiom => "13",
            Algorithm::Lyra2REv2 => "14",
            Algorithm::ScryptJaneNf16 => "15",
            Algorithm::Blake256r8 => "16",
            Algorithm::Blake256r14 => "17",
            Algorithm::Blake256r8vnl => "18",
            Algorithm::Hodl => "19",
            Algorithm::DaggerHashimoto => "20",
            Algorithm::Decred => "21",
            Algorithm::CryptoNight => "22",
            Algorithm::Lbry => "23",
            Algorithm::Equihash => "24",
        }
    }
}
