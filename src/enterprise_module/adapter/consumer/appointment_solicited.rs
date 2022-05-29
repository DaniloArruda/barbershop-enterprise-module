use crate::adapter::config::{app_settings::AppSettings, handler::Handler};

pub struct AppointmentSolicitedHandler {
    pub app_settings: AppSettings,
}

impl Handler for AppointmentSolicitedHandler {
    fn topic(&self) -> String {
        self.app_settings.appointment_solicited_topic_name.clone()
    }

    fn handle(&self) -> Result<(), anyhow::Error> {
        println!("appointment solicited handled!");
        Ok(())
    }
}
