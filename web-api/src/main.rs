use actix_cors::Cors;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Serialize, Deserialize};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[derive(Deserialize)]
struct QueryParams {
    request_id: i32,
    length: f64,
    height: f64,
    width: f64,
    velocity: f64,
}

async fn get_request(query: web::Query<QueryParams>) -> impl Responder {
    let req = request_generation::create_request(query.request_id, query.length, query.height, query.width, query.velocity);
    let res = generating_plan::create_plan(&req);
    
    HttpResponse::Ok().json(res)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(Cors::default().allow_any_origin())
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
            .route("/generateplan", web::get().to(get_request))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
