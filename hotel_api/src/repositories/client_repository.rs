// src/repositories/client_repository.rs


use crate::entities::client_entities::ClientEntity;

pub struct ClientRepository;

impl ClientRepository {
    pub fn find_by_id(id: u64) -> Option<ClientEntity> {
        // Recherche d'un client par son ID

        unimplemented();
    }


    pub fn save(client: ClientEntity) -> bool {
        // enregistrer un client dans la BDD

        unimplemented();
    }
}