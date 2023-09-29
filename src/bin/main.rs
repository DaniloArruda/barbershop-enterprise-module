use anyhow::Result;

use enterprise_module_lib::adapter::config::app_settings::AppSettings;
use tokio::task::JoinError;

use crate::runnable::kafka_consumer::KafkaConsumer;

pub mod runnable;

#[tokio::main]
async fn main() -> Result<()> {
    let app_settings = AppSettings::new().unwrap();

    let kafka_consumer = KafkaConsumer::new(&app_settings);

    let kafka_consumer_task = tokio::spawn(kafka_consumer.start());
    println!("Kafka consumer started");

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
