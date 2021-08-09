use crate::repository::talent::TalentRepository;


pub struct TalentServiceManager {
     pub repository: TalentRepository,
}

// Api Servie Implementation
impl TalentServiceManager {
    pub fn new(repository: TalentRepository) -> Self {
        TalentServiceManager { repository }
    }
}

// Service Manager constructor
pub struct TalentAppState {
    pub talent_service_manager: TalentServiceManager,
}