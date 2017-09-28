extern crate zmq;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate decimal;

mod trade;
use trade::*;

fn main() {
    println!("Connecting to hello world server...\n");

    let context = zmq::Context::new();
    let requester = context.socket(zmq::REQ).unwrap();

    assert!(requester.connect("tcp://localhost:5559").is_ok());

    let mut msg = zmq::Message::new().unwrap();
    let mut req = RequestField {
        id: 0,
        request: AnyRequest::Order(OrderReq {
            account: Account { id: "003600".to_string() },
            security: SecurityUuid { exchange_id: ExchangeId::SHFE, security_id: "IF1711".to_string() },
            direction: Direction::Buy,
            price: d128!(11.32),
            volume: 100
        })
    };

    for request_nbr in 0..5 {
        println!("Sending Request {}...", request_nbr);
        req.id = request_nbr;
        requester.send(serde_json::to_string(&req).unwrap().as_bytes(), 0).unwrap();

        requester.recv(&mut msg, 0).unwrap();
        println!("Received Response {}: {}", msg.as_str().unwrap(), request_nbr);
    }
}