use std::pin::Pin;

use futures::Future;
use rdkafka::{
    consumer::{CommitMode, Consumer, StreamConsumer},
    ClientConfig, Message,
};

use super::handler::{Handler, HandlerWrapper};

pub struct KafkaClient {
    kafka_configuration: ClientConfig,
    handlers: Vec<Box<dyn Handler<Message = String> + Send + Sync>>,
}

impl KafkaClient {
    pub fn new(kafka_configuration: ClientConfig) -> KafkaClient {
        KafkaClient {
            kafka_configuration,
            handlers: Vec::new(),
        }
    }

    pub fn attach<H: Handler + Send + Sync + 'static>(mut self, handler: Box<H>) -> KafkaClient {
        self.handlers
            .push(Box::new(HandlerWrapper { inner: handler }));

        Self {
            handlers: self.handlers,
            ..self
        }
    }

    pub async fn consume(&self) {
        let mut futures: Vec<Pin<Box<dyn Future<Output = ()> + Send + Sync>>> = Vec::new();
        println!("will consume");

        self.handlers.iter().for_each(|handler| {
            futures.push(Box::pin(self.subscribe_handler(handler)));
        });

        futures::future::select_all(futures).await;
    }

    #[allow(unused_must_use)]
    async fn subscribe_handler(&self, handler: &Box<dyn Handler<Message = String> + Send + Sync>) {
        let consumer: StreamConsumer = self.kafka_configuration.create().unwrap();

        let topic = handler.topic();

        println!("will subscribe");

        consumer
            .subscribe(&vec![topic.as_str()])
            .expect("Can't subscribe to specified topics");

        loop {
            consumer.recv()
                .await
                .map_err(|error| {
                    println!("Kafka error: {}", error);
                    error
                })
                .map(|borrowed_message| {
                    let payload = match borrowed_message.payload_view::<str>() {
                        None => "",
                        Some(Ok(s)) => s,
                        Some(Err(e)) => {
                            println!("Error while deserializing message payload: {:?}", e);
                            ""
                        }
                    };
                    println!("key: '{:?}', topic: {}, partition: {}, offset: {}, timestamp: {:?}, payload: '{}'",
                        borrowed_message.key(), borrowed_message.topic(), borrowed_message.partition(), borrowed_message.offset(), borrowed_message.timestamp(), payload);

                    handler.handle(payload.to_string());

                    consumer
                        .commit_message(&borrowed_message, CommitMode::Async)
                        .unwrap();
                });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct FakeHandler {}

    impl Handler for FakeHandler {
        type Message = String;

        fn topic(&self) -> String {
            "fake".to_string()
        }

        fn handle(&self, _message: Self::Message) -> Result<(), anyhow::Error> {
            Ok(())
        }
    }

    #[test]
    fn should_init_without_handlers() {
        // given
        let kafka_configuration = ClientConfig::new();
        let kafka_client = KafkaClient::new(kafka_configuration);

        // then
        assert_eq!(0, kafka_client.handlers.len());
    }

    #[test]
    fn should_add_handlers_vector_when_attach() {
        // given
        let kafka_configuration = ClientConfig::new();
        let mut kafka_client = KafkaClient::new(kafka_configuration);
        let handler = FakeHandler {};

        // when
        kafka_client = kafka_client.attach(Box::new(handler));

        // then
        assert_eq!(1, kafka_client.handlers.len());
    }
}
