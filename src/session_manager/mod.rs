use std::collections::HashSet;
use actix::{Actor, Addr, Context, Handler};
use crate::actor::{ Connect, Disconnect, WebSocket, BroadcastMessage };

pub struct WsSessionManager {
    //A set to track the addresses of connected websocket actors
    sessions: HashSet<Addr<WebSocket>>,

    //Stores the last text message broadcast to clients
    pub(crate) last_text: String
}

impl WsSessionManager {
    pub(crate) fn new() -> Self {
        let default_str = "{\"ops\":[{\"insert\":\"Write \"},{\"attributes\":{\"underline\":true},\
        \"insert\":\"here\"},{\"insert\":\" some \"},{\"attributes\":{\"bold\":true},\
        \"insert\":\"text\"},{\"insert\":\"!\"}]}";

        Self{
            sessions: HashSet::new(), //Start with empty set of sessions
            last_text: String::from(default_str),
        }
    }
}

impl Actor for WsSessionManager {
    type Context = Context<Self>; //standard context type for Actix actors
}

impl Handler<Connect> for WsSessionManager {
    type Result = (); //no specific result is returned after handling the msg
    fn handle(&mut self, msg: Connect, _: &mut Self::Context) {
        println!("New Client Connected!");

        let addr = msg.addr;

        self.sessions.insert(addr.clone());
    }
}

impl Handler<Disconnect> for WsSessionManager {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Self::Context) {
        println!("Client Disconnected!");

        self.sessions.remove(&msg.addr);
    }
}

impl Handler<BroadcastMessage> for WsSessionManager {
    type Result = ();

    fn handle(&mut self, msg: BroadcastMessage, _: &mut Self::Context) {
        self.last_text = msg.msg.clone();

        for addr in self.sessions.iter() {
            if *addr != msg.sender {
                let msg_clone = msg.clone();

                addr.do_send(msg_clone);
            }
        }
    }

}