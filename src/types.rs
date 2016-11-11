//! Types used in the nicehash.com API.

use std::time::Duration;
use std::collections::BTreeMap;

use serde_json::value::Value;

use super::{Algorithm, OrderType};
use error::{Result, Error};

/// `GlobalStats` structure.
#[derive(Debug, Default)]
pub struct GlobalStats {
    scrypt: AlgoStat,
    sha256: AlgoStat,
    scrypt_nf: AlgoStat,
    x11: AlgoStat,
    x13: AlgoStat,
    keccak: AlgoStat,
    x15: AlgoStat,
    nist5: AlgoStat,
    neo_scrypt: AlgoStat,
    lyra2_re: AlgoStat,
    whirlpool_x: AlgoStat,
    qubit: AlgoStat,
    quark: AlgoStat,
    axiom: AlgoStat,
    lyra2_rev2: AlgoStat,
    scrypt_jane_nf16: AlgoStat,
    blake256r8: AlgoStat,
    blake256r14: AlgoStat,
    blake256r8vnl: AlgoStat,
    hodl: AlgoStat,
    dagger_hashimoto: AlgoStat,
    decred: AlgoStat,
    crypto_night: AlgoStat,
    lbry: AlgoStat,
    equihash: AlgoStat,
}

impl GlobalStats {
    /// Creates a `GlobalStats` object from a JSON value.
    pub fn from_json(json: Vec<Value>) -> Result<GlobalStats> {
        if json.len() != 25 {
            return Err(Error::Api(format!("global statistics information structure must have \
                                           25 objects but it had {}",
                                          json.len())));
        }
        let mut stats = GlobalStats::default();
        for stat in json {
            if let Value::Object(stat) = stat {
                let (algorithm, stat) = AlgoStat::from_json(stat)?;
                match algorithm {
                    Algorithm::Scrypt => stats.scrypt = stat,
                    Algorithm::SHA256 => stats.sha256 = stat,
                    Algorithm::ScryptNf => stats.scrypt_nf = stat,
                    Algorithm::X11 => stats.x11 = stat,
                    Algorithm::X13 => stats.x13 = stat,
                    Algorithm::Keccak => stats.keccak = stat,
                    Algorithm::X15 => stats.x15 = stat,
                    Algorithm::Nist5 => stats.nist5 = stat,
                    Algorithm::NeoScrypt => stats.neo_scrypt = stat,
                    Algorithm::Lyra2RE => stats.lyra2_re = stat,
                    Algorithm::WhirlpoolX => stats.whirlpool_x = stat,
                    Algorithm::Qubit => stats.qubit = stat,
                    Algorithm::Quark => stats.quark = stat,
                    Algorithm::Axiom => stats.axiom = stat,
                    Algorithm::Lyra2REv2 => stats.lyra2_rev2 = stat,
                    Algorithm::ScryptJaneNf16 => stats.scrypt_jane_nf16 = stat,
                    Algorithm::Blake256r8 => stats.blake256r8 = stat,
                    Algorithm::Blake256r14 => stats.blake256r14 = stat,
                    Algorithm::Blake256r8vnl => stats.blake256r8vnl = stat,
                    Algorithm::Hodl => stats.hodl = stat,
                    Algorithm::DaggerHashimoto => stats.dagger_hashimoto = stat,
                    Algorithm::Decred => stats.decred = stat,
                    Algorithm::CryptoNight => stats.crypto_night = stat,
                    Algorithm::Lbry => stats.lbry = stat,
                    Algorithm::Equihash => stats.equihash = stat,
                }
            } else {
                return Err(Error::Api("invalid algorithm found in global stats".to_owned()));
            }
        }
        Ok(stats)
    }

