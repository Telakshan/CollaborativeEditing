use actix_files::Files;
use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main()  -> std::io::Result<()> {

    HttpServer::new(|| {
        App::new()
        .service(Files::new("/", "./public").index_file("index.html"))
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
