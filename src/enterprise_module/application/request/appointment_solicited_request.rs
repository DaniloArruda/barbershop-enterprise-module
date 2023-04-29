use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct AppointmentSolicitedRequest {
    pub id: Uuid,
    pub start_at: DateTime<Utc>,
    pub end_at: DateTime<Utc>,
    pub client_id: Uuid,
    pub barber_id: Uuid,
    pub task_id: Uuid,
}
