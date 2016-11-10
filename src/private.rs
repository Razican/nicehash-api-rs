//! Private API methods.

use hyper::Url;
use serde_json::de;
use serde_json::value::Value;

use super::{Client, API_URL, Location, Algorithm};
use error::{Result, Error};
use types::{Order, PoolInfo, NewOrder, Balance};

/// Private API methods.
impl Client {
    /// Gets orders for the current user.
    pub fn get_my_orders<S: AsRef<str>>(&self,
                                        api_id: u64,
                                        api_key: S,
                                        location: Location,
                                        algorithm: Algorithm)
                                        -> Result<Vec<Order>> {
        let mut url = Url::parse(API_URL).unwrap();
        {
            let mut query_pairs = url.query_pairs_mut();
            query_pairs.append_pair("method", "orders.get");
            query_pairs.append_pair("my", "");
            query_pairs.append_pair("id", &format!("{}", api_id));
            query_pairs.append_pair("key", api_key.as_ref());
            query_pairs.append_pair("location", location.as_str());
            query_pairs.append_pair("algo", algorithm.as_str());
        }

        let response = try!(self.inner.get(url).send());
        if let Value::Object(r) = try!(de::from_reader(response)) {
            for (key, value) in r {
                if let ("result", Value::Object(s)) = (key.as_str(), value) {
                    if s.is_empty() {
                        return Err(Error::Api("empty result response to `orders.get&my`"
                            .to_owned()));
                    }
                    for (key, value) in s {
                        match key.as_str() {
                            "orders" => {
                                if let Value::Array(arr) = value {
                                    let mut orders = Vec::with_capacity(arr.len());
                                    for order_json in arr {
                                        match Order::from_json(order_json) {
                                            Ok(o) => orders.push(o),
                                            Err(e) => return Err(e),
                                        }
                                    }
                                    return Ok(orders);
                                } else {
                                    return Err(Error::Api("invalid `orders` field found in \
                                                           `orders.get&my` response, expected \
                                                           array"
                                        .to_owned()));
                                }
                            }
                            "error" => {
                                if let Value::String(s) = value {
                                    return Err(Error::Result(s));
                                } else {
                                    return Err(Error::Api("invalid `error` field found in \
                                                           `orders.get&my` response, expected \
                                                           string"
                                        .to_owned()));
                                }
                            }
                            f => {
                                return Err(Error::Api(format!("unknown field `{}` found in \
                                                               `orders.get&my` response",
                                                              f)));
                            }
                        }
                    }
                    return Err(Error::Api("invalid response to `orders.get&my` method, no \
                                           `orders` field was found"
                        .to_owned()));
                }
            }
            Err(Error::Api("invalid response to `orders.get&my` method, no `result` field was \
                            found"
                .to_owned()))
        } else {
            Err(Error::Api("invalid response to `orders.get&my` method".to_owned()))
        }
    }

