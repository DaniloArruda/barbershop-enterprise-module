use anyhow::anyhow;

use crate::{
    adapter::{
        config::{app_settings::AppSettings, handler::Handler},
        error::AdapterError,
        model::appointment_solicited_message::AppointmentSolicitedMessage,
    },
    application::usecase::{appointment_solicited::AppointmentSolicitedUseCase, usecase::UseCase},
};

pub struct AppointmentSolicitedHandler {
    pub app_settings: AppSettings,
    pub appointment_solicited_use_case: AppointmentSolicitedUseCase,
}

impl Handler for AppointmentSolicitedHandler {
    type Message = AppointmentSolicitedMessage;

    fn topic(&self) -> String {
        self.app_settings.appointment_solicited_topic_name.clone()
    }

    fn handle(&self, message: Self::Message) -> Result<(), anyhow::Error> {
        let appointment_solicited_request = message.clone().try_into()?;

        let result = self
            .appointment_solicited_use_case
            .execute(appointment_solicited_request)
            .map_err(|error| AdapterError::Application(error))
            .inspect_err(|error| println!("appointment solicited error: {}", error))
            .map_err(|error| anyhow!(error))
            .inspect_err(|error| println!("appointment solicited anyhowerror: {}", error))
            .inspect(|_| println!("appointment solicited handled!. payload: {:?}", message));

        result
    }
}