    /// Gets stats for the given algorithm.
    pub fn get_stats_for(&self, alg: Algorithm) -> &AlgoStat {
        match alg {
            Algorithm::Scrypt => &self.scrypt,
            Algorithm::SHA256 => &self.sha256,
            Algorithm::ScryptNf => &self.scrypt_nf,
            Algorithm::X11 => &self.x11,
            Algorithm::X13 => &self.x13,
            Algorithm::Keccak => &self.keccak,
            Algorithm::X15 => &self.x15,
            Algorithm::Nist5 => &self.nist5,
            Algorithm::NeoScrypt => &self.neo_scrypt,
            Algorithm::Lyra2RE => &self.lyra2_re,
            Algorithm::WhirlpoolX => &self.whirlpool_x,
            Algorithm::Qubit => &self.qubit,
            Algorithm::Quark => &self.quark,
            Algorithm::Axiom => &self.axiom,
            Algorithm::Lyra2REv2 => &self.lyra2_rev2,
            Algorithm::ScryptJaneNf16 => &self.scrypt_jane_nf16,
            Algorithm::Blake256r8 => &self.blake256r8,
            Algorithm::Blake256r14 => &self.blake256r14,
            Algorithm::Blake256r8vnl => &self.blake256r8vnl,
            Algorithm::Hodl => &self.hodl,
            Algorithm::DaggerHashimoto => &self.dagger_hashimoto,
            Algorithm::Decred => &self.decred,
            Algorithm::CryptoNight => &self.crypto_night,
            Algorithm::Lbry => &self.lbry,
            Algorithm::Equihash => &self.equihash,
        }
    }
}

/// Statistics about an algorithm.
#[derive(Debug, Default)]
pub struct AlgoStat {
    price: f64,
    speed: f64,
    profitability_above_btc: Option<f64>,
    profitability_btc: Option<f64>,
    profitability_above_ltc: Option<f64>,
    profitability_ltc: Option<f64>,
    profitability_above_eth: Option<f64>,
    profitability_eth: Option<f64>,
}

impl AlgoStat {
    /// Creates an `AlgoStat` from a JSON value.
    fn from_json(json: BTreeMap<String, Value>) -> Result<(Algorithm, AlgoStat)> {
        let algorithm = Algorithm::from_u64(json.get("algo")
                .ok_or(Error::Api("`algo` not found in stats".to_owned()))?
            .as_u64()
            .ok_or(Error::Api("invalid algorithm in stats (must be an unsigned integer)"
                .to_owned()))?)?;

        let profitability_above_btc = match json.get("profitability_above_btc") {
            Some(v) => {
                Some(v.as_str()
                        .ok_or(Error::Api("invalid `profitability_above_btc` in stats (must \
                                           be a float in a string)"
                            .to_owned()))?
                    .parse()?)
            }
            None => None,
        };
        let profitability_btc = match json.get("profitability_btc") {
            Some(v) => {
                Some(v.as_str()
                        .ok_or(Error::Api("invalid `profitability_btc` in stats (must be a \
                                           float in a string)"
                            .to_owned()))?
                    .parse()?)
            }
            None => None,
        };

        let profitability_above_eth = match json.get("profitability_above_eth") {
            Some(v) => {
                Some(v.as_str()
                        .ok_or(Error::Api("invalid `profitability_above_eth` in stats (must \
                                           be a float in a string)"
                            .to_owned()))?
                    .parse()?)
            }
            None => None,
        };
        let profitability_eth = match json.get("profitability_eth") {
            Some(v) => {
                Some(v.as_str()
                        .ok_or(Error::Api("invalid `profitability_eth` in stats (must be a \
                                           float in a string)"
                            .to_owned()))?
                    .parse()?)
            }
            None => None,
        };

        let profitability_above_ltc = match json.get("profitability_above_ltc") {
            Some(v) => {
                Some(v.as_str()
                        .ok_or(Error::Api("invalid `profitability_above_ltc` in stats (must \
                                           be a float in a string)"
                            .to_owned()))?
                    .parse()?)
            }
            None => None,
        };
        let profitability_ltc = match json.get("profitability_ltc") {
            Some(v) => {
                Some(v.as_str()
                        .ok_or(Error::Api("invalid `profitability_ltc` in stats (must be a \
                                           float in a string)"
                            .to_owned()))?
                    .parse()?)
            }
            None => None,
        };

        Ok((algorithm,
            AlgoStat {
            price: json.get("price")
                        .ok_or(Error::Api("`price` not found in stats".to_owned()))?
                    .as_str()
                    .ok_or(Error::Api("invalid `price` in stats (must be a float in a string)"
                        .to_owned()))?
                .parse()?,
            speed: json.get("speed")
                        .ok_or(Error::Api("`speed` not found in stats".to_owned()))?
                    .as_str()
                    .ok_or(Error::Api("invalid `speed` in stats (must be a float in a string)"
                        .to_owned()))?
                .parse()?,
            profitability_above_btc: profitability_above_btc,
            profitability_btc: profitability_btc,
            profitability_above_eth: profitability_above_eth,
            profitability_eth: profitability_eth,
            profitability_above_ltc: profitability_above_ltc,
            profitability_ltc: profitability_ltc,
        }))
    }

