use bmkgw::gempa::{self, Gempa, Url};
use dotenv;

pub mod push;

fn set_message(data: &Gempa) -> String {
    format!(
        "Terjadi Gempa\nJam {}\n{}\n{}\n{}",
        data.jam.as_deref().unwrap_or(&String::new()),
        data.magnitude.as_deref().unwrap_or(&String::new()),
        data.potensi.as_deref().unwrap_or(&String::new()),
        data.wilayah.as_deref().unwrap_or(&String::new())
    )
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    dotenv::dotenv().ok();

    let last_time = String::new();

    let data: Vec<Gempa> = gempa::get_data(Url::GempaTerkini).await?;
    let time: Option<String> = match data.len() {
        n if n > 0 => data[0].jam.clone(),
        _ => None,
    };

    if let Some(t) = time {
        if t != last_time {
            // send push
            let msg = set_message(&data[0]);
            push::notif(&msg).await?;
        }
    }

    Ok(())
}
