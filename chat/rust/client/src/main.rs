use std::net::TcpStream;
use std::io::{Write};
use std::time::{Duration};
use std::thread;
pub use bp256::r1::BrainpoolP256r1;
extern crate openssl;
extern crate rustc_serialize;
extern crate rand;
use crypto_utils::Crypto;
use std::io::*;
const LOCAL: &str = "127.0.0.1:6000";

pub struct Chat {
    socket: TcpStream,
    crypto: crypto_utils::PrimeDiffieHellman
}

impl Chat {

    pub fn new(socket: TcpStream) -> Chat {
        Chat { 
            socket,
            crypto: crypto_utils::PrimeDiffieHellman::new()
        }
    }

    pub fn receive_message(&mut self) -> [u8; 16] {
        let mut data = [0 as u8; 16]; // using 16 byte buffer
        match self.socket.read(&mut data) {
            Ok(_) => {
                let text = String::from_utf8(data.to_vec());
                //println!("Client Received: {}", &text.unwrap());
                return data;
            },
            Err(e) => {
                println!("Failed to receive data: {}", e);
                return data;
            }
        }
    }
    
    pub fn receive(&mut self) {
        let mut data = [0 as u8; 16]; // using 6 byte buffer
        match self.socket.read(&mut data) {
            Ok(_) => {
                let message = self.crypto.decrypt(&data);
                let mut line = String::new();
                match std::io::stdin().read_line(&mut line).unwrap() {
                    0 => assert!(true),
                    _ => {
                        self.send(line);
                    }
                }
            },
            Err(e) => {
                println!("Failed to receive data: {}", e);
            }
        }
    }

    pub fn send(&mut self, line: String) {
        let msg = line.trim().to_string();
        println!("Parsed from stdin {}", msg);
        let msg_bytes = msg.as_bytes();
        let encrypted_msg = self.crypto.encrypt(msg_bytes);
        self.socket.write(&encrypted_msg).unwrap();
        return;
    }

    pub fn dh_handshake(&mut self){
        let ga_wire = self.receive_message();
        let ga = self.crypto.deserialize(&ga_wire);
        let (mut priv_key, pubkey) = self.crypto.generate_keys();
        self.socket.write(&pubkey.to_vec()).unwrap();
        self.crypto.handshake(&mut priv_key, ga);
    }

}

fn main() {
    let stream = TcpStream::connect(LOCAL).expect("Could not connect to server");
    let mut client = Chat::new(stream.try_clone().unwrap());

    let delay = Duration::from_millis(1000);
    loop {
        client.dh_handshake();
        client.receive();
        thread::sleep(delay);
    }
}