    /// Gets the price
    pub fn get_price(&self) -> f64 {
        self.price
    }

    /// Gets the hashing speed.
    pub fn get_speed(&self) -> f64 {
        self.speed
    }

    /// Gets the profitability above BTC.
    pub fn get_profitability_above_btc(&self) -> Option<f64> {
        self.profitability_above_btc
    }

    /// Gets the profitability in BTCs.
    pub fn get_profitability_btc(&self) -> Option<f64> {
        self.profitability_btc
    }

    /// Gets the profitability above ETH.
    pub fn get_profitability_above_eth(&self) -> Option<f64> {
        self.profitability_above_eth
    }

    /// Gets the profitability in ETHs.
    pub fn get_profitability_eth(&self) -> Option<f64> {
        self.profitability_eth
    }

    /// Gets the profitability above LTC.
    pub fn get_profitability_above_ltc(&self) -> Option<f64> {
        self.profitability_above_ltc
    }

    /// Gets the profitability in LTCs.
    pub fn get_profitability_ltc(&self) -> Option<f64> {
        self.profitability_ltc
    }
}

/// Order struct.
#[derive(Debug)]
pub struct Order {
    id: u64,
    order_type: OrderType,
    limit_speed: f64,
    alive: bool,
    price: f64,
    workers: u64,
    algorithm: Algorithm,
    accepted_speed: f64,
}

impl Order {
    /// Creates a new `Order` from a JSON value.
    pub fn from_json(json: Value) -> Result<Order> {
        if let Value::Object(v) = json {
            let id = v.get("id")
                    .ok_or(Error::Api("no `id` parameter found in the order".to_owned()))?
                .as_u64()
                .ok_or(Error::Api("invalid order id".to_owned()))?;

            let order_type = OrderType::from_u64(v.get("type")
                    .ok_or(Error::Api("no `type` parameter found in the order".to_owned()))?
                .as_u64()
                .ok_or(Error::Api("invalid order type".to_owned()))?)?;

            let limit_speed = v.get("limit_speed")
                        .ok_or(Error::Api("no `limit_speed` parameter found in the order"
                            .to_owned()))?
                    .as_str()
                    .ok_or(Error::Api("invalid order limit speed".to_owned()))?
                .parse()?;

            let alive = v.get("alive")
                    .ok_or(Error::Api("no `alive` parameter found in the order".to_owned()))?
                .as_bool()
                .ok_or(Error::Api("invalid order `alive` parameter".to_owned()))?;

            let price = v.get("price")
                        .ok_or(Error::Api("no `price` parameter found in the order".to_owned()))?
                    .as_str()
                    .ok_or(Error::Api("invalid order price".to_owned()))?
                .parse()?;

            let workers = v.get("workers")
                    .ok_or(Error::Api("no `workers` parameter found in the order".to_owned()))?
                .as_u64()
                .ok_or(Error::Api("invalid order workers".to_owned()))?;

            let algorithm = Algorithm::from_u64(v.get("algo")
                    .ok_or(Error::Api("no algorithm found in the order".to_owned()))?
                .as_u64()
                .ok_or(Error::Api("invalid order algorithm".to_owned()))?)?;

            let accepted_speed = v.get("accepted_speed")
                        .ok_or(Error::Api("no `accepted_speed` parameter found in the order"
                            .to_owned()))?
                    .as_str()
                    .ok_or(Error::Api("invalid order accepted speed".to_owned()))?
                .parse()?;

            Ok(Order {
                id: id,
                order_type: order_type,
                limit_speed: limit_speed,
                alive: alive,
                price: price,
                workers: workers,
                algorithm: algorithm,
                accepted_speed: accepted_speed,
            })
        } else {
            Err(Error::Api("invalid order object".to_owned()))
        }
    }

