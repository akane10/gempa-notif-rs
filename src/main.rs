use bmkgw::gempa::{self, Gempa, Url};
use dotenv;
use tokio::time;

pub mod push;

fn set_message(data: &Gempa) -> String {
    format!(
        "Terjadi Gempa\nJam {}\n{}\nLokasi {}\n{}",
        data.jam.as_deref().unwrap_or(&String::new()),
        data.magnitude.as_deref().unwrap_or(&String::new()),
        data.wilayah.as_deref().unwrap_or(&String::new()),
        data.potensi.as_deref().unwrap_or(&String::new()),
    )
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    dotenv::dotenv().ok();

    let mut last_time = String::new();

    let mut interval = time::interval(time::Duration::from_secs(60));
    loop {
        interval.tick().await;
        let data: Vec<Gempa> = gempa::get_data(Url::GempaTerkini).await?;
        let time: Option<String> = match data.len() {
            n if n > 0 => data[0].jam.clone(),
            _ => None,
        };

        if let Some(t) = time {
            if t != last_time {
                let msg = set_message(&data[0]);
                push::notif(&msg).await?;
                last_time = t;
            }
        }
    }
}
