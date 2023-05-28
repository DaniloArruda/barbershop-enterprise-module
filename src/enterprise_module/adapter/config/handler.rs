use crate::adapter::error::AdapterError;

pub trait Handler {
    type Message: serde::de::DeserializeOwned + std::fmt::Debug;

    fn topic(&self) -> String;
    fn handle(&self, message: Self::Message) -> Result<(), anyhow::Error>;
}

pub struct HandlerWrapper<T> {
    pub(crate) inner: Box<dyn Handler<Message = T> + Send + Sync>,
}

impl<T: std::fmt::Debug + for<'de> serde::Deserialize<'de>> Handler for HandlerWrapper<T> {
    type Message = String;

    fn topic(&self) -> String {
        self.inner.topic()
    }

    fn handle(&self, message: Self::Message) -> Result<(), anyhow::Error> {
        let inner_message = serde_json::from_str(message.as_str())
            .map_err(|error| AdapterError::from_serde_error(error))?;

        self.inner.handle(inner_message)
    }
}