    /// Creates a new order.
    pub fn create_order<K: AsRef<str>>(&self,
                                       api_id: u64,
                                       api_key: K,
                                       location: Location,
                                       order: NewOrder,
                                       pool: PoolInfo,
                                       code: Option<u32>)
                                       -> Result<u64> {
        if order.amount < 0.00000001 || order.price < 0.00000001 {
            return Err(Error::Result("Invalid amount or price.".to_owned()));
        }
        let mut url = Url::parse(API_URL).unwrap();
        {
            let mut query_pairs = url.query_pairs_mut();
            query_pairs.append_pair("method", "orders.create");
            query_pairs.append_pair("id", &format!("{}", api_id));
            query_pairs.append_pair("key", api_key.as_ref());
            query_pairs.append_pair("location", location.as_str());
            query_pairs.append_pair("algo", order.algorithm.as_str());
            query_pairs.append_pair("amount", &format!("{:.8}", order.amount));
            query_pairs.append_pair("price", &format!("{}", order.price));
            if let Some(limit) = order.limit {
                query_pairs.append_pair("limit", &format!("{}", limit));
            } else {
                query_pairs.append_pair("limit", "0");
            }
            query_pairs.append_pair("pool_host", &pool.host);
            query_pairs.append_pair("pool_port", &format!("{}", pool.port));
            query_pairs.append_pair("pool_user", &pool.username);
            query_pairs.append_pair("pool_pass", &pool.password);
            if let Some(code) = code {
                query_pairs.append_pair("code", &format!("{:06}", code));
            }
        }

        let response = try!(self.inner.get(url).send());
        if let Value::Object(r) = try!(de::from_reader(response)) {
            for (key, value) in r {
                if let ("result", Value::Object(s)) = (key.as_str(), value) {
                    if s.is_empty() {
                        return Err(Error::Api("empty result response to `orders.create`"
                            .to_owned()));
                    }
                    for (key, value) in s {
                        match key.as_str() {
                            "success" => {
                                if let Value::String(message) = value {
                                    let (_, id_plus) = message.split_at(6);
                                    let (id, _) = id_plus.split_at(try!(id_plus.find(' ')
                                        .ok_or(Error::Api("unexpected success string in \
                                                           orders.create response (it must \
                                                           have a space after the order ID)"
                                            .to_owned()))));
                                    return Ok(try!(id.parse()));
                                } else {
                                    return Err(Error::Api("invalid `success` field found in \
                                                           `orders.create` response, expected \
                                                          string"
                                        .to_owned()));
                                }
                            }
                            "error" => {
                                if let Value::String(s) = value {
                                    return Err(Error::Result(s));
                                } else {
                                    return Err(Error::Api("invalid `error` field found in \
                                                           `orders.create` response, expected \
                                                           string"
                                        .to_owned()));
                                }
                            }
                            f => {
                                return Err(Error::Api(format!("unknown field `{}` found in \
                                                               `orders.create` response",
                                                              f)));
                            }
                        }
                    }
                    return Err(Error::Api("invalid response to `orders.create` method, no \
                                           `success` or `error` fields were found"
                        .to_owned()));
                }
            }
            Err(Error::Api("invalid response to `orders.create` method, no `result` field was \
                            found"
                .to_owned()))
        } else {
            Err(Error::Api("invalid response to `orders.create` method".to_owned()))
        }
    }

    /// Refills the given order with the given amount.
    pub fn refill_order<K: AsRef<str>>(&self,
                                       api_id: u64,
                                       api_key: K,
                                       location: Location,
                                       algorithm: Algorithm,
                                       order_id: u64,
                                       amount: f64)
                                       -> Result<()> {
        if order_id == 0 || amount < 0.00000001 {
            return Err(Error::Result("Invalid amount or order id.".to_owned()));
        }
        let mut url = Url::parse(API_URL).unwrap();
        {
            let mut query_pairs = url.query_pairs_mut();
            query_pairs.append_pair("method", "orders.refill");
            query_pairs.append_pair("id", &format!("{}", api_id));
            query_pairs.append_pair("key", api_key.as_ref());
            query_pairs.append_pair("location", location.as_str());
            query_pairs.append_pair("algo", algorithm.as_str());
            query_pairs.append_pair("order", &format!("{}", order_id));
            query_pairs.append_pair("amount", &format!("{:.8}", amount));
        }

        let response = try!(self.inner.get(url).send());
        if let Value::Object(r) = try!(de::from_reader(response)) {
            for (key, value) in r {
                if let ("result", Value::Object(s)) = (key.as_str(), value) {
                    if s.is_empty() {
                        return Err(Error::Api("empty result response to `orders.refill`"
                            .to_owned()));
                    }
                    for (key, value) in s {
                        match key.as_str() {
                            "success" => {
                                return Ok(());
                            }
                            "error" => {
                                if let Value::String(s) = value {
                                    return Err(Error::Result(s));
                                } else {
                                    return Err(Error::Api("invalid `error` field found in \
                                                           `orders.refill` response, expected \
                                                           string"
                                        .to_owned()));
                                }
                            }
                            f => {
                                return Err(Error::Api(format!("unknown field `{}` found in \
                                                               `orders.refill` response",
                                                              f)));
                            }
                        }
                    }
                    return Err(Error::Api("invalid response to `orders.refill` method, no \
                                           `success` or `error` fields were found"
                        .to_owned()));
                }
            }
            Err(Error::Api("invalid response to `orders.refill` method, no `result` field was \
                            found"
                .to_owned()))
        } else {
            Err(Error::Api("invalid response to `orders.refill` method".to_owned()))
        }
    }

