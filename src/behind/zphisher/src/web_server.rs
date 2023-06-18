use std::path::PathBuf;
use actix_files::NamedFile;
use actix_web::{get, web, App, HttpServer, Responder, HttpRequest, HttpResponse, middleware};
use crate::cli::error_msg;
use actix_cors::Cors;
use actix_web::http::header;
use actix_web::http::header::HeaderValue;


#[get("/")]
async fn index(req: HttpRequest, data: web::Data<PathBuf>) -> impl Responder {
    let client_ip = req.connection_info().peer_addr().unwrap_or("<unknown>").to_string();
    println!("Client IP: {}", client_ip);

    let user_agent = match req.headers().get(header::USER_AGENT) {
        Some(user_agent) => match user_agent.to_str() {
            Ok(user_agent) => user_agent,
            Err(err) => {
                error_msg(&err.to_string());
                std::process::exit(1);
            }
        },
        None => "",
    };


    let file_name = if is_mobile(user_agent) {
        if !data.get_ref().join("mobile.html").exists() {
            "login.html"
        } else {
            "mobile.html"
        }
    } else { "login.html" };

    let file_path = data.get_ref().join(file_name);
    let file = match NamedFile::open(file_path) {
        Ok(file) => file,
        Err(err) => {
            error_msg(&err.to_string());
            std::process::exit(1);
        }
    };
    let mut response = file.into_response(&req);
    response.headers_mut().insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("text/html"),
    );
    response.headers_mut().insert(
        header::CACHE_CONTROL,
        HeaderValue::from_static("no-store"),
    );
    response
}


#[get("/favicon.{ext}")]
async fn favicon(ext: web::Path<String>, data: web::Data<PathBuf>, req: HttpRequest) -> Result<HttpResponse, actix_web::Error> {
    let favicon_path = data.get_ref().join(format!("favicon.{}", ext.into_inner()));
    let file = NamedFile::open(favicon_path)?;
    let mut response = file.into_response(&req);
    response.headers_mut().insert(
        header::CACHE_CONTROL,
        HeaderValue::from_static("no-store"),
    );
    Ok(response)
}

fn is_mobile(user_agent: &str) -> bool {
    user_agent.contains("Mobile") || user_agent.contains("Android") || user_agent.contains("iPhone")
}

pub async fn start_webserver(static_files: PathBuf) -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new().wrap(middleware::Compress::default()).wrap(
            Cors::default().allow_any_origin().allow_any_method().allow_any_header()
        ).service(index).service(favicon).service(actix_files::Files::new("/", static_files.clone())).app_data(web::Data::new(static_files.clone()))
    }).bind(("127.0.0.1", 8080))?.run().await
}
