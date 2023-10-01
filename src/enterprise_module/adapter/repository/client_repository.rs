use uuid::Uuid;

use crate::{
    application::repository::client_repository::ClientRepository,
    domain::{
        entity::client::Client,
        value_object::{email::Email, name::Name},
    },
};

pub struct ClientRepositoryPostgres {}

impl ClientRepository for ClientRepositoryPostgres {
    fn find_by_id(&self, client_id: Uuid) -> Option<Client> {
        Some(Client {
            id: client_id,
            name: Name::new("Danilo".to_string(), "Arruda".to_string()).unwrap(),
            email: Email::new("danilo@arruda.com".to_string()).unwrap(),
        })
    }
}