    /// Removes the given order.
    pub fn remove_order<K: AsRef<str>>(&self,
                                       api_id: u64,
                                       api_key: K,
                                       location: Location,
                                       algorithm: Algorithm,
                                       order_id: u64)
                                       -> Result<()> {
        if order_id == 0 {
            return Err(Error::Result("Unknown order id.".to_owned()));
        }
        let mut url = Url::parse(API_URL).unwrap();
        {
            let mut query_pairs = url.query_pairs_mut();
            query_pairs.append_pair("method", "orders.remove");
            query_pairs.append_pair("id", &format!("{}", api_id));
            query_pairs.append_pair("key", api_key.as_ref());
            query_pairs.append_pair("location", location.as_str());
            query_pairs.append_pair("algo", algorithm.as_str());
            query_pairs.append_pair("order", &format!("{}", order_id));
        }

        let response = try!(self.inner.get(url).send());
        if let Value::Object(r) = try!(de::from_reader(response)) {
            for (key, value) in r {
                if let ("result", Value::Object(s)) = (key.as_str(), value) {
                    if s.is_empty() {
                        return Err(Error::Api("empty result response to `orders.remove`"
                            .to_owned()));
                    }
                    for (key, value) in s {
                        match key.as_str() {
                            "success" => {
                                return Ok(());
                            }
                            "error" => {
                                if let Value::String(s) = value {
                                    return Err(Error::Result(s));
                                } else {
                                    return Err(Error::Api("invalid `error` field found in \
                                                           `orders.remove` response, expected \
                                                           string"
                                        .to_owned()));
                                }
                            }
                            f => {
                                return Err(Error::Api(format!("unknown field `{}` found in \
                                                               `orders.remove` response",
                                                              f)));
                            }
                        }
                    }
                    return Err(Error::Api("invalid response to `orders.remove` method, no \
                                           `success` or `error` fields were found"
                        .to_owned()));
                }
            }
            Err(Error::Api("invalid response to `orders.remove` method, no `result` field was \
                            found"
                .to_owned()))
        } else {
            Err(Error::Api("invalid response to `orders.remove` method".to_owned()))
        }
    }

    /// Sets the price to the given order.
    pub fn set_order_price<K: AsRef<str>>(&self,
                                          api_id: u64,
                                          api_key: K,
                                          location: Location,
                                          algorithm: Algorithm,
                                          order_id: u64,
                                          price: f64)
                                          -> Result<()> {
        if order_id == 0 {
            return Err(Error::Result("Order id/price/algo incorrect.".to_owned()));
        }
        if price < 0.00000001 {
            return Err(Error::Result("Price incorrect.".to_owned()));
        }
        let mut url = Url::parse(API_URL).unwrap();
        {
            let mut query_pairs = url.query_pairs_mut();
            query_pairs.append_pair("method", "orders.set.price");
            query_pairs.append_pair("id", &format!("{}", api_id));
            query_pairs.append_pair("key", api_key.as_ref());
            query_pairs.append_pair("location", location.as_str());
            query_pairs.append_pair("algo", algorithm.as_str());
            query_pairs.append_pair("order", &format!("{}", order_id));
            query_pairs.append_pair("price", &format!("{:.8}", price));
        }

        let response = try!(self.inner.get(url).send());
        if let Value::Object(r) = try!(de::from_reader(response)) {
            for (key, value) in r {
                if let ("result", Value::Object(s)) = (key.as_str(), value) {
                    if s.is_empty() {
                        return Err(Error::Api("empty result response to `orders.set.price`"
                            .to_owned()));
                    }
                    for (key, value) in s {
                        match key.as_str() {
                            "success" => {
                                return Ok(());
                            }
                            "error" => {
                                if let Value::String(s) = value {
                                    return Err(Error::Result(s));
                                } else {
                                    return Err(Error::Api("invalid `error` field found in \
                                                           `orders.set.price` response, \
                                                           expected string"
                                        .to_owned()));
                                }
                            }
                            f => {
                                return Err(Error::Api(format!("unknown field `{}` found in \
                                                               `orders.set.price` response",
                                                              f)));
                            }
                        }
                    }
                    return Err(Error::Api("invalid response to `orders.set.price` method, no \
                                           `success` or `error` fields were found"
                        .to_owned()));
                }
            }
            Err(Error::Api("invalid response to `orders.set.price` method, no `result` field was \
                            found"
                .to_owned()))
        } else {
            Err(Error::Api("invalid response to `orders.set.price` method".to_owned()))
        }
    }

