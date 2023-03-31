use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppointmentSolicitedMessage {
    pub id: String,
    pub start_at: String,
    pub end_at: String,
    pub client_id: String,
    pub barber_id: String,
    pub task_id: String,
}
