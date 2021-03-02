use bytes::Bytes;
use serde_json::{map::Map, Value};
use sqlx::postgres::PgPool;
use sqlx::types::Json;

pub async fn handle_packet(pool: &PgPool, topic: &[&str], data: Bytes) {
    let mut t = topic.iter();
    match *t.next().unwrap() {
        "bridge" => {
            println!("Bridge! {:?} {:?}", topic, data);
        },
        dev => {
            // Check whether there are still elements in topic array
            if t.next() != None  {
                println!("Unhandled Zigbee2mqtt topic: {}", topic.join("/"));
                return;
            }
            // TODO: Check whether topic or device id is present in device list
            let obj: Value = serde_json::from_slice(&data).unwrap();
            /*
            - "battery"
            - "humidity"
            - "linkquality"
            - "temperature"
            - "voltage"
            */

            // Build dimensions object
            let mut dimensions = Map::new();
            dimensions.insert("id".to_string(), Value::String(dev.to_string()));
            dimensions.insert("from".to_string(), Value::String("zigbee2mqtt".to_string()));

            for (k, v) in obj.as_object().unwrap() {
                // TODO: value_meta -> None ??
                let res = sqlx::query("INSERT INTO device_meters
                            (timestamp, name, value, dimensions)
                            VALUES(NOW(), $1, $2, $3)")
                        .bind(k)
                        .bind(v.as_f64())
                        .bind(Json(&dimensions))
                        .execute(pool).await;
                // TODO: Logging...
                println!("INSERT: {:?}?", res);
            }
        }
    }
}
