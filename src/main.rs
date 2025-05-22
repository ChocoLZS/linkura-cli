use chrono::{DateTime, Utc};
use config::init;
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

mod api;
mod config;

fn main() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .init();

    // do prepare
    let global = init().expect("Failed to initialize config");
    let args = &global.args;

    let api_client = &global.api_client;
    let wm_res: serde_json::Value = api_client.get_with_meets_plan_list().unwrap();
    println!("wm_res: {:?}", wm_res);
    let archive_res: serde_json::Value = api_client.get_archive_list().unwrap();
    println!("archive_res: {:?}", archive_res);

    if let Some(id) = &args.id {
        let res = api_client.get_with_meets_info(&id).unwrap();
        println!("wm info: {:?}", res);
    } else {
        let mut res: Option<&serde_json::Value> = None;
        let now = Utc::now();
        for item in wm_res.as_array().unwrap().into_iter() {
            let end_time = item.get("end_time").unwrap().as_str().unwrap();
            let end_time = DateTime::parse_from_rfc3339(end_time).unwrap();
            if now <= end_time {
                res = Some(item)
            } else {
                break;
            }
        }
        let mut id = res
            .unwrap()
            .get("link")
            .unwrap()
            .as_str()
            .unwrap();
        // id may start with "withLive" or "fesLive"
        
        let name: &str = res.unwrap().get("name").unwrap().as_str().unwrap();
        if id.starts_with("withLive") {
            id = id.strip_prefix("withLive").unwrap();
            let res: Result<serde_json::Value, anyhow::Error> = api_client.get_with_meets_info(id);
            match res {
                Ok(res) => {
                    println!(
                        "latest wm info: \n title: {}\n description: {:?}\n thumbnail: {:?}\n characters: {:?}",
                        name,
                        res.get("description").unwrap().as_str().unwrap(),
                        res.get("thumbnail").unwrap().as_str().unwrap(),
                        res.get("characters").unwrap().as_array().unwrap()
                    );
                }
                Err(_) => {
                    tracing::warn!("Can't get latest wm info for now! {:?} {}", name, id);
                }
            }
        } else if id.starts_with("fesLive") {
            id = id.strip_prefix("fesLive").unwrap();
            let res: Result<serde_json::Value, anyhow::Error> = api_client.get_fes_live_info(id);
            match res {
                Ok(res) => {
                    println!("fes live info: \n title: {}\n description: {:?}\n room: {:?}\n characters: {:?}",
                        name,
                        res.get("description").unwrap().as_str().unwrap(),
                        res.get("room").unwrap().as_object().unwrap(),
                        res.get("characters").unwrap().as_array().unwrap(),
                    );
                }
                Err(_) => {
                    tracing::warn!("Can't get latest fes live info for now! {:?} {}", name, id);
                }
            }
        }
    }

    let latest_archive_res = archive_res.as_array().unwrap()[0].clone();
    let title = latest_archive_res.get("name").unwrap().as_str().unwrap();
    let description = latest_archive_res
        .get("description")
        .unwrap()
        .as_str()
        .unwrap();
    let thumbnail = latest_archive_res
        .get("thumbnail_image_url")
        .unwrap()
        .as_str()
        .unwrap();
    let link = latest_archive_res
        .get("external_link")
        .unwrap()
        .as_str()
        .unwrap();

    println!(
        "latest archive: \n title: {:?}\n description: {:?}\n thumbnail: {:?}\n link: {:?}",
        title, description, thumbnail, link
    );

    return;
}
