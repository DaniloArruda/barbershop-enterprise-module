use std::pin::Pin;

use futures::Future;
use rdkafka::{
    consumer::{CommitMode, Consumer, StreamConsumer},
    ClientConfig, Message,
};

use super::handler::Handler;

pub struct KafkaClient {
    kafka_configuration: ClientConfig,
    handlers: Vec<Box<dyn Handler>>,
}

impl KafkaClient {
    pub fn new(kafka_configuration: ClientConfig) -> KafkaClient {
        KafkaClient {
            kafka_configuration,
            handlers: Vec::new(),
        }
    }

    pub fn attach(&mut self, handler: Box<dyn Handler>) -> &mut Self {
        self.handlers.push(handler);

        self
    }

    pub async fn consume(&self) {
        let mut futures: Vec<Pin<Box<dyn Future<Output = ()>>>> = Vec::new();
        println!("will consume");

        self.handlers.iter().for_each(|handler| {
            futures.push(Box::pin(self.subscribe_handler(handler)));
        });

        futures::future::select_all(futures).await;
    }

    #[allow(unused_must_use)]
    async fn subscribe_handler(&self, handler: &Box<dyn Handler>) {
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
                    println!("key: '{:?}', payload: '{}', topic: {}, partition: {}, offset: {}, timestamp: {:?}",
                        borrowed_message.key(), payload, borrowed_message.topic(), borrowed_message.partition(), borrowed_message.offset(), borrowed_message.timestamp());

                    consumer
                        .commit_message(&borrowed_message, CommitMode::Async)
                        .unwrap();
                });
        }
    }
}
