extern crate nicehash;

use std::time::Duration;
use std::error::Error;
use std::f64;

use nicehash::{Client, Location, Algorithm};
use nicehash::types::{NewOrder, PoolInfo};

const TEST_API_ID: u64 = 70022;
const TEST_API_KEY: &'static str = "ea454eef-ef74-42da-a2ed-b971bb212718";
const TEST_READ_API_KEY: &'static str = "fd1baeda-e66f-4ebe-aa27-c791ae87ba86";

#[test]
fn it_version_number() {
    Client::new().unwrap();
}

#[test]
fn it_global_stats_current() {
    let client = Client::new().unwrap();

    client.global_stats_current(None).unwrap();
    client.global_stats_current(Some(Location::Europe)).unwrap();
    client.global_stats_current(Some(Location::USA)).unwrap();
}

#[test]
fn it_global_stats_24h() {
    let client = Client::new().unwrap();
    client.global_stats_24h().unwrap();
}

#[test]
fn it_get_orders() {
    let client = Client::new().unwrap();
    client.get_orders(Location::Europe, Algorithm::SHA256).unwrap();
    client.get_orders(Location::USA, Algorithm::DaggerHashimoto).unwrap();
    client.get_orders(Location::Europe, Algorithm::Equihash).unwrap();
    client.get_orders(Location::USA, Algorithm::X15).unwrap();
}

#[test]
fn it_get_buy_info() {
    let client = Client::new().unwrap();
    let info = client.get_buy_info().unwrap();
    assert_eq!(info.get_down_time(), Duration::from_secs(600));
}

#[test]
fn it_get_my_orders() {
    let client = Client::new().unwrap();
    client.get_my_orders(TEST_API_ID,
                       TEST_API_KEY,
                       Location::Europe,
                       Algorithm::SHA256)
        .unwrap();
    client.get_my_orders(TEST_API_ID,
                       TEST_READ_API_KEY,
                       Location::USA,
                       Algorithm::DaggerHashimoto)
        .unwrap();
    client.get_my_orders(TEST_API_ID,
                       TEST_READ_API_KEY,
                       Location::Europe,
                       Algorithm::Equihash)
        .unwrap();
    client.get_my_orders(TEST_API_ID, TEST_API_KEY, Location::USA, Algorithm::X15).unwrap();
}

#[test]
fn it_get_my_orders_errors() {
    let client = Client::new().unwrap();
    assert_eq!("Incorrect key.",
               client.get_my_orders(TEST_API_ID,
                                  "invalid-api-key",
                                  Location::Europe,
                                  Algorithm::SHA256)
                   .err()
                   .unwrap()
                   .description());
    assert_eq!("Incorrect key.",
               client.get_my_orders(TEST_API_ID,
                                  "invalid-api-key",
                                  Location::USA,
                                  Algorithm::Equihash)
                   .err()
                   .unwrap()
                   .description());
    assert_eq!("Incorrect key.",
               client.get_my_orders(99999999999999,
                                  TEST_API_KEY,
                                  Location::Europe,
                                  Algorithm::DaggerHashimoto)
                   .err()
                   .unwrap()
                   .description());
    assert_eq!("Incorrect key.",
               client.get_my_orders(99999999999999,
                                  TEST_READ_API_KEY,
                                  Location::USA,
                                  Algorithm::X11)
                   .err()
                   .unwrap()
                   .description());
}

