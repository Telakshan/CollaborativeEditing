use actix::{ Actor, Addr, AsyncContext, Context, Handler, Message, Running, StreamHandler };
use actix_web_actors::ws;
use crate::ws_session_manager::WsSessionManager;

//Message structs for WebSocket Events

//This message is used to notify the WsSessionManager when a new client connects
//addr - address of the websocket actor representing the new connection
#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {
    pub(crate) addr: Addr<WebSocket>
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {
    pub(crate) addr: Addr<WebSocket>
}