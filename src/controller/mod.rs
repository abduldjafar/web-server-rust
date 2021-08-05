use actix_web::{web,HttpResponse,Responder,HttpRequest};
use crate::entity;

pub async  fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

pub async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")

}
pub async fn extract_item(item: web::Json<entity::Talent>, req: HttpRequest) -> HttpResponse {
    println!("request: {:?}", req);
    println!("model: {:?}", item);

    HttpResponse::Ok().json(item.into_inner()) // <- send json response
}