#[test]
fn it_create_order_errors() {
    let client = Client::new().unwrap();
    assert_eq!("Not enough funds.",
               client.create_order(TEST_API_ID,
                                 TEST_API_KEY,
                                 Location::USA,
                                 NewOrder {
                                     algorithm: Algorithm::Equihash,
                                     amount: 0.34,
                                     price: 0.0985,
                                     limit: Some(1.5),
                                 },
                                 PoolInfo {
                                     host: "my.test.pool".to_owned(),
                                     port: 5650,
                                     username: "TestUser".to_owned(),
                                     password: "test_password".to_owned(),
                                 },
                                 None)
                   .err()
                   .unwrap()
                   .description());
    assert_eq!("Incorrect key.",
               client.create_order(523,
                                 TEST_API_KEY,
                                 Location::USA,
                                 NewOrder {
                                     algorithm: Algorithm::X15,
                                     amount: 0.34,
                                     price: 0.0985,
                                     limit: Some(1.5),
                                 },
                                 PoolInfo {
                                     host: "my.test.pool".to_owned(),
                                     port: 5650,
                                     username: "TestUser".to_owned(),
                                     password: "test_password".to_owned(),
                                 },
                                 None)
                   .err()
                   .unwrap()
                   .description());
    assert_eq!("Invalid amount or price.",
               client.create_order(TEST_API_ID,
                                 TEST_API_KEY,
                                 Location::USA,
                                 NewOrder {
                                     algorithm: Algorithm::SHA256,
                                     amount: 0.0,
                                     price: 0.0985,
                                     limit: None,
                                 },
                                 PoolInfo {
                                     host: "my.test.pool".to_owned(),
                                     port: 5650,
                                     username: "TestUser".to_owned(),
                                     password: "test_password".to_owned(),
                                 },
                                 None)
                   .err()
                   .unwrap()
                   .description());
    assert_eq!("Invalid amount or price.",
               client.create_order(TEST_API_ID,
                                 TEST_API_KEY,
                                 Location::USA,
                                 NewOrder {
                                     algorithm: Algorithm::SHA256,
                                     amount: 0.15,
                                     price: 0.0,
                                     limit: None,
                                 },
                                 PoolInfo {
                                     host: "my.test.pool".to_owned(),
                                     port: 5650,
                                     username: "TestUser".to_owned(),
                                     password: "test_password".to_owned(),
                                 },
                                 None)
                   .err()
                   .unwrap()
                   .description());
    assert_eq!("Not enough funds.",
               client.create_order(TEST_API_ID,
                                 TEST_API_KEY,
                                 Location::USA,
                                 NewOrder {
                                     algorithm: Algorithm::SHA256,
                                     amount: 0.15,
                                     price: 0.095,
                                     limit: None,
                                 },
                                 PoolInfo {
                                     host: "my.test.pool".to_owned(),
                                     port: 5650,
                                     username: "TestUser".to_owned(),
                                     password: "test_password".to_owned(),
                                 },
                                 Some(127))
                   .err()
                   .unwrap()
                   .description());
    assert_eq!("Invalid amount or price.",
               client.create_order(TEST_API_ID,
                                 TEST_API_KEY,
                                 Location::USA,
                                 NewOrder {
                                     algorithm: Algorithm::SHA256,
                                     amount: 0.0,
                                     price: 0.095,
                                     limit: None,
                                 },
                                 PoolInfo {
                                     host: "my.test.pool".to_owned(),
                                     port: 5650,
                                     username: "TestUser".to_owned(),
                                     password: "test_password".to_owned(),
                                 },
                                 Some(127))
                   .err()
                   .unwrap()
                   .description());
    assert_eq!("Invalid amount or price.",
               client.create_order(TEST_API_ID,
                                 TEST_API_KEY,
                                 Location::USA,
                                 NewOrder {
                                     algorithm: Algorithm::SHA256,
                                     amount: 0.15,
                                     price: 0.0,
                                     limit: None,
                                 },
                                 PoolInfo {
                                     host: "my.test.pool".to_owned(),
                                     port: 5650,
                                     username: "TestUser".to_owned(),
                                     password: "test_password".to_owned(),
                                 },
                                 Some(127))
                   .err()
                   .unwrap()
                   .description());
}

#[test]
fn it_refill_order_errors() {
    let client = Client::new().unwrap();
    assert_eq!("Not enough funds.",
               client.refill_order(TEST_API_ID,
                                 TEST_API_KEY,
                                 Location::Europe,
                                 Algorithm::Equihash,
                                 10,
                                 5.0)
                   .err()
                   .unwrap()
                   .description());
    assert_eq!("Invalid amount or order id.",
               client.refill_order(TEST_API_ID,
                                 TEST_API_KEY,
                                 Location::Europe,
                                 Algorithm::Equihash,
                                 0,
                                 5.0)
                   .err()
                   .unwrap()
                   .description());
    assert_eq!("Invalid amount or order id.",
               client.refill_order(TEST_API_ID,
                                 TEST_API_KEY,
                                 Location::USA,
                                 Algorithm::Axiom,
                                 10,
                                 0.0)
                   .err()
                   .unwrap()
                   .description());
    assert_eq!("Incorrect key.",
               client.refill_order(9999999999,
                                 TEST_API_KEY,
                                 Location::Europe,
                                 Algorithm::Qubit,
                                 10,
                                 5.0)
                   .err()
                   .unwrap()
                   .description());
    assert_eq!("Incorrect key.",
               client.refill_order(TEST_API_ID,
                                 "invalid-api-key",
                                 Location::USA,
                                 Algorithm::Quark,
                                 10,
                                 5.0)
                   .err()
                   .unwrap()
                   .description());
}

#[test]
fn it_remove_order_errors() {
    let client = Client::new().unwrap();
    assert_eq!("Unknown order id.",
               client.remove_order(TEST_API_ID,
                                 TEST_API_KEY,
                                 Location::Europe,
                                 Algorithm::Equihash,
                                 0)
                   .err()
                   .unwrap()
                   .description());
    assert_eq!("Incorrect key.",
               client.remove_order(9999999999,
                                 TEST_API_KEY,
                                 Location::Europe,
                                 Algorithm::Qubit,
                                 10)
                   .err()
                   .unwrap()
                   .description());
    assert_eq!("Incorrect key.",
               client.remove_order(TEST_API_ID,
                                 "invalid-api-key",
                                 Location::USA,
                                 Algorithm::Quark,
                                 10)
                   .err()
                   .unwrap()
                   .description());
}

