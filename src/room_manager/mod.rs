use std::collections::HashMap;
use actix::Addr;
use crate::actor::WebSocket;

pub type RoomId = String;

pub struct RoomData {
    pub rooms: HashMap<RoomId, Vec<Addr<WebSocket>>>,
    pub client_rooms: HashMap<Addr<WebSocket>, RoomId>,
}

impl RoomData {
    pub fn new() -> Self {
        RoomData {
            rooms: HashMap::new(),
            client_rooms: HashMap::new(),
        }
    }
}