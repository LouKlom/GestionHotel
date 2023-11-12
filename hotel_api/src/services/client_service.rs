// src/services/client_service.rs

use crate::entities::client_entity::ClientEntity;
use crate::repositories::client_repository::ClientRepository;

pub struct ClientService;

impl ClientService {
    pub fn create_client(name: &str, email: &str, phone_number: &str) -> bool {


        unimplemented();
    }
}