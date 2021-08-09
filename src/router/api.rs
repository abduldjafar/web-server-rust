use actix_web::web;
use crate::controller::talent;

pub fn init(cfg: &mut web::ServiceConfig)  {
    cfg.route("/save",web::post().to(talent::add_user));


}