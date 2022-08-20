use log::warn;
use paho_mqtt::Message;
use serde::de::Deserialize;
use std::collections::HashMap;

pub trait Exporter<T> {
    /// The name of the mqtt channel to grab messages from
    fn channel(&self) -> &str;

    /// Do the exporting of the deserializable message into prometheus
    fn process(&self, buf: T)
    where
        T: for<'a> Deserialize<'a>;
}

pub struct ExporterRegistry<T>(HashMap<String, Box<dyn Exporter<T>>>);

impl<T> ExporterRegistry<T>
where
    T: for<'a> Deserialize<'a>,
{
    pub fn new() -> Self {
        ExporterRegistry(HashMap::new())
    }

    pub fn register(&mut self, parser: Box<dyn Exporter<T>>) {
        self.0.insert(String::from(parser.channel()), parser);
    }

    pub fn dispatch(&self, msg: Message) {
        let topic = msg.topic();
        match self.0.get(topic) {
            Some(parser) => {
                // TODO do some schema version logic checking here
                parser.process(postcard::from_bytes(msg.payload()).unwrap());
            }
            None => warn!("Unknown topic being recieved by exporter: {}", topic),
        }
    }
}
