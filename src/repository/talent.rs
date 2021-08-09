use bson::{doc, Document};
use mongodb::results::{ InsertOneResult};
use mongodb::{error::Error, Collection};

use crate::models::talent::TalentData;

#[derive(Clone)]
pub struct TalentRepository {
    collection: Collection,
}

impl TalentRepository{
   pub fn new(collection: Collection) -> TalentRepository {
        TalentRepository { collection }
    }
    pub fn talent_data_to_document(data: &TalentData) -> Document {
        let TalentData {
            email,
            name,
            username,
            address,
            body_size
        } = data;
        doc! {
            "email": email,
            "name": name,
            "username":username,
            "address":address,
            "body_size":{
                "height":body_size.height,
                "bust":body_size.bust,
                "waist":body_size.waist
            }

        }
    }
    pub fn create(&self, _data:&TalentData) -> Result<InsertOneResult, Error>{
        self.collection.insert_one(TalentRepository::talent_data_to_document(_data), None)
    }
}