use crate::adapter::config::{app_settings::AppSettings, handler::Handler};

pub struct AppointmentConfirmedHandler {
    pub app_settings: AppSettings,
}

impl Handler for AppointmentConfirmedHandler {
    fn topic(&self) -> String {
        self.app_settings.appointment_confirmed_topic_name.clone()
    }

    fn handle(&self) -> Result<(), anyhow::Error> {
        println!("appointment confirmed handled!");
        Ok(())
    }
}
