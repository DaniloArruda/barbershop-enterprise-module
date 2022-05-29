pub trait Handler {
    fn topic(&self) -> String;
    fn handle(&self) -> Result<(), anyhow::Error>;
}
