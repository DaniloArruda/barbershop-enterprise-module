use uuid::Uuid;

pub struct AppointmentCreatedEvent {
    pub appointment_id: Uuid,
}
