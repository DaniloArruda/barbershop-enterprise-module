use uuid::Uuid;

use crate::domain::value_object::{email::Email, name::Name};

#[derive(Debug, Clone)]
pub struct Client {
    pub id: Uuid,
    pub name: Name,
    pub email: Email,
}
