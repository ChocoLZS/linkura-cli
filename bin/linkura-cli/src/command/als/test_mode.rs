use std::fs::File;

use anyhow::Result;
use linkura_common::jwt::extract_jwt_payload;

use crate::config::Global;

use super::{ArgsALS, api_runtime::fetch_token, model::parse_plan_items};

pub async fn run(ctx: &mut Global, args: &ArgsALS) -> Result<()> {
    let mut test_results: Vec<serde_json::Value> = Vec::new();
    let api_client = &ctx.api_client;
    let plan_list = api_client.high_level().get_plan_list().await?;
    let plans = parse_plan_items(plan_list)?;

    if let (Some(live_id), Some(live_type)) = (args.live_id.clone(), args.live_type) {
        let hit = plans
            .iter()
            .find(|p| p.live_id == live_id && p.live_type == live_type);
        let token = fetch_token(api_client, &live_id, live_type).await?;
        let payload_json = extract_jwt_payload(token.as_str())?;
        test_results.push(serde_json::json!({
            "name": hit.map(|h| h.name.clone()).unwrap_or_default(),
            "live_id": live_id,
            "live_type": live_type,
            "address": payload_json["pod"]["address"].as_str().unwrap_or_default(),
            "port": payload_json["pod"]["port"].as_u64().unwrap_or(0),
            "room_id": payload_json["room_id"].as_str().unwrap_or_default(),
        }));
    } else {
        for p in plans {
            match fetch_token(api_client, &p.live_id, p.live_type).await {
                Ok(token) => {
                    let payload_json = extract_jwt_payload(token.as_str())?;
                    test_results.push(serde_json::json!({
                        "name": p.name,
                        "live_id": p.live_id,
                        "live_type": p.live_type,
                        "address": payload_json["pod"]["address"].as_str().unwrap_or_default(),
                        "port": payload_json["pod"]["port"].as_u64().unwrap_or(0),
                        "room_id": payload_json["room_id"].as_str().unwrap_or_default(),
                    }));
                }
                Err(e) => {
                    tracing::warn!(
                        "No valid token for live: live_id={}, live_type={}, err={}",
                        p.live_id,
                        p.live_type,
                        e
                    );
                }
            }
        }
    }

    if let Some(path) = args.test_result_file.as_ref() {
        let mut file = File::create(path)?;
        serde_json::to_writer_pretty(&mut file, &test_results)?;
    }
    Ok(())
}
