use crate::adapter::{
    config::{app_settings::AppSettings, handler::Handler},
    model::appointment_solicited_message::AppointmentSolicitedMessage,
};

pub struct AppointmentSolicitedHandler {
    pub app_settings: AppSettings,
}

impl Handler for AppointmentSolicitedHandler {
    type Message = AppointmentSolicitedMessage;

    fn topic(&self) -> String {
        self.app_settings.appointment_solicited_topic_name.clone()
    }

    fn handle(&self, message: Self::Message) -> Result<(), anyhow::Error> {
        println!("appointment solicited handled!. payload: {:?}", message);
        Ok(())
    }
}
