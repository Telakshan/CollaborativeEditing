mod actor;
mod errors;
mod session_manager;
mod room_manager;

use actix::prelude::*;
use actix_web::{web, App, HttpServer, HttpResponse, HttpRequest, Error };
use actix_web::web::Data;
use actix_web_actors::ws;
use crate::errors::ConnectionError;
use crate::actor::WebSocket;
use crate::session_manager::WsSessionManager;

async fn ws_index(r: HttpRequest, stream: web::Payload, server_instance: web::Data<Addr<WsSessionManager>>)
    -> Result<HttpResponse, Error> {

    let ws_actor: WebSocket = WebSocket {
        manager: server_instance.get_ref().clone(), //provide actor with a ref to the sess. manager
    };

    ws::start(ws_actor, &r, stream)
}

#[actix_web::main]
async fn main()  -> std::io::Result<()> {
    //create and start the WssessionManager Actor
    let manager = WsSessionManager::new().start();
    let address = "0.0.0.0:8000";

    // constructing the Actix web app
    let server = HttpServer::new(move || {

        App::new()
            // WS route
            .route("/ws/", web::get().to(ws_index))

            // serving static file
            .service(actix_files::Files::new("/", "public").index_file("index.html"))

            // share the manager actor across the application
            .app_data(Data::new(manager.clone()))
    })
        .workers(1)
        .bind(address);

    match server {
        Ok(srv) => {
            println!("Starting server on {}", address);
            srv.run().await
        },
        Err(e) => {
            panic!("{}", ConnectionError::CreateServerError(e.to_string()));
        }
    }
}
