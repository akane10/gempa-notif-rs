use redis::Commands;
use redis::{self, RedisError};
use serde::{Deserialize, Serialize};
use std::fs::File;
use web_push::*;

fn conn_redis() -> redis::RedisResult<redis::Connection> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let con = client.get_connection()?;
    Ok(con)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Data {
    auth: String,
    p256dh: String,
    endpoint: String,
}

pub async fn notif(msg: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let mut con = conn_redis()?;
    let keys: Vec<String> = con.keys("*")?;

    for k in keys.into_iter() {
        let val: String = con.get(k)?;
        let data: Result<Data, _> = serde_json::from_str(&val);
        match data {
            Ok(d) => {
                let mailto = &dotenv::var("MAILTO").expect("Missing mailto");

                let subscription_info = SubscriptionInfo::new(d.endpoint, d.p256dh, d.auth.clone());

                let file = File::open("private_key.pem")?;
                let mut sig_builder = VapidSignatureBuilder::from_pem(file, &subscription_info)?;
                sig_builder.add_claim("sub", mailto.as_str());
                let signature = sig_builder.build()?;

                let mut builder = WebPushMessageBuilder::new(&subscription_info)?;
                builder.set_vapid_signature(signature);
                builder.set_payload(ContentEncoding::Aes128Gcm, msg.as_bytes());

                let message = builder.build()?;
                let client = WebPushClient::new()?;
                let response = client.send(message).await;

                match response {
                    Err(e) => match e {
                        err if err == WebPushError::EndpointNotValid
                            || err == WebPushError::Unauthorized =>
                        {
                            let res: Result<(), RedisError> = con.del(d.auth);
                            match res {
                                _ => (),
                            }
                        }
                        _ => {
                            println!("failed {:#?}", e);
                        }
                    },
                    _ => {
                        println!("sent {}", d.auth);
                    }
                }
            }
            _ => (),
        }
    }
    Ok(())
}