#[test]
fn it_set_order_price_errors() {
    let client = Client::new().unwrap();
    assert_eq!("Order id/price/algo incorrect.",
               client.set_order_price(TEST_API_ID,
                                    TEST_API_KEY,
                                    Location::Europe,
                                    Algorithm::Equihash,
                                    0,
                                    10.3)
                   .err()
                   .unwrap()
                   .description());
    assert_eq!("Price incorrect.",
               client.set_order_price(TEST_API_ID,
                                    TEST_API_KEY,
                                    Location::Europe,
                                    Algorithm::Equihash,
                                    10,
                                    0.0)
                   .err()
                   .unwrap()
                   .description());
    assert_eq!("Order id/price/algo incorrect.",
               client.set_order_price(TEST_API_ID,
                                    TEST_API_KEY,
                                    Location::Europe,
                                    Algorithm::Equihash,
                                    0,
                                    0.0)
                   .err()
                   .unwrap()
                   .description());
    assert_eq!("Order id incorrect.",
               client.set_order_price(TEST_API_ID,
                                    TEST_API_KEY,
                                    Location::Europe,
                                    Algorithm::Equihash,
                                    10,
                                    0.5)
                   .err()
                   .unwrap()
                   .description());
    assert_eq!("Incorrect key.",
               client.set_order_price(9999999999,
                                    TEST_API_KEY,
                                    Location::Europe,
                                    Algorithm::Qubit,
                                    10,
                                    5.5)
                   .err()
                   .unwrap()
                   .description());
    assert_eq!("Incorrect key.",
               client.set_order_price(TEST_API_ID,
                                    "invalid-api-key",
                                    Location::USA,
                                    Algorithm::Quark,
                                    10,
                                    4.3)
                   .err()
                   .unwrap()
                   .description());
}

#[test]
fn it_decrease_order_price_errors() {
    let client = Client::new().unwrap();
    assert_eq!("Order id/price/algo incorrect.",
               client.decrease_order_price(TEST_API_ID,
                                         TEST_API_KEY,
                                         Location::Europe,
                                         Algorithm::Equihash,
                                         0)
                   .err()
                   .unwrap()
                   .description());
    assert_eq!("No such order.",
               client.decrease_order_price(TEST_API_ID,
                                         TEST_API_KEY,
                                         Location::Europe,
                                         Algorithm::Equihash,
                                         10)
                   .err()
                   .unwrap()
                   .description());
    assert_eq!("Incorrect key.",
               client.decrease_order_price(9999999999,
                                         TEST_API_KEY,
                                         Location::Europe,
                                         Algorithm::Qubit,
                                         10)
                   .err()
                   .unwrap()
                   .description());
    assert_eq!("Incorrect key.",
               client.decrease_order_price(TEST_API_ID,
                                         "invalid-api-key",
                                         Location::USA,
                                         Algorithm::Quark,
                                         10)
                   .err()
                   .unwrap()
                   .description());
}

#[test]
fn it_set_order_speed_limit_errors() {
    let client = Client::new().unwrap();
    assert_eq!("Order id/limit/algo incorrect.",
               client.set_order_speed_limit(TEST_API_ID,
                                          TEST_API_KEY,
                                          Location::Europe,
                                          Algorithm::Equihash,
                                          0,
                                          None)
                   .err()
                   .unwrap()
                   .description());
    assert_eq!("Order id/limit/algo incorrect.",
               client.set_order_speed_limit(TEST_API_ID,
                                          TEST_API_KEY,
                                          Location::Europe,
                                          Algorithm::Equihash,
                                          10,
                                          Some(-5.4))
                   .err()
                   .unwrap()
                   .description());
    assert_eq!("Order id incorrect.",
               client.set_order_speed_limit(TEST_API_ID,
                                          TEST_API_KEY,
                                          Location::Europe,
                                          Algorithm::Equihash,
                                          10,
                                          Some(1.1))
                   .err()
                   .unwrap()
                   .description());
    assert_eq!("Incorrect key.",
               client.set_order_speed_limit(9999999999,
                                          TEST_API_KEY,
                                          Location::Europe,
                                          Algorithm::Qubit,
                                          10,
                                          Some(1.5))
                   .err()
                   .unwrap()
                   .description());
    assert_eq!("Incorrect key.",
               client.set_order_speed_limit(TEST_API_ID,
                                          "invalid-api-key",
                                          Location::USA,
                                          Algorithm::Quark,
                                          10,
                                          None)
                   .err()
                   .unwrap()
                   .description());
}

#[test]
fn it_get_balance() {
    let client = Client::new().unwrap();
    let balance = client.get_balance(TEST_API_ID, TEST_API_KEY).unwrap();
    assert!(balance.confirmed < f64::EPSILON && balance.confirmed > f64::EPSILON * -1.0);
    assert!(balance.pending < f64::EPSILON && balance.pending > f64::EPSILON * -1.0);

    assert_eq!("Incorrect key.",
               client.get_balance(99999999, TEST_API_KEY)
                   .err()
                   .unwrap()
                   .description());
    assert_eq!("Incorrect key.",
               client.get_balance(TEST_API_ID, "invalid-api-key")
                   .err()
                   .unwrap()
                   .description());
}
