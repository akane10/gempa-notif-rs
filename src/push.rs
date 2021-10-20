use pretty_env_logger;
use std::fs::File;
use web_push::*;

// TODO:
//  - get data from redis
pub async fn notif(msg: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    pretty_env_logger::init();

    let endpoint = &dotenv::var("ENDPOINT_EXAMPLE").expect("Missing example endpoint");
    let p256dh = &dotenv::var("P256DH_EXAMPLE").expect("Missing example p256dh");
    let auth = &dotenv::var("AUTH_EXAMPLE").expect("Missing example auth");
    let mailto = &dotenv::var("MAILTO").expect("Missing mailto");

    let subscription_info = SubscriptionInfo::new(endpoint, &p256dh, &auth);

    let file = File::open("vapid_private.pem")?;
    let mut sig_builder = VapidSignatureBuilder::from_pem(file, &subscription_info)?;
    sig_builder.add_claim("sub", mailto.as_str());
    let signature = sig_builder.build()?;

    let mut builder = WebPushMessageBuilder::new(&subscription_info)?;
    builder.set_vapid_signature(signature);
    builder.set_payload(ContentEncoding::Aes128Gcm, msg.as_bytes());

    let message = builder.build()?;
    let client = WebPushClient::new()?;
    let response = client.send(message).await?;
    println!("Sent: {:?}", response);
    Ok(())
}
