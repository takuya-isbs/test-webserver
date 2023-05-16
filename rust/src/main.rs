use std::path::PathBuf;
use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader};

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, middleware};
use serde::{Deserialize, Serialize};

/// from https://docs.rs/clap/latest/clap/_derive/_tutorial/index.html

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Listen address
    #[arg(short, long, value_name = "ADDR",
	  default_value = "0.0.0.0"
    )]
    listen: String,

    /// Port number
    #[arg(short, long, value_name = "PORT",
	  default_value_t = 80,
	  value_parser = clap::value_parser!(u16).range(1..)
    )]
    port: u16,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,
}

#[derive(Debug, Serialize, Deserialize)]
struct Info {
    id: u32,
    age: Option<u32>,
    value: String,
    flag: bool,
}

// for acix_web 3.3.2
//async fn get_info(web::Path(id): web::Path<u32>) -> impl Responder {

#[get("/info/{id}")]
async fn get_info(path: web::Path<u32>) -> impl Responder {
    println!("get_info");
    let id = path.into_inner();
    let age: Option<u32> = Some(id);
    let flag: bool = false;
    HttpResponse::Ok().json(Info {
	id,
	age,
	value: String::from("あいうえおテスト"),
	flag,
    })
}

// curl -X POST -H "Content-Type: application/json" -d '{"id", 100, "value": "hoge", "flag": true}' http://127.0.0.1:8080/info
#[post("/info")]
async fn post_info(info: web::Json<Info>) -> impl Responder {
    println!("post_info");
    println!("{:?}", info);
    //HttpResponse::Ok().body("OK")
    HttpResponse::Ok().json(info)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let cli = Cli::parse();

    let listen_addr = cli.listen;
    let port = cli.port;
    let bind_addr = listen_addr + ":" + &port.to_string();
    println!("bind_addr: {bind_addr}");

    let config_file = cli.config.as_deref();
    if let Some(config_file) = config_file {
	println!("config file: {}", config_file.display());

	let file = File::open(config_file)?;
	let reader = BufReader::new(file);
	for line in reader.lines() {
	    let line = line?;
	    let columns: Vec<&str> = line.split(' ').collect();
	    if let Some(second_column) = columns.get(1) {
		println!("{}", second_column);
	    }
	}
    } else {
	println!("no config file")
    }

    match cli.debug {
	0 => println!("no -d"),
	1 => println!("use -d"),
	2 => println!("use -dd"),
	_ => println!("use -ddd"),
    }

    HttpServer::new(
	|| App::new()
	    .wrap(middleware::Logger::default())
	    .service(get_info).service(post_info)
    )
	.bind(bind_addr)?
	.run()
	.await
}

// from https://actix.rs/docs/testing/

#[cfg(test)]
mod tests {
    use actix_web::{http::header::ContentType, test, App};

    use super::*;

    #[actix_web::test]
    async fn test_get() {
	let app = test::init_service(
	    App::new().service(get_info)).await;
	let req = test::TestRequest::get().uri("/info/1").to_request();
	let resp: Info = test::call_and_read_body_json(&app, req).await;
	assert_eq!(resp.id, 1);
	assert_eq!(resp.age, Some(1));
	assert_eq!(resp.value, String::from("あいうえおテスト"))
    }

    #[actix_web::test]
    async fn test_post() {
	let app = test::init_service(
	    App::new().service(post_info)).await;
	let payload = r#"{"id": 12345, "age": 67890, "value": "testvalue", "flag": true}"#.as_bytes();
	let req = test::TestRequest::post()
	    .uri("/info")
	    .insert_header(ContentType::json())
	    .set_payload(payload)
	    .to_request();
	let resp: Info = test::call_and_read_body_json(&app, req).await;
	assert_eq!(resp.id, 12345);
	assert_eq!(resp.age, Some(67890));
    }

    #[test]
    async fn verify_cli() {
	use clap::CommandFactory;
	Cli::command().debug_assert()
    }
}
