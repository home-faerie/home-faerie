use bytes::Bytes;
use serde_json::{map::Map, Result, Value};
use sqlx::postgres::PgPool;
use sqlx::types::Json;

fn parse(payload: &Bytes) -> Option<Vec<(String, f64)>> {
    let mut results: Vec<(String, f64)> = Vec::new();
    let res: Result<Value> = serde_json::from_slice(&payload);
    match res {
        Ok(val) => {
            if ! val.is_object() {
                // TODO: Log error?
                return None;
            }
            for (k, v) in val.as_object().unwrap() {
                match v.as_f64() {
                    Some(vv) => {
                        results.push((k.to_string(), vv));
                    },
                    None => {}
                }
            }
            if results.len() > 0 {
                return Some(results);
            }
            None
        },
        Err(_) => {
            // TODO: Log error?
            None
        }
    }
}

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
            match parse(&data) {
                Some(obj) => {
                    let mut dimensions = Map::new();
                    dimensions.insert("id".to_string(), Value::String(dev.to_string()));
                    dimensions.insert("from".to_string(), Value::String("zigbee2mqtt".to_string()));

                    for (k, v) in obj {
                        // TODO: value_meta -> None ??
                        let res = sqlx::query("INSERT INTO device_meters
                                    (timestamp, name, value, dimensions)
                                    VALUES(NOW(), $1, $2, $3)")
                                .bind(k)
                                .bind(v)
                                .bind(Json(&dimensions))
                        .execute(pool).await;
                        // TODO: Logging...
                        println!("INSERT: {:?}?", res);
                    }
                },
                None => {
                    println!("Error parsing packet from {:?}: payload: {:?}", topic.join("/"), data);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn meter_json_parse() {
        assert_eq!(None, parse(&Bytes::from_static(b"broken json")));

        assert_eq!(None, parse(&Bytes::from_static(b"{}")));

        assert_eq!(None, parse(&Bytes::from_static(b"{\"foo\":[1,2]}")));

        assert_eq!(None, parse(&Bytes::from_static(b"{\"foo\":{\"x\":0.2}}")));

        assert_eq!(parse(&Bytes::from_static(b"{\"foo\":0.44,\"bar\":22,\"action\":\"test\"}")),
            Some(vec![("foo".to_string(), 0.44 as f64), ("bar".to_string(), 22 as f64)]));
    }
}
