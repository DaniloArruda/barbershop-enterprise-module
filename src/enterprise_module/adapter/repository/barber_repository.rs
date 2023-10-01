use uuid::Uuid;

use crate::{
    application::repository::barber_repository::BarberRepository,
    domain::{
        entity::barber::Barber,
        value_object::{email::Email, name::Name},
    },
};

pub struct BarberRepositoryPostgres {}

impl BarberRepository for BarberRepositoryPostgres {
    fn is_barber_busy(&self, _barber_id: Uuid) -> bool {
        false
    }

    fn find_by_id(&self, barber_id: Uuid) -> Option<Barber> {
        Some(Barber {
            id: barber_id,
            name: Name::new("Danilo".to_string(), "Arruda".to_string()).unwrap(),
            email: Email::new("danilo@email.com".to_string()).unwrap(),
        })
    }
}