    /// Decrease the price to the given order.
    ///
    /// The decrease can only be done each 600 seconds (10 minutes). This number might change in
    /// the future and can be obtained with the `get_buy_info()` call. The price decrease is always
    /// done in finite amounts that are different for each algorithm and can also be obtained in
    /// that call.
    pub fn decrease_order_price<K: AsRef<str>>(&self,
                                               api_id: u64,
                                               api_key: K,
                                               location: Location,
                                               algorithm: Algorithm,
                                               order_id: u64)
                                               -> Result<f64> {
        if order_id == 0 {
            return Err(Error::Result("Order id/price/algo incorrect.".to_owned()));
        }
        let mut url = Url::parse(API_URL).unwrap();
        {
            let mut query_pairs = url.query_pairs_mut();
            query_pairs.append_pair("method", "orders.set.price.decrease");
            query_pairs.append_pair("id", &format!("{}", api_id));
            query_pairs.append_pair("key", api_key.as_ref());
            query_pairs.append_pair("location", location.as_str());
            query_pairs.append_pair("algo", algorithm.as_str());
            query_pairs.append_pair("order", &format!("{}", order_id));
        }

        let response = try!(self.inner.get(url).send());
        if let Value::Object(r) = try!(de::from_reader(response)) {
            for (key, value) in r {
                if let ("result", Value::Object(s)) = (key.as_str(), value) {
                    if s.is_empty() {
                        return Err(Error::Api("empty result response to \
                                               `orders.set.price.decrease`"
                            .to_owned()));
                    }
                    for (key, value) in s {
                        match key.as_str() {
                            "success" => {
                                if let Value::String(s) = value {
                                    let (_, price) = s.split_at(24);
                                    return Ok(try!(price.parse()));
                                } else {
                                    return Err(Error::Api("invalid `success` field found in \
                                                           `orders.set.price.decrease` \
                                                           response, expected string"
                                        .to_owned()));
                                }
                            }
                            "error" => {
                                if let Value::String(s) = value {
                                    return Err(Error::Result(s));
                                } else {
                                    return Err(Error::Api("invalid `error` field found in \
                                                           `orders.set.price.decrease` \
                                                           response, expected string"
                                        .to_owned()));
                                }
                            }
                            f => {
                                return Err(Error::Api(format!("unknown field `{}` found in \
                                                               `orders.set.price.decrease` \
                                                               response",
                                                              f)));
                            }
                        }
                    }
                    return Err(Error::Api("invalid response to `orders.set.price.decrease` \
                                           method, no `success` or `error` fields were found"
                        .to_owned()));
                }
            }
            Err(Error::Api("invalid response to `orders.set.price.decrease` method, no `result` \
                            field was found"
                .to_owned()))
        } else {
            Err(Error::Api("invalid response to `orders.set.price.decrease` method".to_owned()))
        }
    }

