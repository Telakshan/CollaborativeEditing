use std::collections::HashSet;
use actix::{Actor, AsyncContext, Context, Handler, };

pub struct WsSessionManager {
    sessions: HashSet<Addr<WebSocket>>,

    pub(crate) last_text: String
}

impl WsSessionManager {

}