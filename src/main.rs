use std::net::TcpStream;
use std::{net::TcpListener, thread::spawn};
use std::sync::mpsc::*;
use tungstenite::{
    accept_hdr,
    handshake::server::{Request, Response},
};

fn main() {
    env_logger::init();
    let mut counter = 0;
    let server = TcpListener::bind("0.0.0.0:8080").unwrap();
    //let (tx, rx) = channel::<i32>();
    for stream in server.incoming() {
        //let tx = tx.clone();
        spawn(move || {
            match stream {
                Ok(stream) => {
                    handle_connection(stream, &mut counter);
                },
                Err(e) => {
                    eprintln!("Error: {}", e);
                },
            }
        });
    }
}

fn new_connection_info(req: &Request) {
    println!("new connection: {}", req.uri().path());
}

fn print_message(msg: &tungstenite::Message, counter: i32) {
    match msg {
        tungstenite::Message::Text(t) => {
            let txt: String = String::from(t.clone().as_bytes().iter().map(|c| {
                if *c == b'\n' {
                    ' '
                } else {
                    char::from(*c)
                }
            }).collect::<String>());
            println!("[↘︎({})] {}", counter, txt);
        }
        tungstenite::Message::Binary(_) => println!("[↘︎] binary message"),
        _ => {}
    }
}

fn handle_connection(stream: TcpStream, counter: &mut i32) {
    let mut websocket = accept_hdr(stream, |req: &Request, mut response: Response| {
        response.headers_mut().insert("signature","ArtiSocketSrv".parse().unwrap());
        new_connection_info(req);
        Ok(response)
    }).unwrap();

    loop {
        let msg = websocket.read().unwrap();
        if msg.is_binary() || msg.is_text() {
            websocket.send(msg.clone()).unwrap();
            *counter = *counter + 1;
            print_message(&msg, *counter);
        }
    }

}