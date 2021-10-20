use dotenv;

pub mod push;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    dotenv::dotenv().ok();

    let msg = "Hello There";
    push::notif(msg).await?;
    Ok(())
}
