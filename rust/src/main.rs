use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, middleware};
use serde::{Deserialize, Serialize};
use env_logger;

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
    HttpResponse::Ok().json(Info {
	id: id,
	age: age,
	value: String::from("あいうえおテスト"),
	flag: false,
    })
}

// curl -X POST -H "Content-Type: application/json" -d '{"id", 100, "value": "hoge", "flag": true}' http://127.0.0.1:8080/info
#[post("/info")]
async fn post_info(info: web::Json<Info>) -> impl Responder {
    println!("post_info");
    println!("{:?}", info);
    HttpResponse::Ok().body("OK")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    HttpServer::new(
	|| App::new()
	    .wrap(middleware::Logger::default())
	    .service(get_info).service(post_info)
    )
	.bind("0.0.0.0:8000")?
	.run()
	.await
}
