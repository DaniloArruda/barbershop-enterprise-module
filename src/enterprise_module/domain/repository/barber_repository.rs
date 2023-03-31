#[cfg(test)]
use mockall::automock;
use uuid::Uuid;

#[cfg_attr(test, automock)]
pub trait BarberRepository {
    fn is_barber_busy(&self, barber_id: Uuid) -> bool;
}
