#[cfg(test)]
use mockall::automock;
use uuid::Uuid;

use crate::domain::entity::client::Client;

#[cfg_attr(test, automock)]
pub trait ClientRepository {
    fn find_by_id(&self, client_id: Uuid) -> Option<Client>;
}
