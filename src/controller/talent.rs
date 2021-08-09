use actix_web::{ web, HttpResponse, Responder};
use crate::models::talent::TalentData;
use crate::services::talent::TalentAppState;

pub async fn add_user(app_data: web::Data<TalentAppState>, data: web::Json<TalentData>) -> impl Responder {
    let action_talent =  app_data.talent_service_manager.repository.create(&data);
    let result = web::block(move || action_talent).await;
    match result {
        Ok(result) => HttpResponse::Ok().json(result.inserted_id),
        Err(e) => {
            println!("Error while getting, {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}