    /// Gets the ID of the order.
    pub fn get_id(&self) -> u64 {
        self.id
    }

    /// Gets the type of the order.
    pub fn get_order_type(&self) -> OrderType {
        self.order_type
    }

    /// Gets the speed limit of the order.
    pub fn get_limit_speed(&self) -> f64 {
        self.limit_speed
    }

    /// Gets if the order is alive or not.
    pub fn is_alive(&self) -> bool {
        self.alive
    }

    /// Gets the price of the order.
    pub fn get_price(&self) -> f64 {
        self.price
    }

    /// Gets the number of workers working for the order.
    pub fn get_workers(&self) -> u64 {
        self.workers
    }

    /// Gets the algorithm of the order.
    pub fn get_algorithm(&self) -> Algorithm {
        self.algorithm
    }

    /// Gets the current accepted speed of the order.
    pub fn get_accepted_speed(&self) -> f64 {
        self.accepted_speed
    }
}

/// Buy information structure.
#[derive(Debug)]
pub struct BuyInfo {
    down_time: Duration,
    scrypt: AlgoBuyInfo,
    sha256: AlgoBuyInfo,
    scrypt_nf: AlgoBuyInfo,
    x11: AlgoBuyInfo,
    x13: AlgoBuyInfo,
    keccak: AlgoBuyInfo,
    x15: AlgoBuyInfo,
    nist5: AlgoBuyInfo,
    neo_scrypt: AlgoBuyInfo,
    lyra2_re: AlgoBuyInfo,
    whirlpool_x: AlgoBuyInfo,
    qubit: AlgoBuyInfo,
    quark: AlgoBuyInfo,
    axiom: AlgoBuyInfo,
    lyra2_rev2: AlgoBuyInfo,
    scrypt_jane_nf16: AlgoBuyInfo,
    blake256r8: AlgoBuyInfo,
    blake256r14: AlgoBuyInfo,
    blake256r8vnl: AlgoBuyInfo,
    hodl: AlgoBuyInfo,
    dagger_hashimoto: AlgoBuyInfo,
    decred: AlgoBuyInfo,
    crypto_night: AlgoBuyInfo,
    lbry: AlgoBuyInfo,
    equihash: AlgoBuyInfo,
}

