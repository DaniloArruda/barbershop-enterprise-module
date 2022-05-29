use enterprise_module_lib::adapter::{
    config::{app_settings::AppSettings, kafka_client::KafkaClient},
    consumer::{
        appointment_confirmed::AppointmentConfirmedHandler,
        appointment_solicited::AppointmentSolicitedHandler,
    },
};
use rdkafka::{config::RDKafkaLogLevel, ClientConfig};

#[tokio::main]
async fn main() {
    let app_settings = AppSettings::new().unwrap();

    let kafka_configuration = kafka_configuration(&app_settings);

    consume_kafka_topics(kafka_configuration, &app_settings).await;
}

fn kafka_configuration(app_settings: &AppSettings) -> ClientConfig {
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

    configuration
}

async fn consume_kafka_topics(kafka_configuration: ClientConfig, app_settings: &AppSettings) {
    println!("will create kafka client");
    KafkaClient::new(kafka_configuration)
        .attach(Box::new(AppointmentSolicitedHandler {
            app_settings: app_settings.clone(),
        }))
        .attach(Box::new(AppointmentConfirmedHandler {
            app_settings: app_settings.clone(),
        }))
        .consume()
        .await
}
