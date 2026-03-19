use chrono::Utc;
use linkura_api::ArchiveListOptions;
use linkura_api::model::FesliveLobbyRequest;
use rmcp::{
    Json,
    handler::server::router::tool::ToolRouter,
    handler::server::wrapper::Parameters,
    schemars,
    schemars::JsonSchema,
    tool, tool_router,
};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use tokio::task::JoinSet;

use super::server::LinkuraMcpServer;
use crate::generated::id_to_name::{costume_id_to_name, live_location_id_to_name};

pub fn router() -> ToolRouter<LinkuraMcpServer> {
    LinkuraMcpServer::tool_router()
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct LiveStreamingInfoResponse {
    pub items: Vec<LiveStreamingInfoItem>,
    pub total: usize,
    pub generated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ArchiveListResponse {
    pub items: Vec<ArchiveListItem>,
    pub total: usize,
    pub generated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
pub struct ListArchivesRequest {
    pub limit: Option<u32>,
    pub order: Option<String>,
    pub sort: Option<String>,
    pub live_type: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ArchiveListItem {
    pub archives_id: String,
    pub live_id: String,
    pub title: String,
    pub category: LiveStreamingCategory,
    pub start_at: Option<String>,
    pub end_at: Option<String>,
    pub open_at: Option<String>,
    pub summary: Option<String>,
    pub raw: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ArchiveDetailResponse {
    pub archives_id: String,
    pub live_type: i32,
    pub category: LiveStreamingCategory,
    pub title: Option<String>,
    pub summary: Option<String>,
    pub live_location_id: Option<u32>,
    pub live_location_name: Option<String>,
    pub costume_ids: Vec<u32>,
    pub costume_names: Vec<String>,
    pub raw: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GetArchiveDetailRequest {
    pub archives_id: String,
    pub live_type: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct LiveStreamingInfoItem {
    pub id: String,
    pub title: String,
    pub category: LiveStreamingCategory,
    pub status: LiveStreamingStatus,
    pub start_at: Option<String>,
    pub open_at: Option<String>,
    pub end_at: Option<String>,
    pub summary: Option<String>,
    pub live_location_id: Option<u32>,
    pub live_location_name: Option<String>,
    pub costume_ids: Vec<u32>,
    pub costume_names: Vec<String>,
    pub raw: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum LiveStreamingCategory {
    WithMeets,
    FesLive,
    Live,
    Trailer,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum LiveStreamingStatus {
    Upcoming,
    Live,
    Ended,
    Unknown,
}

#[tool_router(router = tool_router)]
impl LinkuraMcpServer {
    #[tool(
        name = "list_live_streaming_info",
        description = "List latest Linkura live streaming information, including With Meets, Fes Live, Live, and Trailers."
    )]
    pub async fn list_live_streaming_info(
        &self,
    ) -> Result<Json<LiveStreamingInfoResponse>, rmcp::ErrorData> {
        let plan_list = self
            .api_client
            .high_level()
            .get_plan_list()
            .await
            .map_err(|err| rmcp::ErrorData::internal_error(err.to_string(), None))?;

        let items: Vec<Value> = plan_list
            .as_array()
            .map(|items| items.to_vec())
            .unwrap_or_default();

        let mut enriched_items = items.iter().map(base_live_streaming_item).collect::<Vec<_>>();
        let now = Utc::now();
        let mut join_set = JoinSet::new();

        for (index, item) in items.into_iter().enumerate() {
            if !should_fetch_live_streaming_detail(&item, now) {
                continue;
            }

            let server = self.clone();
            join_set.spawn(async move {
                let detail = server.fetch_live_streaming_detail(&item).await;
                (index, item, detail)
            });
        }

        while let Some(task) = join_set.join_next().await {
            let Ok((index, item, detail)) = task else {
                continue;
            };

            if let Some(detail) = detail {
                enriched_items[index] = merge_live_streaming_item(item, detail);
            }
        }

        Ok(Json(LiveStreamingInfoResponse {
            total: enriched_items.len(),
            items: enriched_items,
            generated_at: Utc::now().to_rfc3339(),
        }))
    }

    #[tool(
        name = "list_archives",
        description = "List latest Linkura archive items."
    )]
    pub async fn list_archives(
        &self,
        Parameters(ListArchivesRequest {
            limit,
            order,
            sort,
            live_type,
        }): Parameters<ListArchivesRequest>,
    ) -> Result<Json<ArchiveListResponse>, rmcp::ErrorData> {
        let archive_list = self
            .api_client
            .high_level()
            .get_archive_list(ArchiveListOptions {
                limit: Some(limit.unwrap_or(20)),
                order,
                sort,
                live_type,
            })
            .await
            .map_err(|err| rmcp::ErrorData::internal_error(err.to_string(), None))?;

        let items = archive_list
            .as_array()
            .map(|items| items.iter().map(map_archive_list_item).collect::<Vec<_>>())
            .unwrap_or_default();

        Ok(Json(ArchiveListResponse {
            total: items.len(),
            items,
            generated_at: Utc::now().to_rfc3339(),
        }))
    }

    #[tool(
        name = "get_archive_detail",
        description = "Get Linkura archive detail for a specific archive item."
    )]
    pub async fn get_archive_detail(
        &self,
        Parameters(GetArchiveDetailRequest {
            archives_id,
            live_type,
        }): Parameters<GetArchiveDetailRequest>,
    ) -> Result<Json<ArchiveDetailResponse>, rmcp::ErrorData> {
        let raw = self
            .api_client
            .high_level()
            .get_archive_details(&archives_id, live_type as u8)
            .await
            .map_err(|err| rmcp::ErrorData::internal_error(err.to_string(), None))?;

        Ok(Json(map_archive_detail(archives_id, live_type, raw)))
    }
}

impl LinkuraMcpServer {
    async fn fetch_live_streaming_detail(&self, value: &Value) -> Option<Value> {
        let live_id = value.get("live_id").and_then(Value::as_str)?;
        let category = map_category(value);

        match category {
            LiveStreamingCategory::WithMeets => self
                .api_client
                .high_level()
                .get_with_meets_info(live_id)
                .await
                .ok(),
            LiveStreamingCategory::FesLive => {
                let lobby_request = FesliveLobbyRequest {
                    live_id: Some(live_id.to_string()),
                    ..Default::default()
                };
                let _ = self.api_client.raw().fes_live().lobby(&lobby_request).await;
                self.api_client
                    .high_level()
                    .get_fes_live_info(live_id)
                    .await
                    .ok()
            }
            _ => None,
        }
    }
}

fn base_live_streaming_item(value: &Value) -> LiveStreamingInfoItem {
    let start_at = value
        .get("live_start_time")
        .and_then(Value::as_str)
        .map(str::to_string);
    let open_at = value
        .get("open_time")
        .and_then(Value::as_str)
        .map(str::to_string);
    let end_at = value
        .get("live_end_time")
        .and_then(Value::as_str)
        .map(str::to_string);
    let live_location_id = value.get("live_location_id").and_then(value_as_u32);
    let costume_ids = value
        .get("costume_ids")
        .and_then(Value::as_array)
        .map(|values| values.iter().filter_map(value_as_u32).collect::<Vec<_>>())
        .unwrap_or_default();

    LiveStreamingInfoItem {
        id: first_string(value, &["live_id", "id"]),
        title: first_string(value, &["name", "title"]),
        category: map_category(value),
        status: map_status(start_at.as_deref(), open_at.as_deref(), end_at.as_deref()),
        start_at,
        open_at,
        end_at,
        summary: value
            .get("description")
            .and_then(Value::as_str)
            .map(str::to_string),
        live_location_id,
        live_location_name: live_location_id
            .and_then(|id| live_location_id_to_name(id).map(str::to_string)),
        costume_names: costume_ids
            .iter()
            .filter_map(|id| costume_id_to_name(*id).map(str::to_string))
            .collect(),
        costume_ids,
        raw: value.clone(),
    }
}

fn map_archive_list_item(value: &Value) -> ArchiveListItem {
    ArchiveListItem {
        archives_id: first_string(value, &["archives_id"]),
        live_id: first_string(value, &["live_id"]),
        title: first_string(value, &["name", "title"]),
        category: map_category(value),
        start_at: value
            .get("live_start_time")
            .and_then(Value::as_str)
            .map(str::to_string),
        end_at: value
            .get("live_end_time")
            .and_then(Value::as_str)
            .map(str::to_string),
        open_at: value
            .get("open_time")
            .and_then(Value::as_str)
            .map(str::to_string),
        summary: value
            .get("description")
            .and_then(Value::as_str)
            .map(str::to_string),
        raw: value.clone(),
    }
}

fn map_archive_detail(archives_id: String, live_type: i32, raw: Value) -> ArchiveDetailResponse {
    let live_location_id = raw.get("live_location_id").and_then(value_as_u32);
    let costume_ids = raw
        .get("costume_ids")
        .and_then(Value::as_array)
        .map(|values| values.iter().filter_map(value_as_u32).collect::<Vec<_>>())
        .unwrap_or_default();

    ArchiveDetailResponse {
        archives_id,
        live_type,
        category: map_category_from_live_type(live_type),
        title: raw
            .get("title")
            .and_then(Value::as_str)
            .map(str::to_string),
        summary: raw
            .get("description")
            .and_then(Value::as_str)
            .map(str::to_string),
        live_location_id,
        live_location_name: live_location_id
            .and_then(|id| live_location_id_to_name(id).map(str::to_string)),
        costume_names: costume_ids
            .iter()
            .filter_map(|id| costume_id_to_name(*id).map(str::to_string))
            .collect(),
        costume_ids,
        raw,
    }
}

fn merge_live_streaming_item(fallback: Value, detail: Value) -> LiveStreamingInfoItem {
    let mut item = base_live_streaming_item(&fallback);
    let (live_location_id, live_location_name, costume_ids, costume_names, raw) =
        extract_live_streaming_names(&fallback, detail);

    item.live_location_id = live_location_id;
    item.live_location_name = live_location_name;
    item.costume_ids = costume_ids;
    item.costume_names = costume_names;
    item.raw = raw;
    item
}

fn should_fetch_live_streaming_detail(value: &Value, now: chrono::DateTime<Utc>) -> bool {
    let Some(open_at) = value
        .get("open_time")
        .and_then(Value::as_str)
        .and_then(parse_rfc3339_utc)
    else {
        return false;
    };

    now >= open_at
}

fn extract_live_streaming_names(
    fallback: &Value,
    detail: Value,
) -> (Option<u32>, Option<String>, Vec<u32>, Vec<String>, Value) {
    let live_location_id = detail
        .get("live_location_id")
        .and_then(value_as_u32)
        .or_else(|| fallback.get("live_location_id").and_then(value_as_u32));
    let live_location_name =
        live_location_id.and_then(|id| live_location_id_to_name(id).map(str::to_string));

    let costume_ids = detail
        .get("costume_ids")
        .and_then(Value::as_array)
        .map(|values| values.iter().filter_map(value_as_u32).collect::<Vec<_>>())
        .or_else(|| {
            fallback
                .get("costume_ids")
                .and_then(Value::as_array)
                .map(|values| values.iter().filter_map(value_as_u32).collect::<Vec<_>>())
        })
        .unwrap_or_default();
    let costume_names = costume_ids
        .iter()
        .filter_map(|id| costume_id_to_name(*id).map(str::to_string))
        .collect::<Vec<_>>();

    let raw = merge_detail_into_raw(fallback, detail);

    (
        live_location_id,
        live_location_name,
        costume_ids,
        costume_names,
        raw,
    )
}

fn merge_detail_into_raw(fallback: &Value, detail: Value) -> Value {
    match (fallback.as_object(), detail) {
        (Some(base), Value::Object(detail_map)) => {
            let mut merged: Map<String, Value> = base.clone();
            merged.extend(detail_map);
            Value::Object(merged)
        }
        (_, other) => other,
    }
}

fn value_as_u32(value: &Value) -> Option<u32> {
    value.as_u64().and_then(|v| u32::try_from(v).ok())
}

fn first_string(value: &Value, keys: &[&str]) -> String {
    keys.iter()
        .find_map(|key| value.get(key).and_then(Value::as_str))
        .unwrap_or_default()
        .to_string()
}

fn map_category(value: &Value) -> LiveStreamingCategory {
    match value.get("live_type").and_then(Value::as_u64) {
        Some(live_type) => map_category_from_live_type(live_type as i32),
        None => {
            let title = first_string(value, &["name", "title"]).to_ascii_lowercase();
            if title.contains("with meets") {
                LiveStreamingCategory::WithMeets
            } else if title.contains("fes live") {
                LiveStreamingCategory::FesLive
            } else if title.contains("trailer") {
                LiveStreamingCategory::Trailer
            } else {
                LiveStreamingCategory::Unknown
            }
        }
    }
}

fn map_category_from_live_type(live_type: i32) -> LiveStreamingCategory {
    match live_type {
        2 => LiveStreamingCategory::WithMeets,
        1 => LiveStreamingCategory::FesLive,
        _ => LiveStreamingCategory::Live,
    }
}

fn map_status(
    start_at: Option<&str>,
    open_at: Option<&str>,
    end_at: Option<&str>,
) -> LiveStreamingStatus {
    let now = Utc::now();

    let start_at = start_at.and_then(parse_rfc3339_utc);
    let open_at = open_at.and_then(parse_rfc3339_utc);
    let end_at = end_at.and_then(parse_rfc3339_utc);

    if let Some(end_at) = end_at {
        if now > end_at {
            return LiveStreamingStatus::Ended;
        }
    }

    if let Some(start_at) = start_at {
        if now >= start_at {
            return LiveStreamingStatus::Live;
        }
    }

    if let Some(open_at) = open_at {
        if now < open_at {
            return LiveStreamingStatus::Upcoming;
        }
        return LiveStreamingStatus::Live;
    }

    LiveStreamingStatus::Unknown
}

fn parse_rfc3339_utc(value: &str) -> Option<chrono::DateTime<Utc>> {
    chrono::DateTime::parse_from_rfc3339(value)
        .ok()
        .map(|dt| dt.with_timezone(&Utc))
}