impl BuyInfo {
    /// Creates a `BuyInfo` object from a JSON value.
    pub fn from_json(json: BTreeMap<String, Value>) -> Result<BuyInfo> {
        let mut down_time = None;
        let mut scrypt = None;
        let mut sha256 = None;
        let mut scrypt_nf = None;
        let mut x11 = None;
        let mut x13 = None;
        let mut keccak = None;
        let mut x15 = None;
        let mut nist5 = None;
        let mut neo_scrypt = None;
        let mut lyra2_re = None;
        let mut whirlpool_x = None;
        let mut qubit = None;
        let mut quark = None;
        let mut axiom = None;
        let mut lyra2_rev2 = None;
        let mut scrypt_jane_nf16 = None;
        let mut blake256r8 = None;
        let mut blake256r14 = None;
        let mut blake256r8vnl = None;
        let mut hodl = None;
        let mut dagger_hashimoto = None;
        let mut decred = None;
        let mut crypto_night = None;
        let mut lbry = None;
        let mut equihash = None;
        for (key, value) in json {
            match key.as_str() {
                "down_time" => {
                    down_time = Some(value.as_u64()
                        .ok_or(Error::Api("invalid `down_time` in buy information, expected \
                                           u64"
                            .to_owned()))?);
                }
                "algorithms" => {
                    if let Value::Array(arr) = value {
                        if arr.len() != 25 {
                            return Err(Error::Api(format!("buy information structure must have \
                                                           25 objects but it had {}",
                                                          arr.len())));
                        }

                        for alg_buy_info in arr {
                            let (algorithm, alg_buy_info) =
                                AlgoBuyInfo::from_json(alg_buy_info)?;
                            match algorithm {
                                Algorithm::Scrypt => scrypt = Some(alg_buy_info),
                                Algorithm::SHA256 => sha256 = Some(alg_buy_info),
                                Algorithm::ScryptNf => scrypt_nf = Some(alg_buy_info),
                                Algorithm::X11 => x11 = Some(alg_buy_info),
                                Algorithm::X13 => x13 = Some(alg_buy_info),
                                Algorithm::Keccak => keccak = Some(alg_buy_info),
                                Algorithm::X15 => x15 = Some(alg_buy_info),
                                Algorithm::Nist5 => nist5 = Some(alg_buy_info),
                                Algorithm::NeoScrypt => neo_scrypt = Some(alg_buy_info),
                                Algorithm::Lyra2RE => lyra2_re = Some(alg_buy_info),
                                Algorithm::WhirlpoolX => whirlpool_x = Some(alg_buy_info),
                                Algorithm::Qubit => qubit = Some(alg_buy_info),
                                Algorithm::Quark => quark = Some(alg_buy_info),
                                Algorithm::Axiom => axiom = Some(alg_buy_info),
                                Algorithm::Lyra2REv2 => lyra2_rev2 = Some(alg_buy_info),
                                Algorithm::ScryptJaneNf16 => scrypt_jane_nf16 = Some(alg_buy_info),
                                Algorithm::Blake256r8 => blake256r8 = Some(alg_buy_info),
                                Algorithm::Blake256r14 => blake256r14 = Some(alg_buy_info),
                                Algorithm::Blake256r8vnl => blake256r8vnl = Some(alg_buy_info),
                                Algorithm::Hodl => hodl = Some(alg_buy_info),
                                Algorithm::DaggerHashimoto => dagger_hashimoto = Some(alg_buy_info),
                                Algorithm::Decred => decred = Some(alg_buy_info),
                                Algorithm::CryptoNight => crypto_night = Some(alg_buy_info),
                                Algorithm::Lbry => lbry = Some(alg_buy_info),
                                Algorithm::Equihash => equihash = Some(alg_buy_info),
                            }
                        }
                    } else {
                        return Err(Error::Api("invalid `algorithms` in buy information, \
                                               expected array"
                            .to_owned()));
                    }
                }
                k => {
                    return Err(Error::Api(format!("unknown key `{}` found in buy information \
                                                   JSON object",
                                                  k)));
                }
            }
        }

        Ok(BuyInfo {
            down_time: Duration::from_secs(down_time.ok_or(
                Error::Api("`down_time` not found in buy information structure".to_owned())
            )?),
            scrypt: scrypt.ok_or(
                Error::Api("Scrypt algorithm information not found in buy information \
                structure".to_owned())
            )?,
            sha256: sha256.ok_or(
                Error::Api("SHA256 algorithm information not found in buy information \
                structure".to_owned())
            )?,
            scrypt_nf: scrypt_nf.ok_or(
                Error::Api("ScryptNf algorithm information not found in buy information \
                structure".to_owned())
            )?,
            x11: x11.ok_or(Error::Api("X11 algorithm information not found in buy information \
                                   structure"
                    .to_owned()))?,
            x13: x13.ok_or(Error::Api("X13 algorithm information not found in buy information \
                                   structure"
                    .to_owned()))?,
            keccak: keccak.ok_or(
                Error::Api("Keccak algorithm information not found in buy information \
                structure".to_owned())
            )?,
            x15: x15.ok_or(Error::Api("X15 algorithm information not found in buy information \
                                   structure"
                    .to_owned()))?,
            nist5: nist5.ok_or(
                Error::Api("Nist5 algorithm information not found in buy information \
                structure".to_owned())
            )?,
            neo_scrypt: neo_scrypt.ok_or(
                Error::Api("NeoScrypt algorithm information not found in buy information \
                structure".to_owned())
            )?,
            lyra2_re: lyra2_re.ok_or(
                Error::Api("Lyra2RE algorithm information not found in buy information \
                structure".to_owned())
            )?,
            whirlpool_x: whirlpool_x.ok_or(
                Error::Api("WhirlpoolX algorithm information not found in buy information \
                structure".to_owned())
            )?,
            qubit: qubit.ok_or(
                Error::Api("Qubit algorithm information not found in buy information \
                structure".to_owned())
            )?,
            quark: quark.ok_or(
                Error::Api("Quark algorithm information not found in buy information \
                structure".to_owned())
            )?,
            axiom: axiom.ok_or(
                Error::Api("Axiom algorithm information not found in buy information \
                structure".to_owned())
            )?,
            lyra2_rev2: lyra2_rev2.ok_or(
                Error::Api("Lyra2REv2 algorithm information not found in buy information \
                structure".to_owned())
            )?,
            scrypt_jane_nf16: scrypt_jane_nf16.ok_or(
                Error::Api("ScryptJaneNf16 algorithm information not found in buy information \
                structure".to_owned())
            )?,
            blake256r8: blake256r8.ok_or(
                Error::Api("Blake256r8 algorithm information not found in buy information \
                structure".to_owned())
            )?,
            blake256r14: blake256r14.ok_or(
                Error::Api("Blake256r14 algorithm information not found in buy information \
                structure".to_owned())
            )?,
            blake256r8vnl: blake256r8vnl.ok_or(
                Error::Api("Blake256r8vnl algorithm information not found in buy information \
                structure".to_owned())
            )?,
            hodl: hodl.ok_or(
                Error::Api("Hodl algorithm information not found in buy information \
                structure".to_owned())
            )?,
            dagger_hashimoto: dagger_hashimoto.ok_or(
                Error::Api("DaggerHashimoto algorithm information not found in buy information \
                structure".to_owned())
            )?,
            decred: decred.ok_or(
                Error::Api("Decred algorithm information not found in buy information \
                structure".to_owned())
            )?,
            crypto_night: crypto_night.ok_or(
                Error::Api("CryptoNight algorithm information not found in buy information \
                structure".to_owned())
            )?,
            lbry: lbry.ok_or(
                Error::Api("Lbry algorithm information not found in buy information \
                structure".to_owned())
            )?,
            equihash: equihash.ok_or(
                Error::Api("Equihash algorithm information not found in buy information \
                structure".to_owned())
            )?,
        })
    }

