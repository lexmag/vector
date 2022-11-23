use crate::event::Event;
use crate::sinks::pulsar::config::PulsarSinkConfig;
use std::collections::BTreeMap;
use value::Value;
use vector_config::component::GenerateConfig;

use bytes::Bytes;

use crate::event::LogEvent;

#[test]
fn generate_config() {
    PulsarSinkConfig::generate_config();
}

#[test]
fn pulsar_get_headers() {
    let properties_key = "properties";
    let mut property_values = BTreeMap::new();
    property_values.insert("a-key".to_string(), Value::Bytes(Bytes::from("a-value")));
    property_values.insert("b-key".to_string(), Value::Bytes(Bytes::from("b-value")));

    let mut event = Event::Log(LogEvent::from("hello"));
    event.as_mut_log().insert(properties_key, property_values);

    let properties =
        super::util::get_properties(&event, &Some(properties_key.to_string())).unwrap();
    assert_eq!(properties.get("a-key").unwrap(), "a-value".as_bytes());
    assert_eq!(properties.get("b-key").unwrap(), "b-value".as_bytes());
}
