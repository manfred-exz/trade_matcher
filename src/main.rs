#[macro_use]
extern crate serde_derive;
extern crate decimal;

extern crate serde_json;
extern crate zmq;
extern crate crossbeam;

mod trade;
mod network;

use std::sync::mpsc;
use network::*;

fn main() {
    let (tx, rx) = mpsc::channel::<trade::RequestField>();

    let request_thread = std::thread::spawn(move ||{
        let json_socket = JsonSocket::new();
        json_socket.listen_request("tcp://*:5559", tx);
    });

    let handle_thread = std::thread::spawn(move ||{
        for i in 0..5 {
            let req = rx.recv().unwrap();
            println!("{:?}", req);
        }
    });

    request_thread.join();
    handle_thread.join();
}
