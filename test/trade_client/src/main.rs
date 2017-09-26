extern crate zmq;

fn main() {
    println!("Connecting to hello world server...\n");

    let context = zmq::Context::new();
    let requester = context.socket(zmq::REQ).unwrap();

    assert!(requester.connect("tcp://localhost:5559").is_ok());

    let mut msg = zmq::Message::new().unwrap();

    let data =
        r#"{
            "id": 10,
            "request": {
                "Order": {
                    "account": {
                        "id": "003600"
                    },
                    "security": {
                        "sec_type": "futures",
                        "sec_id": "IF1704"
                    },
                    "direction": "Buy",
                    "price": "11.32",
                    "volume": 100
                }
            }
        }"#;
    for request_nbr in 0..10 {
        println!("Sending Hello {}...", request_nbr);
        requester.send(data.as_bytes(), 0).unwrap();

        requester.recv(&mut msg, 0).unwrap();
        println!("Received World {}: {}", msg.as_str().unwrap(), request_nbr);
    }
}