    /// Sets the speed limit for the given order.
    ///
    /// The decrease can only be done each 600 seconds (10 minutes). This number might change in
    /// the future and can be obtained with the `get_buy_info()` call.
    pub fn set_order_speed_limit<K: AsRef<str>>(&self,
                                                api_id: u64,
                                                api_key: K,
                                                location: Location,
                                                algorithm: Algorithm,
                                                order_id: u64,
                                                speed_limit: Option<f64>)
                                                -> Result<()> {
        // if order_id == 0 {
        //     return Err(Error::Result("Order id/price/algo incorrect.".to_owned()));
        // }
        let mut url = Url::parse(API_URL).unwrap();
        {
            let mut query_pairs = url.query_pairs_mut();
            query_pairs.append_pair("method", "orders.set.limit");
            query_pairs.append_pair("id", &format!("{}", api_id));
            query_pairs.append_pair("key", api_key.as_ref());
            query_pairs.append_pair("location", location.as_str());
            query_pairs.append_pair("algo", algorithm.as_str());
            query_pairs.append_pair("order", &format!("{}", order_id));
            if let Some(l) = speed_limit {
                // if l < 0.2 {
                //     return Err(Error::Result("Invalid limit.".to_owned()));
                // }
                query_pairs.append_pair("limit", &format!("{:.8}", l));
            } else {
                query_pairs.append_pair("limit", "0");
            }

        }

        let response = try!(self.inner.get(url).send());
        if let Value::Object(r) = try!(de::from_reader(response)) {
            for (key, value) in r {
                if let ("result", Value::Object(s)) = (key.as_str(), value) {
                    if s.is_empty() {
                        return Err(Error::Api("empty result response to \
                                               `orders.set.price.decrease`"
                            .to_owned()));
                    }
                    for (key, value) in s {
                        match key.as_str() {
                            "success" => {
                                return Ok(());
                            }
                            "error" => {
                                if let Value::String(s) = value {
                                    return Err(Error::Result(s));
                                } else {
                                    return Err(Error::Api("invalid `error` field found in \
                                                           `orders.set.price.decrease` \
                                                           response, expected string"
                                        .to_owned()));
                                }
                            }
                            f => {
                                return Err(Error::Api(format!("unknown field `{}` found in \
                                                               `orders.set.price.decrease` \
                                                               response",
                                                              f)));
                            }
                        }
                    }
                    return Err(Error::Api("invalid response to `orders.set.price.decrease` \
                                           method, no `success` or `error` fields were found"
                        .to_owned()));
                }
            }
            Err(Error::Api("invalid response to `orders.set.price.decrease` method, no `result` \
                            field was found"
                .to_owned()))
        } else {
            Err(Error::Api("invalid response to `orders.set.price.decrease` method".to_owned()))
        }
    }

    /// Gets the balance of the given account.
    pub fn get_balance<K: AsRef<str>>(&self, api_id: u64, api_key: K) -> Result<Balance> {
        let mut url = Url::parse(API_URL).unwrap();
        {
            let mut query_pairs = url.query_pairs_mut();
            query_pairs.append_pair("method", "balance");
            query_pairs.append_pair("id", &format!("{}", api_id));
            query_pairs.append_pair("key", api_key.as_ref());
        }

        let response = try!(self.inner.get(url).send());
        if let Value::Object(r) = try!(de::from_reader(response)) {
            for (key, value) in r {
                if let ("result", Value::Object(s)) = (key.as_str(), value) {
                    if s.is_empty() {
                        return Err(Error::Api("empty result response to `balance`".to_owned()));
                    }
                    let mut balance = Balance::default();
                    for (key, value) in s {
                        match key.as_str() {
                            "balance_confirmed" => {
                                if let Value::String(confirmed) = value {
                                    balance.confirmed = try!(confirmed.parse());
                                } else {
                                    return Err(Error::Api("invalid `balance_confirmed` field \
                                                           found in `balance` response, \
                                                           expected float in a string"
                                        .to_owned()));
                                }
                            }
                            "balance_pending" => {
                                if let Value::String(pending) = value {
                                    balance.pending = try!(pending.parse());
                                } else {
                                    return Err(Error::Api("invalid `balance_pending` field \
                                                           found in `balance` response, \
                                                           expected float in a string"
                                        .to_owned()));
                                }
                            }
                            "error" => {
                                if let Value::String(s) = value {
                                    return Err(Error::Result(s));
                                } else {
                                    return Err(Error::Api("invalid `error` field found in \
                                                           `balance` response, expected string"
                                        .to_owned()));
                                }
                            }
                            f => {
                                return Err(Error::Api(format!("unknown field `{}` found in \
                                                               `balance` response",
                                                              f)));
                            }
                        }
                    }
                    return Ok(balance);
                }
            }
            Err(Error::Api("invalid response to `balance` method, no `result` field was found"
                .to_owned()))
        } else {
            Err(Error::Api("invalid response to `balance` method".to_owned()))
        }
    }
}
