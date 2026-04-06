use anyhow::{Context, Result};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct PlanItem {
    pub live_id: String,
    pub live_type: u64,
    pub name: String,
    pub live_open_time: DateTime<Utc>,
    pub live_start_time: DateTime<Utc>,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct JobKey {
    pub live_id: String,
    pub live_type: u64,
}

impl PlanItem {
    pub fn key(&self) -> JobKey {
        JobKey {
            live_id: self.live_id.clone(),
            live_type: self.live_type,
        }
    }
}

pub fn parse_plan_items(value: serde_json::Value) -> Result<Vec<PlanItem>> {
    let arr = value
        .as_array()
        .context("get_plan_list should return an array")?;

    let mut items = Vec::new();
    for item in arr {
        let live_id = match item.get("live_id").and_then(|v| v.as_str()) {
            Some(v) if !v.is_empty() => v.to_string(),
            _ => continue,
        };
        let live_type = item.get("live_type").and_then(|v| v.as_u64()).unwrap_or(2);
        let name = item
            .get("name")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown")
            .to_string();
        let Some(open_time_str) = item.get("open_time").and_then(|v| v.as_str()) else {
            continue;
        };
        let Ok(open_time) = DateTime::parse_from_rfc3339(open_time_str) else {
            continue;
        };
        let live_start_time = item
            .get("live_start_time")
            .and_then(|v| v.as_str())
            .and_then(|v| DateTime::parse_from_rfc3339(v).ok())
            .map(|v| v.with_timezone(&Utc))
            .unwrap_or_else(|| open_time.with_timezone(&Utc));
        items.push(PlanItem {
            live_id,
            live_type,
            name,
            live_open_time: open_time.with_timezone(&Utc),
            live_start_time,
        });
    }
    Ok(items)
}