    /// Gets buy information for the given algorithm.
    pub fn get_buy_info_for(&self, alg: Algorithm) -> &AlgoBuyInfo {
        match alg {
            Algorithm::Scrypt => &self.scrypt,
            Algorithm::SHA256 => &self.sha256,
            Algorithm::ScryptNf => &self.scrypt_nf,
            Algorithm::X11 => &self.x11,
            Algorithm::X13 => &self.x13,
            Algorithm::Keccak => &self.keccak,
            Algorithm::X15 => &self.x15,
            Algorithm::Nist5 => &self.nist5,
            Algorithm::NeoScrypt => &self.neo_scrypt,
            Algorithm::Lyra2RE => &self.lyra2_re,
            Algorithm::WhirlpoolX => &self.whirlpool_x,
            Algorithm::Qubit => &self.qubit,
            Algorithm::Quark => &self.quark,
            Algorithm::Axiom => &self.axiom,
            Algorithm::Lyra2REv2 => &self.lyra2_rev2,
            Algorithm::ScryptJaneNf16 => &self.scrypt_jane_nf16,
            Algorithm::Blake256r8 => &self.blake256r8,
            Algorithm::Blake256r14 => &self.blake256r14,
            Algorithm::Blake256r8vnl => &self.blake256r8vnl,
            Algorithm::Hodl => &self.hodl,
            Algorithm::DaggerHashimoto => &self.dagger_hashimoto,
            Algorithm::Decred => &self.decred,
            Algorithm::CryptoNight => &self.crypto_night,
            Algorithm::Lbry => &self.lbry,
            Algorithm::Equihash => &self.equihash,
        }
    }

