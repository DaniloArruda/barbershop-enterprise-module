use anyhow::Result;

use enterprise_module_lib::adapter::{
    config::{app_settings::AppSettings, kafka_client::KafkaClient},
    consumer::{
        appointment_confirmed::AppointmentConfirmedHandler,
        appointment_solicited::AppointmentSolicitedHandler,
    },
};
use rdkafka::{config::RDKafkaLogLevel, ClientConfig};
use tokio::task::JoinError;

#[tokio::main]
async fn main() -> Result<()> {
    let app_settings = AppSettings::new().unwrap();

    let kafka_consumer = KafkaConsumer::new(&app_settings);

    let kafka_consumer_task = tokio::spawn(kafka_consumer.start());

    tokio::select! {
        o = kafka_consumer_task => report_exit("kafka consumer", o),
    }

    Ok(())
}

pub fn report_exit(task_name: &str, outcome: Result<(), JoinError>) {
    match outcome {
        Ok(()) => println!("{} has exited", task_name),
        Err(error) => println!("{} failed; error: {}", task_name, error),
    }
}

pub struct KafkaConsumer {
    pub client: KafkaClient,
}

impl KafkaConsumer {
    pub fn new(app_settings: &AppSettings) -> KafkaConsumer {
        let mut configuration = ClientConfig::new();

        configuration
            .set("group.id", app_settings.kafka_group_id.clone())
            .set(
                "bootstrap.servers",
                app_settings.kafka_bootstrap_servers.clone(),
            )
            .set("enable.partition.eof", "false")
            .set("session.timeout.ms", "6000")
            .set("enable.auto.commit", "true")
            .set_log_level(RDKafkaLogLevel::Debug);

        KafkaConsumer {
            client: KafkaClient::new(configuration)
                .attach(Box::new(AppointmentSolicitedHandler {
                    app_settings: app_settings.clone(),
                }))
                .attach(Box::new(AppointmentConfirmedHandler {
                    app_settings: app_settings.clone(),
                })),
        }
    }

    pub async fn start(self) {
        self.client.consume().await
    }
}
