// Estructure data for DB
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TalentData {
    pub email : String,
    pub name: String,
    pub username: String,
    pub address: String
}
#[derive(Debug, Serialize, Deserialize)]
#[derive(Clone)]
pub struct TalentBodySize {
    pub height: i32,
    pub bust: i32,
    pub waist: i32
}