    /// Gets the minimum duration between two consecutive order price reductions.
    pub fn get_down_time(&self) -> Duration {
        self.down_time
    }
}

/// Buy information for an algorithm.
#[derive(Debug, Default)]
pub struct AlgoBuyInfo {
    down_step: f64,
    min_limit: f64,
    speed_text: String,
    multi: f64,
}

impl AlgoBuyInfo {
    /// Creates a `BuyInfo` object from a JSON value.
    pub fn from_json(json: Value) -> Result<(Algorithm, AlgoBuyInfo)> {
        if let Value::Object(buy_info) = json {
            let algorithm = Algorithm::from_u64(buy_info.get("algo")
                    .ok_or(Error::Api("`algo` not found in buy information".to_owned()))?
                .as_u64()
                .ok_or(Error::Api("invalid algorithm in buy information (must be an unsigned \
                                   integer)"
                    .to_owned()))?)?;

            Ok((algorithm,
                AlgoBuyInfo {
                down_step: buy_info.get("down_step")
                            .ok_or(Error::Api("`down_step` not found in buy information"
                                .to_owned()))?
                        .as_str()
                        .ok_or(Error::Api("invalid `down_step` in stats (must be a float in a \
                                           string)"
                            .to_owned()))?
                    .parse()?,
                min_limit: buy_info.get("min_limit")
                            .ok_or(Error::Api("`min_limit` not found in buy information"
                                .to_owned()))?
                        .as_str()
                        .ok_or(Error::Api("invalid `min_limit` in stats (must be a float in a \
                                           string)"
                            .to_owned()))?
                    .parse()?,
                speed_text: buy_info.get("speed_text")
                            .ok_or(Error::Api("`speed_text` not found in buy information"
                                .to_owned()))?
                        .as_str()
                        .ok_or(Error::Api("invalid `speed_text` in stats (must be a string)"
                            .to_owned()))?
                    .to_owned(),
                multi: buy_info.get("multi")
                            .ok_or(Error::Api("`multi` not found in buy information"
                                .to_owned()))?
                        .as_str()
                        .ok_or(Error::Api("invalid `multi` in stats (must be a float in a \
                                           string)"
                            .to_owned()))?
                    .parse()?,
            }))
        } else {
            Err(Error::Api("invalid buy information object".to_owned()))
        }
    }

    /// Gets the step for price downgrade.
    pub fn get_down_step(&self) -> f64 {
        self.down_step
    }

    /// Gets the minimum hashing limit of the algorithm.
    pub fn get_min_limit(&self) -> f64 {
        self.min_limit
    }

    /// Gets the text for the unit of the speed of the algorithm.
    pub fn get_speed_text(&self) -> &str {
        &self.speed_text
    }

    /// Gets the multiplier of the algorithm.
    pub fn get_multiplier(&self) -> f64 {
        self.multi
    }
}

/// Pool information.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PoolInfo {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
}

/// New order information.
#[derive(Debug)]
pub struct NewOrder {
    pub algorithm: Algorithm,
    pub amount: f64,
    pub price: f64,
    pub limit: Option<f64>,
}

/// Account balance.
#[derive(Debug, Default)]
pub struct Balance {
    pub confirmed: f64,
    pub pending: f64,
}
