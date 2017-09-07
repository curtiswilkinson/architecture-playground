extern crate kafka;
extern crate env_logger;

use kafka::consumer::{Consumer, FetchOffset, GroupOffsetStorage};
use kafka::error::Error as KafkaError;

fn main() {
    env_logger::init().unwrap();
    let broker = "localhost:9092".to_owned();
    let topic = "test".to_owned();
    let group = "group".to_owned();

    if let Err(e) = consume(group, topic, vec![broker]) {
        println!("Failed consuming messages: {}", e);
    }
}

fn consume(group: String, topic: String, brokers: Vec<String>) -> Result<(), KafkaError> {
    let mut connection = try!(Consumer::from_hosts(brokers)
                              .with_topic(topic)
                              .with_group(group)
                              .with_fallback_offset(FetchOffset::Earliest)
                              .with_offset_storage(GroupOffsetStorage::Kafka)
                              .create());
    loop {
        let mss = try!(connection.poll());

        if mss.is_empty() {
            println!("No Messages!");
        }

        for ms in mss.iter() {
            for m in ms.messages() {
                println!("{}:{}@{}: {:?}", ms.topic(), ms.partition(), m.offset, m.value);
            }
            let _ = connection.consume_messageset(ms);
        }
        try!(connection.commit_consumed());
    }
}
