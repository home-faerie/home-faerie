use main_error::MainError;
use rand::{distributions::Alphanumeric, Rng};
use rumqttc::{AsyncClient, Event, MqttOptions, Packet, QoS};
use sqlx::postgres::PgPool;

pub mod zigbee2mqtt;

#[tokio::main]
async fn main() -> Result<(), MainError> {
    let pgsql_url = std::env::var("POSTGRESQL_URL")?;
    let mqtt_url = std::env::var("MQTT_URL")?;
    let mqtt_port: u16 = std::env::var("MQTT_PORT")?.parse().unwrap_or(1883);
    let s: String = rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(7)
                .map(char::from)
                .collect();

    let mut mqttoptions = MqttOptions::new(["home", "faerie", &s].join("-"), mqtt_url, mqtt_port);
    mqttoptions.set_keep_alive(5);

    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);

    let pool = PgPool::connect(&pgsql_url).await?;

    loop {
        match eventloop.poll().await {
            Ok(Event::Incoming(Packet::Publish(p))) => {
                // TODO: Set up handlers dynamically...
                let topic = p.topic.as_str().split('/').collect::<Vec<&str>>();
                if topic.len() == 0 {
                    continue;
                }
                match topic[0] {
                    "zigbee2mqtt" => {
                        zigbee2mqtt::handle_packet(&pool, &topic[1..], p.payload).await;
                    },
                    _ => {
                        //println!("Some(_): {:?}:{:?}", p.topic, p.payload);
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
