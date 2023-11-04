use std::collections::HashMap;

use anyhow::Result;
use serde::{Deserialize, Serialize};

const ENVIRONMENT_KEY: &str = "ENVIRONMENT";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppSettings {
    pub kafka_bootstrap_servers: String,
    pub kafka_group_id: String,
    pub appointment_solicited_topic_name: String,
    pub appointment_confirmed_topic_name: String,
    pub database_url: String,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            kafka_bootstrap_servers: String::default(),
            kafka_group_id: String::default(),
            appointment_solicited_topic_name: String::default(),
            appointment_confirmed_topic_name: String::default(),
            database_url: String::default(),
        }
    }
}

impl AppSettings {
    pub fn new() -> Result<AppSettings> {
        let environment_file = Self::get_environment_file();

        confy::load_path(environment_file).map_err(anyhow::Error::new)
    }

    fn get_environment_file() -> String {
        match std::env::vars()
            .collect::<HashMap<String, String>>()
            .get(ENVIRONMENT_KEY)
        {
            None => "./env/local.toml".to_string(),
            Some(_) => "./env/local.toml".to_string(),
        }
    }
}
