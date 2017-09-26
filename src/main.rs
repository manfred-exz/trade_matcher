#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate decimal;
extern crate zmq;

use serde_json::{Value, Error};
use std::str::FromStr;
use decimal::d128;

type SecurityId = String;
type Volume = u32;
type Price = d128;

#[derive(Serialize, Deserialize, Debug)]
enum ExchangeId {
    SH, SZ, SHFE, ZCE, CFFEX, DCE
}

#[derive(Serialize, Deserialize, Debug)]
enum Direction {
    Buy, Sell
}

#[derive(Serialize, Deserialize, Debug)]
struct SecurityUuid {
    exchange_id: ExchangeId,
    security_id: SecurityId
}

#[derive(Serialize, Deserialize, Debug)]
struct Account {
    id: String
}

#[derive(Serialize, Deserialize, Debug)]
struct OrderReq {
    account: Account,
    security: SecurityUuid,
    direction: Direction,
    price: Price,
    volume: Volume,
}

#[derive(Serialize, Deserialize, Debug)]
enum AnyRequest {
    Order(OrderReq),
}

#[derive(Serialize, Deserialize, Debug)]
struct RequestField {
    id: u32,
    request: AnyRequest
}

fn main() {
    let handle = std::thread::spawn(start_order_listener);
    handle.join();
}

fn start_order_listener() {
    let ctx = zmq::Context::new();
    let mut responder = ctx.socket(zmq::REP).unwrap();

    assert!(responder.bind("tcp://*:5559").is_ok());

    let mut msg = zmq::Message::new().unwrap();
    loop {
        let recv_data = responder.recv(&mut msg, 0);
        // process raw zmq::Message
        match recv_data {
            Ok(_) => {
                let request: Result<RequestField, Error> = serde_json::from_str(msg.as_str().unwrap());
                // process json
                match request {
                    Ok(req) => { eprintln!("request = {:?}", req); }
                    Err(_) => { println!("not a valid request") }
                }
//                std::thread::sleep(std::time::Duration::from_millis(1000));
                responder.send(b"ack", 0).unwrap();
            },
            Err(err) => { println!("ZMQErr {}", err) }
        }
    }
}

fn parse_json_request(json: &serde_json::Value) {

}

