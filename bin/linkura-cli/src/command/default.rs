use crate::config::Global;
use chrono::{Local, Utc};
use linkura_api::ArchiveListOptions;
use linkura_api::model::FesliveLobbyRequest;
use linkura_i18n::t;

pub async fn run(ctx: &Global) {
    let _args = &ctx.args;

    let api_client = &ctx.api_client;
    let wm_res: serde_json::Value = api_client.high_level().get_plan_list().await.unwrap();

    let trailers = wm_res.as_array().unwrap();
    tracing::trace!("Trailers: {:?}", trailers);
    trailers.iter().for_each(|value| {
        print_trailer_info(value);
    });
    print_enterable_trailer_info(ctx, trailers).await;

    let archive_res: serde_json::Value = api_client
        .high_level()
        .get_archive_list(ArchiveListOptions {
            limit: Some(4),
            ..Default::default()
        })
        .await
        .unwrap();
    let latest_archive_res = archive_res.as_array().unwrap()[0].clone();
    print_latest_archive_info(ctx, &latest_archive_res).await;
}

fn print_trailer_info(wm: &serde_json::Value) {
    let live_type = wm.get("live_type").unwrap().as_u64().unwrap();
    let name: &str = wm.get("name").unwrap().as_str().unwrap();
    let description: &str = wm.get("description").unwrap().as_str().unwrap();
    let start_time: &str = wm.get("live_start_time").unwrap().as_str().unwrap();
    let open_time: &str = wm.get("open_time").unwrap().as_str().unwrap();
    tracing::info!(
        "{}",
        t!(
            "linkura.command.default.trailer.info",
            live_kind = if live_type == 2 {
                t!("linkura.command.default.trailer.kind.with_meets")
            } else {
                t!("linkura.command.default.trailer.kind.fes_live")
            },
            name = name,
            description = description,
            start_time = chrono::DateTime::parse_from_rfc3339(start_time)
                .unwrap()
                .with_timezone(&Local)
                .format("%Y-%m-%d %H:%M:%S %:z")
                .to_string(),
            open_time = chrono::DateTime::parse_from_rfc3339(open_time)
                .unwrap()
                .with_timezone(&Local)
                .format("%Y-%m-%d %H:%M:%S %:z")
                .to_string()
        )
    );
}

async fn print_latest_trailer_info(ctx: &Global, wm: &serde_json::Value) {
    let api_client = &ctx.api_client;
    let id = wm.get("live_id").unwrap().as_str().unwrap();
    let live_type = wm.get("live_type").unwrap().as_u64().unwrap();

    let name: &str = wm.get("name").unwrap().as_str().unwrap();
    let open_time: &str = wm.get("open_time").unwrap().as_str().unwrap();
    let now = Utc::now();
    if now < chrono::DateTime::parse_from_rfc3339(open_time).unwrap() {
        tracing::warn!(
            "{}",
            t!(
                "linkura.command.default.trailer.not_open",
                name = name,
                open_time = chrono::DateTime::parse_from_rfc3339(open_time)
                    .unwrap()
                    .with_timezone(&Local)
                    .format("%Y-%m-%d %H:%M:%S %:z")
                    .to_string()
            )
        );
        return;
    }

    if live_type == 2 {
        let res: Result<serde_json::Value, anyhow::Error> =
            api_client.high_level().get_with_meets_info(id).await;
        match res {
            Ok(res) => {
                let characters = res
                    .get("characters")
                    .unwrap()
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|v| v["character_id"].as_u64().unwrap())
                    .collect::<Vec<u64>>();
                tracing::info!(
                    "{}",
                    t!(
                        "linkura.command.default.with_meets.info",
                        title = name,
                        description = res.get("description").unwrap().as_str().unwrap(),
                        room = format!("{:?}", res.get("room").unwrap().as_object().unwrap()),
                        thumbnail = res.get("cover_image_url").unwrap().as_str().unwrap(),
                        hls_url = res
                            .get("hls")
                            .unwrap()
                            .as_object()
                            .unwrap()
                            .get("url")
                            .unwrap()
                            .as_str()
                            .unwrap(),
                        characters = format!("{:?}", characters),
                        costume_ids = format!(
                            "{:?}",
                            res.get("costume_ids").unwrap().as_array().unwrap()
                        ),
                        live_location_id = res.get("live_location_id").unwrap().as_u64().unwrap()
                    )
                );
            }
            Err(_) => {
                tracing::warn!(
                    "{}",
                    t!(
                        "linkura.command.default.with_meets.unavailable",
                        name = name,
                        id = id
                    )
                );
            }
        }
    }
    if live_type == 1 {
        // enter fes lobby first
        let lobby_request = FesliveLobbyRequest {
            live_id: Some(id.to_string()),
            ..Default::default()
        };
        let _ = api_client.raw().fes_live().lobby(&lobby_request).await;
        let res: Result<serde_json::Value, anyhow::Error> =
            api_client.high_level().get_fes_live_info(id).await;
        match res {
            Ok(res) => {
                let characters = res
                    .get("characters")
                    .unwrap()
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|v| v["character_id"].as_u64().unwrap())
                    .collect::<Vec<u64>>();
                tracing::info!(
                    "{}",
                    t!(
                        "linkura.command.default.fes_live.info",
                        title = name,
                        description = res.get("description").unwrap().as_str().unwrap(),
                        room = format!("{:?}", res.get("room").unwrap().as_object().unwrap()),
                        characters = format!("{:?}", characters),
                        hls = format!("{:?}", res.get("hls").unwrap().as_object().unwrap()),
                        costume_ids = format!(
                            "{:?}",
                            res.get("costume_ids").unwrap().as_array().unwrap()
                        ),
                        live_location_id = res.get("live_location_id").unwrap().as_u64().unwrap()
                    )
                );
            }
            Err(_) => {
                tracing::warn!(
                    "{}",
                    t!(
                        "linkura.command.default.fes_live.unavailable",
                        name = name,
                        id = id
                    )
                );
            }
        }
    }
}

async fn print_enterable_trailer_info(ctx: &Global, trailers: &Vec<serde_json::Value>) {
    let now = Utc::now();
    let mut enterable_trailers: Vec<&serde_json::Value> = Vec::new();
    for wm in trailers {
        let open_time: &str = wm.get("open_time").unwrap().as_str().unwrap();
        if now >= chrono::DateTime::parse_from_rfc3339(open_time).unwrap() {
            enterable_trailers.push(wm);
        }
    }
    if enterable_trailers.is_empty() {
        tracing::info!("{}", t!("linkura.command.default.enterable.none"));
        return;
    }
    tracing::info!(
        "{}",
        t!(
            "linkura.command.default.enterable.count",
            count = enterable_trailers.len()
        )
    );
    for wm in enterable_trailers {
        print_latest_trailer_info(ctx, wm).await;
    }
}

async fn print_latest_archive_info(ctx: &Global, archive: &serde_json::Value) {
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
            .await
            .unwrap_or_else(|_| String::new());
    }
    tracing::info!(
        "{}",
        t!(
            "linkura.command.default.latest_archive.info",
            title = title,
            description = description,
            thumbnail = thumbnail,
            link = link,
            url = real_url,
            video_url = video_url
        )
    );
}
