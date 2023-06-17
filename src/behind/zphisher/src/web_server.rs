use std::path::PathBuf;
use actix_files::NamedFile;
use actix_web::{get, web, App, HttpServer, Responder, HttpRequest};
use crate::cli::error_msg;

#[get("/")]
async fn index(req: HttpRequest, data: web::Data<PathBuf>) -> impl Responder {
    let client_ip = req.connection_info().peer_addr().unwrap_or("<unknown>").to_string();
    println!("Client IP: {}", client_ip);

    let file_path = data.get_ref().join("login.html");
    let file = match NamedFile::open(file_path) {
        Ok(file) => file,
        Err(err) => {
            error_msg(&err.to_string());
            std::process::exit(1);
        }
    };
    file.into_response(&req)
}



pub async fn start_webserver(static_files: PathBuf) -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .service(index)
            .app_data(web::Data::new(static_files.clone()))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
