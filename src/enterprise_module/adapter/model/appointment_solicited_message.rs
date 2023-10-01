use std::str::FromStr;

use chrono::{DateTime, ParseError};
use serde::Deserialize;
use uuid::Uuid;

use crate::{application::request::appointment_solicited_request::AppointmentSolicitedRequest, adapter::error::AdapterError};

#[derive(Debug, Deserialize, Clone)]
pub struct AppointmentSolicitedMessage {
    pub id: String,
    pub start_at: String,
    pub end_at: String,
    pub client_id: String,
    pub barber_id: String,
    pub task_id: String,
}

impl TryInto<AppointmentSolicitedRequest> for AppointmentSolicitedMessage {
    type Error = AdapterError;

    fn try_into(self) -> Result<AppointmentSolicitedRequest, Self::Error> {
        Ok(AppointmentSolicitedRequest {
            id: Uuid::from_str(&self.id).map_err(|error| AdapterError::Converter { cause: error.to_string() })?,
            barber_id: Uuid::from_str(&self.barber_id).map_err(|error| AdapterError::Converter { cause: error.to_string() })?,
            client_id: Uuid::from_str(&self.client_id).map_err(|error| AdapterError::Converter { cause: error.to_string() })?,
            task_id: Uuid::from_str(&self.task_id).map_err(|error| AdapterError::Converter { cause: error.to_string() })?,
            start_at: DateTime::from_str(&self.start_at).map_err(|error: ParseError| AdapterError::Converter { cause: error.to_string() })?,
            end_at: DateTime::from_str(&self.end_at).map_err(|error: ParseError| AdapterError::Converter { cause: error.to_string() })?,
        })
    }
}
