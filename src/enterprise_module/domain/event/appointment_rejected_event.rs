use uuid::Uuid;

pub struct AppointmentRejectedEvent {
    pub appointment_id: Uuid,
}
