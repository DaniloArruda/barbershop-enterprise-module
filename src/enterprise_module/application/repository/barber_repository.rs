#[cfg(test)]
use mockall::automock;
use uuid::Uuid;

use crate::domain::entity::barber::Barber;

#[cfg_attr(test, automock)]
pub trait BarberRepository {
    fn is_barber_busy(&self, barber_id: Uuid) -> bool;
    fn find_by_id(&self, barber_id: Uuid) -> Option<Barber>;
}
