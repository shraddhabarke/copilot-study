use crypto_utils::{Crypto, PrimeDiffieHellman};
use std::collections::HashMap;
use std::io::{self, *};
use std::net::{Shutdown, SocketAddr, TcpListener, TcpStream};
use std::sync::mpsc::{channel, Sender};
use std::thread;

const LOCAL: &str = "127.0.0.1:4040";

pub struct EncryptedStream {
    socket: TcpStream,
    crypto: PrimeDiffieHellman,
}

impl EncryptedStream {

    /** Complete the Diffie-Hellman handshake before sending any data. */

    pub fn establish(mut socket: TcpStream) -> io::Result<Self> {
        let mut crypto = PrimeDiffieHellman::new();

        let (priv_key, pubkey) = crypto.generate_keys();
        socket.write_all(&pubkey.to_vec())?;

        let b_bytes = {
            let mut data = [0_u8; 16]; // using 16 byte buffer
            socket.read_exact(&mut data)?;
            data
        };

        let other_pub_key = crypto.deserialize(&b_bytes);
        crypto.handshake(&priv_key, &other_pub_key);
        println!("Handshake complete!");

        Ok(EncryptedStream { socket, crypto })
    }

    /** Close connection with client */

    pub fn close(&mut self) {
        if let Err(e) = self.socket.shutdown(Shutdown::Both) {
            eprintln!("Error shutting down socket: {:?}", e);
        }
    }

    /** Send an encrypted message to the connected client. */

    pub fn send(&mut self, msg: &str) -> io::Result<()> {
        let mut msg_bytes: Vec<u8> = msg.trim().as_bytes().to_vec();
        msg_bytes.push(msg_bytes.len() as u8); // add data length
        let encrypted_msg = self.crypto.encrypt(&msg_bytes);
        self.socket.write_all(&encrypted_msg)?;
        println!("Sent: {}", &msg);
        Ok(())
    }

    /** Clone the tcp stream, this function can be used to generate separate streams for each connected client  */

    pub fn try_clone(&self) -> io::Result<Self> {
        let socket = self.socket.try_clone()?;

        Ok(EncryptedStream {
            socket,
            crypto: self.crypto.clone(),
        })
    }

    /** Receive an encrypted message from the connected client and decrypt it */

    pub fn recv(&mut self) -> io::Result<Option<String>> {
        let raw = Self::receive_raw(&mut self.socket)?;
        let message = self.crypto.decrypt(&raw);
        let txt = std::str::from_utf8(&message)
            .ok()
            .map(str::trim)
            .map(String::from);
        println!("Received: {:?}", &txt);
        Ok(txt)
    }

    fn receive_raw(socket: &mut TcpStream) -> io::Result<Vec<u8>> {
        let mut data = vec![0_u8; 256]; // using 256 byte buffer
        socket.read(&mut data).map(|_| data)
    }
}

enum Message {
    Connected(EncryptedStream),
    Disconnected,
    Text(String),
}

fn accept(channel: Sender<(SocketAddr, Message)>) {
    loop {
        // create a new socket to accept connections
        let socket = match TcpListener::bind(LOCAL) {
            Ok(socket) => socket,
            Err(e) => panic!("could not read start TCP listener: {}", e),
        };

        // TODO1: accept incoming connections and spawn a new thread for each one

    }
}

fn handle_stream(socket: TcpStream, channel: Sender<(SocketAddr, Message)>) -> io::Result<()> {
    let addr = socket.peer_addr()?; // get the address of the client
    // TODO2: establish a new Diffie-Hellman handshake with the client and begin receiving messages

}

struct ClientConnection {
    stream: EncryptedStream,
    username: Option<String>,
}

impl ClientConnection {
    fn send(&mut self, txt: &str) {
        if let Err(e) = self.stream.send(txt) {
            eprintln!("Error sending message to client: {:?}", e);
        }
    }
}

#[derive(Default)]
struct ChatServer {
    clients: HashMap<SocketAddr, ClientConnection>,
}

impl ChatServer {
    pub fn new() -> Self {
        Default::default()
    }

    fn handle_msg(&mut self, addr: SocketAddr, msg: Message) {
        //TODO3: handle incoming messages, prompt for 'enter username'

    }

    pub fn handle_chat_msg(&mut self, addr: SocketAddr, msg: &str) {
        //TODO4: handle the chat commands and send the chat messages to all other clients.
    }

}

fn main() {

    // Create a channel to send messages to the server
    let (send, recv) = channel();
    thread::spawn(move || accept(send));

    let mut server = ChatServer::new();
    while let Ok((addr, msg)) = recv.recv() {
        server.handle_msg(addr, msg)
    }
}
