use ws::{connect, listen, Handler, Sender, Handshake, Result, Message, CloseCode, Error};
use version;

pub struct Server {
    pub out: Sender,
}

pub struct Client {
    pub out: Sender,
}

impl Handler for Server {

    fn on_message(&mut self, msg: Message) -> Result<()> {
        // TODO: Allow client to send WS commands to spirc. Currently just broadcast the message back.
        //self.broadcast_message(Message::from("Testing"));
        self.out.broadcast(msg)
        //self.out.send(msg)
    }
    
    fn on_open(&mut self, _: Handshake) -> Result<()> {
        // We have a new connection, say hello
        self.out.send(format!("WebSockets Server for librespot {} ({}). Built on {}.",
             version::short_sha(),
             version::commit_date(),
             version::short_now()))
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        // The WebSocket protocol allows for a utf8 reason for the closing state after the
        // close code. WS-RS will attempt to interpret this data as a utf8 description of the
        // reason for closing the connection. I many cases, `reason` will be an empty string.
        // So, you may not normally want to display `reason` to the user,
        // but let's assume that we know that `reason` is human-readable.
        match code {
            CloseCode::Normal => println!("Client closed the connection."),
            CloseCode::Away   => println!("Client dropped the connection."),
            _ => println!("Error connecting to client: {}", reason),
        }
    }
    
    fn on_error(&mut self, err: Error) {
        println!("The server encountered an error: {:?}", err);
    }
}

impl Server {
    pub fn broadcast_message(&mut self, msg: Message) -> Result<()> {
        //self is a mutable reference to the Server struct containing the out variable from below.
        self.out.broadcast(msg)
    }
}

pub fn broadcast_message(msg: Message) -> Result<()> {
    //self is a mutable reference to the Server struct containing the out variable from below.
    connect("ws://127.0.0.1:3012", |out| {
        out.broadcast(msg.clone()).unwrap();

        move |msg| {
            out.close(CloseCode::Normal)
        }
    })
 }


pub fn setup_websockets() {
    listen("127.0.0.1:3012", |out| {
        Server {
            out: out,
        }
    }).unwrap();
}