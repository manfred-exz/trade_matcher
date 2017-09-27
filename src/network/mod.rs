use std;
use super::zmq;
use super::serde_json;


pub use zmq::{SocketType, Error};
pub use trade::RequestField;

pub struct JsonSocket {
    ctx: zmq::Context,
}

impl JsonSocket {
    pub fn new() -> JsonSocket {
        let _ctx = zmq::Context::new();
        JsonSocket {
            ctx: _ctx,
        }
    }

    /// start the loop to listen for data. if data is a valid json Value,
    /// and can be deserialized to RequestField, then request_handler will
    /// be called on it.
    pub fn listen_request(&self, endpoint: &str, tx: std::sync::mpsc::Sender<RequestField>)
    {
        let socket = self.ctx.socket(SocketType::REP).expect("cannot create socket");
        socket.bind(endpoint).expect("cannot bind");
        loop {
            let msg = socket.recv_msg(0);
            // process raw zmq::Message
            match msg {
                Ok(msg) => {
                    let request = serde_json::from_str::<RequestField>(msg.as_str().unwrap());
                    // process json
                    match request {
                        Ok(req) => tx.send(req).unwrap(),
                        Err(_) => println!("[WARN] not a valid request")
                    }
                    socket.send(b"ack", 0).unwrap();
                },
                Err(err) => println!("[ERROR] {}", err)
            }
        }
    }
}