use actix_web::{web};
use crate::controller;


pub  fn v1(config: &mut web::ServiceConfig) {
     config
         .route("/hey", web::get().to(controller::manual_hello))
         .route("/asoi",web::get().to(controller::hello))
         .route("/data",web::post().to(controller::extract_item))
         .route("/echo",web::post().to(controller::echo));

}
