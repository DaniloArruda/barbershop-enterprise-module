use enterprise_module_lib::{
    adapter::{
        config::{app_settings::AppSettings, kafka_client::KafkaClient},
        consumer::{
            appointment_confirmed::AppointmentConfirmedHandler,
            appointment_solicited::AppointmentSolicitedHandler,
        },
        producer::{
            appointment_created_producer::AppointmentCreatedProducerKafka,
            appointment_rejected_producer::AppointmentRejectedProducerKafka,
        },
        repository::{
            appointment_repository::AppointmentRepositoryPostgres,
            barber_repository::BarberRepositoryPostgres,
            client_repository::ClientRepositoryPostgres, task_repository::TaskRepositoryPostgres,
        },
    },
    application::usecase::appointment_solicited::AppointmentSolicitedUseCase,
};
use rdkafka::{config::RDKafkaLogLevel, ClientConfig};

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

        let appointment_solicited_use_case = AppointmentSolicitedUseCase {
            barber_repository: Box::new(BarberRepositoryPostgres {}),
            client_repository: Box::new(ClientRepositoryPostgres {}),
            task_repository: Box::new(TaskRepositoryPostgres {}),
            appointment_repository: Box::new(AppointmentRepositoryPostgres {}),
            appointment_rejected_producer: Box::new(AppointmentRejectedProducerKafka {}),
            appointment_created_producer: Box::new(AppointmentCreatedProducerKafka {}),
        };

        KafkaConsumer {
            client: KafkaClient::new(configuration)
                .attach(Box::new(AppointmentSolicitedHandler {
                    app_settings: app_settings.clone(),
                    appointment_solicited_use_case,
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
