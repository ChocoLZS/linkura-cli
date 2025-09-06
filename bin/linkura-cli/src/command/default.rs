use crate::config::Global;
use chrono::{Local, Utc};
use linkura_common::jwt::extract_jwt_payload;

pub fn run(ctx: &Global) {
    let args = &ctx.args;

    let api_client = &ctx.api_client;
    let wm_res: serde_json::Value = api_client.high_level().get_plan_list().unwrap();

    if let Some(id) = &args.id {
        let res = api_client.high_level().get_with_meets_info(&id).unwrap();
        tracing::info!("wm info: {:?}", res);
    } else {
        let trailers = wm_res.as_array().unwrap();
        tracing::info!("Trailers: {:?}", trailers);
        let first_trailer = trailers.first();
        if let Some(trailer) = first_trailer {
            print_latest_trailer_archive(ctx, trailer);
        }
    }

    let archive_res: serde_json::Value = api_client.high_level().get_archive_list(Some(4)).unwrap();
    let latest_archive_res = archive_res.as_array().unwrap()[0].clone();
    print_latest_archive_info(ctx, &latest_archive_res);
}

fn print_latest_trailer_archive(ctx: &Global, wm: &serde_json::Value) {
    let api_client = &ctx.api_client;
    let id = wm.get("live_id").unwrap().as_str().unwrap();
    let live_type = wm.get("live_type").unwrap().as_u64().unwrap();

    let name: &str = wm.get("name").unwrap().as_str().unwrap();
    let description: &str = wm.get("description").unwrap().as_str().unwrap();
    let start_time: &str = wm.get("live_start_time").unwrap().as_str().unwrap();
    let open_time: &str = wm.get("open_time").unwrap().as_str().unwrap();
    tracing::info!(
        "latest {} info: \n{}\n\n{}\nstart_time: {}\nopen_time: {}",
        if live_type == 2 {
            "with meets"
        } else {
            "fes live"
        },
        name,
        description,
        chrono::DateTime::parse_from_rfc3339(start_time)
            .unwrap()
            .with_timezone(&Local)
            .format("%Y-%m-%d %H:%M:%S %:z"),
        chrono::DateTime::parse_from_rfc3339(open_time)
            .unwrap()
            .with_timezone(&Local)
            .format("%Y-%m-%d %H:%M:%S %:z")
    );
    let now = Utc::now();
    if now < chrono::DateTime::parse_from_rfc3339(open_time).unwrap() {
        tracing::warn!(
            "The live has not openned yet! {} {}",
            name,
            chrono::DateTime::parse_from_rfc3339(open_time)
                .unwrap()
                .with_timezone(&Local)
                .format("%Y-%m-%d %H:%M:%S %:z")
        );
        return;
    }
    let maybe_started = Utc::now()
        >= chrono::DateTime::parse_from_rfc3339(start_time).unwrap()
            - chrono::Duration::minutes(10);

    if live_type == 2 {
        let res: Result<serde_json::Value, anyhow::Error> =
            api_client.high_level().get_with_meets_info(id);
        match res {
            Ok(res) => {
                tracing::info!(
                    "with meets info: \n title: {}\n description: {:?}\n room: {:?}\n thumbnail: {:?}\n hls_url: {:?}\n characters: {:?}",
                    name,
                    res.get("description").unwrap().as_str().unwrap(),
                    res.get("room").unwrap().as_object().unwrap(),
                    res.get("thumbnail").unwrap().as_str().unwrap(),
                    res.get("hls_url").unwrap().as_str().unwrap(),
                    res.get("characters").unwrap().as_array().unwrap()
                );
            }
            Err(_) => {
                tracing::warn!(
                    "Can't get latest with meets info for now! {:?} {}",
                    name,
                    id
                );
            }
        }
        if maybe_started {
            let token = api_client.high_level().get_with_meets_connect_token(id);
            match token {
                Ok(token) => {
                    let _ = extract_jwt_payload(token.as_str()).and_then(|payload| {
                        tracing::info!(
                            "Room info: \n address: {}\n port: {}\n room_id: {}",
                            payload["pod"]["address"].as_str().unwrap().to_string(),
                            payload["pod"]["port"].as_u64().unwrap(),
                            payload["room_id"].as_str().unwrap().to_string()
                        );
                        Ok(payload)
                    });
                }
                Err(e) => {
                    tracing::warn!("Failed to get with meets connect token: {}", e);
                }
            }
        }
    }
    if live_type == 1 {
        let res: Result<serde_json::Value, anyhow::Error> =
            api_client.high_level().get_fes_live_info(id);
        match res {
            Ok(res) => {
                tracing::info!(
                    "fes live info: \n title: {}\n description: {:?}\n room: {:?}\n characters: {:?}",
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

fn print_latest_archive_info(ctx: &Global, archive: &serde_json::Value) {
    let title = archive.get("name").unwrap().as_str().unwrap();
    let description = archive.get("description").unwrap().as_str().unwrap();
    let thumbnail = archive
        .get("thumbnail_image_url")
        .unwrap()
        .as_str()
        .unwrap();
    let link = archive.get("external_link").unwrap().as_str().unwrap();
    let video_url = archive.get("video_url").unwrap().as_str().unwrap();
    let mut real_url = String::new();
    if !link.is_empty() {
        real_url = ctx
            .api_client
            .assets()
            .get_hls_url_from_archive(link)
            .unwrap_or_else(|_| String::new());
    }
    tracing::info!(
        "Latest archive: \n title: {:?}\n description: {:?}\n thumbnail: {:?}\n link: {:?}\n url: {:?}\n video_url: {:?}",
        title,
        description,
        thumbnail,
        link,
        real_url,
        video_url
    );
}
