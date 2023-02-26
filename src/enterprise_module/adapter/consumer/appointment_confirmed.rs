use crate::adapter::{
    config::{app_settings::AppSettings, handler::Handler},
    model::appointment_confirmed_message::AppointmentConfirmedMessage,
};

pub struct AppointmentConfirmedHandler {
    pub app_settings: AppSettings,
}

impl Handler for AppointmentConfirmedHandler {
    type Message = AppointmentConfirmedMessage;

    fn topic(&self) -> String {
        self.app_settings.appointment_confirmed_topic_name.clone()
    }

    fn handle(&self, message: Self::Message) -> Result<(), anyhow::Error> {
        println!("appointment confirmed handled!. payload: {:?}", message);
        Ok(())
    }
}
