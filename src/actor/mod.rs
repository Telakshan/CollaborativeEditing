use actix::{ Actor, Addr, AsyncContext, Handler, Message, Running, StreamHandler };
use actix_web_actors::ws;
use crate::session_manager::WsSessionManager;

//Message structs for WebSocket Events

//This message is used to notify the WsSessionManager when a new client connects
//addr - address of the websocket actor representing the new connection
#[derive(Message) ]
#[rtype(result = "()")]
pub struct Connect {
    pub(crate) addr: Addr<WebSocket>
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub(crate) addr: Addr<WebSocket>
}

//a message to be broadcast to all connected ws clients
//msg = the content that will be sent
//sender = address of the ws actor that sent the msg
#[derive(Message, Clone)] //Clone is derived to allow easy duplication of the  messages for easy broadcasting
#[rtype(result = "()")]
pub struct BroadcastMessage {
    pub(crate) msg: String,
    pub(crate) sender: Addr<WebSocket>
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct DefaultMessage {
    pub text: String,
}

impl Handler<DefaultMessage> for WebSocket {
    type Result = ();
    fn handle(&mut self, msg: DefaultMessage, ctx: &mut Self::Context) {
        ctx.text(msg.text);
    }
}

impl Handler<BroadcastMessage> for WebSocket {
    type Result = ();
    fn handle(&mut self, msg: BroadcastMessage, ctx: &mut Self::Context) {
        ctx.text(msg.msg);
    }
}

pub struct WebSocket {
    pub(crate) manager: Addr<WsSessionManager>,
}

//Implementing actor trait, so websocket can be used as  an Actix Actor
impl Actor for WebSocket {
    type Context = ws::WebsocketContext<Self>; //define the type if context used by the websocket actor
    fn started(&mut self, ctx: &mut Self::Context) {
        self.manager.do_send(Connect {
            addr: ctx.address(), //Send the address of this WS actor to the session manager
        });
    }

    fn stopping(&mut self, ctx: &mut Self::Context) -> Running {
        self.manager.do_send(Disconnect {
            addr: ctx.address()
        });

        Running::Stop //Indicate to the actor that the runtime should stop
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        // check if the message is a valid text message
        if let Ok(ws::Message::Text(text)) = msg {
            self.manager.do_send(BroadcastMessage {
                msg: text.to_string(),
                sender: ctx.address(),
            });
        }
    }
}