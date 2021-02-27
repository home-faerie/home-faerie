use main_error::MainError;
use rumqttc::{AsyncClient, Event, MqttOptions, Packet, QoS};

#[tokio::main]
async fn main() -> Result<(), MainError> {
    let mqtt_url = std::env::var("MQTT_URL")?;
    let mqtt_port: u16 = std::env::var("MQTT_PORT")?.parse().unwrap_or(1883);
    let mut mqttoptions = MqttOptions::new("rumqtt-async", mqtt_url, mqtt_port);
    mqttoptions.set_keep_alive(5);

    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);

    loop {
        match eventloop.poll().await {
            Ok(Event::Incoming(Packet::Publish(p))) => {
                // TODO: Set up handlers dynamically...
                match p.topic.as_str().split('/').take(1).next() {
                    Some("zigbee2mqtt") => {
                        println!("TODO: {:?}:{:?}", p.topic, p.payload);
                    },
                    Some(_) => {
                        println!("Some(_): {:?}:{:?}", p.topic, p.payload);
                    },
                    None => {
                        println!("None: {:?}:{:?}", p.topic, p.payload);
                    }
                }
            },
            Ok(Event::Incoming(Packet::ConnAck(_p))) => {
                // Subscribe to topics after ConnAck
                client.subscribe("#", QoS::AtMostOnce).await.unwrap();
            },
            Ok(Event::Incoming(_p)) => {
                // println!("In: {:?}", _p);
            },
            Ok(Event::Outgoing(_p)) => {
                // println!("Out: {:?}", _p);
            },
            Err(e) => {
                println!("Error = {:?}", e);
            }
        }
    }
}
