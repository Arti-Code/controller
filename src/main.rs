use std::{net::TcpListener, thread::spawn};

use tungstenite::{
    accept_hdr,
    handshake::server::{Request, Response},
};

fn main() {
    env_logger::init();
    let server = TcpListener::bind("0.0.0.0:8080").unwrap();
    for stream in server.incoming() {
        spawn(move || -> ! {
            let callback = |req: &Request, mut response: Response| {
                response.headers_mut().insert(
                    "signature",
                    "ArtiSocketSrv".parse().unwrap(),
                );
                new_connection_info(req);
                Ok(response)
            };
            let mut websocket = accept_hdr(stream.unwrap(), callback).unwrap();

            loop {
                let msg = websocket.read().unwrap();
                if msg.is_binary() || msg.is_text() {
                    websocket.send(msg.clone()).unwrap();
                    print_message(&msg);
                }
            }
        });
    }
}

fn new_connection_info(req: &Request) {
    println!("new connection: {}", req.uri().path());
}

fn print_message(msg: &tungstenite::Message) {
    match msg {
        tungstenite::Message::Text(t) => {
            let txt: String = String::from(t.clone().as_bytes().iter().map(|c| {
                if *c == b'\n' {
                    ' '
                } else {
                    char::from(*c)
                }
            }).collect::<String>());
            println!("[↘︎] {}", txt);
        }
        tungstenite::Message::Binary(_) => println!("[↘︎] binary message"),
        _ => {}
    }
}