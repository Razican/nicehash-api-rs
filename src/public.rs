//! Public API methods.

use hyper::Url;
use serde_json::de;
use serde_json::value::Value;

use super::{Client, API_URL, Location, Algorithm};
use error::{Result, Error};
use types::{GlobalStats, Order, BuyInfo};

/// Public API methods.
impl Client {
    /// Gets current global stats.
    pub fn global_stats_current(&self, location: Option<Location>) -> Result<GlobalStats> {
        let mut url = Url::parse(API_URL).unwrap();
        {
            let mut query_pairs = url.query_pairs_mut();
            let _ = query_pairs.append_pair("method", "stats.global.current");
            if let Some(location) = location {
                let _ = query_pairs.append_pair("location", location.as_str());
            }
        }

        let response = self.inner.get(url).send()?;
        if let Value::Object(r) = de::from_reader(response)? {
            for (key, value) in r {
                if let ("result", Value::Object(s)) = (key.as_str(), value) {
                    if s.is_empty() {
                        return Err(Error::Api("empty result response to `stats.global.current`"
                            .to_owned()));
                    }
                    for (key, value) in s {
                        match key.as_str() {
                            "stats" => {
                                if let Value::Array(arr) = value {
                                    return GlobalStats::from_json(arr);
                                } else {
                                    return Err(Error::Api("invalid `stats` field found in \
                                                           `stats.global.current` response, \
                                                           expected array"
                                        .to_owned()));
                                }
                            }
                            "error" => {
                                if let Value::String(s) = value {
                                    return Err(Error::Result(s));
                                } else {
                                    return Err(Error::Api("invalid `error` field found in \
                                                           `stats.global.current` response, \
                                                           expected string"
                                        .to_owned()));
                                }
                            }
                            f => {
                                return Err(Error::Api(format!("unknown field `{}` found in \
                                                               `stats.global.current` response",
                                                              f)));
                            }
                        }
                    }
                    return Err(Error::Api("invalid response to `stats.global.current` \
                                           method, no `stats` field was found"
                        .to_owned()));
                }
            }
            Err(Error::Api("invalid response to `stats.global.current` method, no `result` \
                            field was found"
                .to_owned()))
        } else {
            Err(Error::Api("invalid response to `stats.global.current` method".to_owned()))
        }
    }

    /// Gets global stats for the last 24h.
    pub fn global_stats_24h(&self) -> Result<GlobalStats> {
        let mut url = Url::parse(API_URL).unwrap();
        {
            let mut query_pairs = url.query_pairs_mut();
            let _ = query_pairs.append_pair("method", "stats.global.24h");
        }

        let response = self.inner.get(url).send()?;
        if let Value::Object(r) = de::from_reader(response)? {
            for (key, value) in r {
                if let ("result", Value::Object(s)) = (key.as_str(), value) {
                    if let Some((key, Value::Array(arr))) = s.into_iter().next() {
                        if key.as_str() == "stats" {
                            return GlobalStats::from_json(arr);
                        } else {
                            return Err(Error::Api("invalid response to `stats.global.24` \
                                                   method, no `stats` field was found"
                                .to_owned()));
                        }
                    } else {
                        return Err(Error::Api("invalid response to `stats.global.24` \
                                               method, no `stats` field was found"
                            .to_owned()));
                    }
                }
            }
            Err(Error::Api("invalid response to `stats.global.24` method, no `result` \
                            field was found"
                .to_owned()))
        } else {
            Err(Error::Api("invalid response to `stats.global.24` method".to_owned()))
        }
    }

    /// Gets all orders for the given algorithm and location.
    pub fn get_orders(&self, location: Location, algorithm: Algorithm) -> Result<Vec<Order>> {
        let mut url = Url::parse(API_URL).unwrap();
        {
            let mut query_pairs = url.query_pairs_mut();
            let _ = query_pairs.append_pair("method", "orders.get");
            let _ = query_pairs.append_pair("location", location.as_str());
            let _ = query_pairs.append_pair("algo", algorithm.as_str());
        }

        let response = self.inner.get(url).send()?;
        if let Value::Object(r) = de::from_reader(response)? {
            for (key, value) in r {
                if let ("result", Value::Object(s)) = (key.as_str(), value) {
                    if let Some((key, Value::Array(arr))) = s.into_iter().next() {
                        if key.as_str() == "orders" {
                            let mut orders = Vec::with_capacity(arr.len());
                            for order_json in arr {
                                match Order::from_json(order_json) {
                                    Ok(o) => orders.push(o),
                                    Err(e) => return Err(e),
                                }
                            }
                            return Ok(orders);
                        } else {
                            return Err(Error::Api("invalid response to `orders.get` method, no \
                                                   `orders` field was found"
                                .to_owned()));
                        }
                    } else {
                        return Err(Error::Api("invalid response to `orders.get` method, no \
                                               `orders` field was found"
                            .to_owned()));
                    }
                }
            }
            Err(Error::Api("invalid response to `orders.get` method, no `result` field was found"
                .to_owned()))
        } else {
            Err(Error::Api("invalid response to `orders.get` method".to_owned()))
        }
    }

    /// Gets needed information for buying hashing power.
    pub fn get_buy_info(&self) -> Result<BuyInfo> {
        let mut url = Url::parse(API_URL).unwrap();
        {
            let mut query_pairs = url.query_pairs_mut();
            let _ = query_pairs.append_pair("method", "buy.info");
        }

        let response = self.inner.get(url).send()?;
        if let Value::Object(r) = de::from_reader(response)? {
            for (key, value) in r {
                if let ("result", Value::Object(res)) = (key.as_str(), value) {
                    return BuyInfo::from_json(res);
                }
            }
            Err(Error::Api("invalid response to `buy.info` method, no `result` field was found"
                .to_owned()))
        } else {
            Err(Error::Api("invalid response to `buy.info` method".to_owned()))
        }
    }
}
