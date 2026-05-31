use std::fmt;

use crate::{
    get_appstore_version, get_google_play_version,
    macros::{define_api_struct, use_common_crate},
    model::{
        AccountConnectRequest, ArchiveGetArchiveListRequest, ArchiveGetFesArchiveDataRequest,
        ArchiveGetFesTimelineDataRequest, ArchiveGetWithArchiveDataRequest,
        FesliveConnectTokenRequest, FesliveEnterRequest, GetWithStationDataRequest,
        LiveConnectTokenRequest, UserLoginRequest, WithliveEnterRequest,
    },
};
use reqwest::header;
use serde_json::json;

use crate::UA_PREFIX;

use_common_crate!();

/// Helper struct to format Response with body for debugging
pub struct ResponseDebug {
    pub url: String,
    pub status: reqwest::StatusCode,
    pub headers: reqwest::header::HeaderMap,
    pub body: String,
}

#[derive(Debug, Clone, Default)]
pub struct ArchiveListOptions {
    pub limit: Option<u32>,
    pub order: Option<String>,
    pub sort: Option<String>,
    pub live_type: Option<i32>,
}

impl fmt::Debug for ResponseDebug {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Response")
            .field("url", &self.url)
            .field("status", &self.status)
            .field("headers", &self.headers)
            .field("body", &self.body)
            .finish()
    }
}

impl ResponseDebug {
    #[allow(unused)]
    /// Create a ResponseDebug from a reqwest::Response (consumes the response)
    pub async fn from_response(res: reqwest::Response) -> Result<Self> {
        let url = res.url().to_string();
        let status = res.status();
        let headers = res.headers().clone();
        let body = res.text().await?;

        Ok(Self {
            url,
            status,
            headers,
            body,
        })
    }
}
define_api_struct!(AssetsApi);

impl<'a> AssetsApi<'a> {
    pub async fn get_hls_url_from_archive(&self, url: &str) -> Result<String> {
        let res = self.assets_client.get(url).send().await?;
        if res.status() != reqwest::StatusCode::OK {
            return Err(anyhow::anyhow!("Get archive failed: {:?}", res));
        }
        let json: serde_json::Value = res.json().await?;
        let hls_url = format!(
            "{}/{}",
            json["path"].as_str().unwrap(),
            json["playlist_file"].as_str().unwrap()
        );
        Ok(hls_url.to_string())
    }
}

define_api_struct!(HighLevelApi);

impl<'a> HighLevelApi<'a> {
    /// Get x-res-version from headers
    ///
    /// x-res-version is like: `R2504300@XXX`
    ///
    /// Get client version from
    ///
    /// Returns (x-res-version, `app version from website`)
    pub async fn get_app_version(&self) -> Result<(Option<String>, Option<String>)> {
        let app_version = match get_appstore_version().await {
            Some(version) => Some(version),
            None => get_google_play_version().await,
        };
        tracing::info!("Detected app version: {:?}", app_version);
        // empty id login check
        let url = format!("{API_BASE}/user/login");
        let res = self
            .client
            .post(url)
            .headers(self.runtime_header.clone())
            .header("x-idempotency-key", gen_random_idempotency_key())
            .header("x-client-version", app_version.clone().unwrap_or_default())
            .header(
                header::USER_AGENT,
                format!("{UA_PREFIX}/{}", app_version.clone().unwrap_or_default()),
            )
            .json(&json!({
                "player_id": "",
                "device_specific_id": "",
                "version": 1
            }))
            .send()
            .await?;

        let headers = res.headers().clone();
        if res.status() != reqwest::StatusCode::OK {
            tracing::error!(
                "Linkura api request failed: {:?}",
                ResponseDebug::from_response(res).await?
            );
            return Ok((None, app_version));
        }
        let res_version = headers.get("x-res-version").map(|v| {
            let version = v.to_str().unwrap_or_default();
            tracing::debug!("x-res-version: {}", version);
            version.split('@').next().unwrap_or_default().to_string()
        });

        Ok((res_version, app_version))
    }

    /// Returns the `device_specific_id`
    ///
    /// **Response example**
    ///
    /// ```json
    /// {   
    ///     "player_id": "114514",
    ///     "device_specific_id": "1919810",
    ///     "session_token": "1919810",
    ///     "player_name": "yaju senpai",
    ///     "player_level": 114514
    /// }
    /// ```
    pub async fn password_login(&self, id: &str, password: &str) -> Result<String> {
        let request = AccountConnectRequest {
            provider: Some(1),
            player_id: Some(id.to_string()),
            id_token: Some(password.to_string()),
            platform_type: Some(1),
            ..Default::default()
        };
        let body = self.raw().account().connect(&request).await?;
        let device_specific_id = body.device_specific_id.unwrap_or_default();
        if device_specific_id.is_empty() {
            return Err(anyhow::anyhow!("Login failed, device_specific_id is empty"));
        }
        Ok(device_specific_id)
    }

    /// Returns the `session_token`
    ///
    /// **Return example**
    ///
    /// ```json
    /// {   
    ///     ...
    ///     "session_token": "1919810",
    ///     ...
    /// }
    /// ```
    pub async fn device_id_login(&self, id: &str, device_id: &str) -> Result<String> {
        let request = UserLoginRequest {
            player_id: Some(id.to_string()),
            device_specific_id: Some(device_id.to_string()),
            version: Some(1),
            ..Default::default()
        };
        let body = self.raw().user().login(&request).await?;
        let session_token = body.session_token.unwrap_or_default();
        if session_token.is_empty() {
            return Err(anyhow::anyhow!("Login failed"));
        }
        Ok(session_token)
    }

    pub async fn get_plan_list(&self) -> Result<serde_json::Value> {
        let body = self.raw().archive().get_home().await?;
        let mut merged = body.live_archive_list.unwrap_or_default();
        merged.extend(body.trailer_archive_list.unwrap_or_default());
        Ok(serde_json::to_value(merged)?)
    }

    pub async fn get_archive_list(&self, options: ArchiveListOptions) -> Result<serde_json::Value> {
        let request = ArchiveGetArchiveListRequest {
            order: Some(options.order.unwrap_or_else(|| "desc".to_string())),
            characters: Some(Vec::new()),
            limit: Some(options.limit.unwrap_or(4) as i32),
            sort: Some(options.sort.unwrap_or_else(|| "live_start_time".to_string())),
            live_type: options.live_type,
            ..Default::default()
        };
        let body = self.raw().archive().get_archive_list(&request).await?;
        Ok(serde_json::to_value(body.archive_list.unwrap_or_default())?)
    }

    pub async fn get_with_meets_info(&self, id: &str) -> Result<serde_json::Value> {
        let request = WithliveEnterRequest {
            live_id: Some(id.to_string()),
            ..Default::default()
        };
        let body = self.raw().with_live().enter(&request).await?;
        Ok(serde_json::to_value(body)?)
    }

    pub async fn get_with_meets_connect_token(&self, live_id: &str) -> Result<String> {
        let request = LiveConnectTokenRequest {
            live_id: Some(live_id.to_string()),
            ..Default::default()
        };
        let body = self.raw().with_live().connect_token(&request).await?;
        let connect_token = body
            .audience_token
            .clone()
            .ok_or_else(|| anyhow::anyhow!("Get connect token failed: {:?}", body))?;
        Ok(connect_token)
    }

    pub async fn get_fes_live_info(&self, id: &str) -> Result<serde_json::Value> {
        let request = FesliveEnterRequest {
            live_id: Some(id.to_string()),
            ..Default::default()
        };
        let body = self.raw().fes_live().enter(&request).await?;
        Ok(serde_json::to_value(body)?)
    }

    pub async fn get_fes_live_connect_token(&self, live_id: &str) -> Result<String> {
        let request = FesliveConnectTokenRequest {
            live_id: Some(live_id.to_string()),
            ..Default::default()
        };
        let body = self.raw().fes_live().connect_token(&request).await?;
        let connect_token = body
            .audience_token
            .clone()
            .ok_or_else(|| anyhow::anyhow!("Get connect token failed: {:?}", body))?;
        Ok(connect_token)
    }

    pub async fn get_archive_details(&self, id: &str, live_type: u8) -> Result<serde_json::Value> {
        if live_type == 1 {
            let request = ArchiveGetFesArchiveDataRequest {
                archives_id: Some(id.to_string()),
                ..Default::default()
            };
            let body = self.raw().archive().get_fes_archive_data(&request).await?;
            Ok(serde_json::to_value(body)?)
        } else if live_type == 2 {
            let request = ArchiveGetWithArchiveDataRequest {
                archives_id: Some(id.to_string()),
                ..Default::default()
            };
            let body = self.raw().archive().get_with_archive_data(&request).await?;
            Ok(serde_json::to_value(body)?)
        } else {
            Err(anyhow::anyhow!("Unsupported live type: {}", live_type))
        }
    }

    /// Fetch the comment timelines for a live/archive, together with the
    /// metadata (start time and title) needed to name the output file(s).
    ///
    /// `live_type`: `1` = fes live, `2` = with live, `3` = with station.
    ///
    /// A fes live yields three sections (the paginated archive comments, plus
    /// single-shot `live_info` and `lobby_info` snapshots); with live yields two
    /// (archive comments + `live_info`); with station yields one.
    pub async fn get_live_comments(&self, live_id: &str, live_type: u8) -> Result<LiveComments> {
        match live_type {
            1 => {
                let detail = self
                    .raw()
                    .archive()
                    .get_fes_archive_data(&ArchiveGetFesArchiveDataRequest {
                        archives_id: Some(live_id.to_string()),
                        ..Default::default()
                    })
                    .await?;
                let archive_initial = detail.timelines.clone().unwrap_or_default();
                let archive_timelines = self
                    .fetch_archive_timelines(
                        live_id,
                        archive_initial,
                        detail.unixtime_live_rec_started.unwrap_or_default(),
                        true,
                    )
                    .await?;
                let live_info = self
                    .raw()
                    .fes_live()
                    .live_info(live_id.to_string())
                    .await?
                    .timelines
                    .unwrap_or_default();
                let lobby_info = self
                    .raw()
                    .fes_live()
                    .lobby_info(live_id.to_string())
                    .await?
                    .timelines
                    .unwrap_or_default();
                Ok(LiveComments {
                    start_time: detail.live_start_time.unwrap_or_default(),
                    title: detail.title.unwrap_or_default(),
                    kind: "feslive".to_string(),
                    sections: vec![
                        LiveCommentsSection {
                            tags: vec![],
                            timelines: serde_json::to_value(archive_timelines)?,
                        },
                        LiveCommentsSection {
                            tags: vec!["live".to_string()],
                            timelines: serde_json::to_value(live_info)?,
                        },
                        LiveCommentsSection {
                            tags: vec!["live".to_string(), "lobby".to_string()],
                            timelines: serde_json::to_value(lobby_info)?,
                        },
                    ],
                })
            }
            2 => {
                let detail = self
                    .raw()
                    .archive()
                    .get_with_archive_data(&ArchiveGetWithArchiveDataRequest {
                        archives_id: Some(live_id.to_string()),
                        ..Default::default()
                    })
                    .await?;
                let archive_initial = detail.timelines.clone().unwrap_or_default();
                let archive_timelines = self
                    .fetch_archive_timelines(
                        live_id,
                        archive_initial,
                        detail.unixtime_live_rec_started.unwrap_or_default(),
                        false,
                    )
                    .await?;
                let live_info = self
                    .raw()
                    .with_live()
                    .live_info(live_id.to_string())
                    .await?
                    .timelines
                    .unwrap_or_default();
                Ok(LiveComments {
                    start_time: detail.live_start_time.unwrap_or_default(),
                    title: detail.title.unwrap_or_default(),
                    kind: "withlive".to_string(),
                    sections: vec![
                        LiveCommentsSection {
                            tags: vec![],
                            timelines: serde_json::to_value(archive_timelines)?,
                        },
                        LiveCommentsSection {
                            tags: vec!["live".to_string()],
                            timelines: serde_json::to_value(live_info)?,
                        },
                    ],
                })
            }
            3 => {
                let detail = self
                    .raw()
                    .archive()
                    .get_with_station_data(&GetWithStationDataRequest {
                        withstation_id: Some(live_id.to_string()),
                        ..Default::default()
                    })
                    .await?;
                let timelines = self.fetch_withstation_timelines(live_id).await?;
                Ok(LiveComments {
                    start_time: detail.start_time.unwrap_or_default(),
                    title: detail.title.unwrap_or_default(),
                    kind: "withstation".to_string(),
                    sections: vec![LiveCommentsSection {
                        tags: vec![],
                        timelines: serde_json::to_value(timelines)?,
                    }],
                })
            }
            _ => Err(anyhow::anyhow!("Unsupported live type: {}", live_type)),
        }
    }

    /// Build a spinner that reports how many comment pages have been fetched.
    fn comment_progress_bar(label: &str) -> indicatif::ProgressBar {
        let pb = indicatif::ProgressBar::new_spinner();
        pb.set_style(
            indicatif::ProgressStyle::default_spinner()
                .template("{spinner:.green} {msg}")
                .unwrap_or_else(|_| indicatif::ProgressStyle::default_spinner()),
        );
        pb.set_message(format!("{label}: fetching comments..."));
        pb
    }

    /// Page through fes/with live archive comments.
    ///
    /// Seeds with the `timelines` already returned by `get_*_archive_data`, then
    /// starts paging from `play_time_second = 1` and
    /// `timeline_unixtime = unixtime_live_rec_started`. Fes live uses
    /// `get_fes_timeline_data`; with live uses `archive/withlive_info`.
    ///
    /// An empty batch means there are no comments at the current cursor (either
    /// before the first comment or in a gap between comments), so both cursors
    /// step one second at a time until comments resume; pagination ends only
    /// after 600 consecutive empty seconds (~10 minutes). When a batch has data,
    /// the cursor advances by the gap in seconds between the last `submit_time`
    /// of consecutive batches (or one second when non-positive), and the empty
    /// counter resets.
    async fn fetch_archive_timelines(
        &self,
        id: &str,
        initial: Vec<crate::model::LiveTimeline>,
        unixtime_live_rec_started: i64,
        feslive: bool,
    ) -> Result<Vec<crate::model::LiveTimeline>> {
        const MAX_EMPTY_STEPS: u32 = 600;
        let label = if feslive {
            "feslive archive"
        } else {
            "withlive archive"
        };
        let pb = Self::comment_progress_bar(label);
        let mut all = initial;
        let mut play_time_second: i64 = 1;
        let mut timeline_unixtime: i64 = unixtime_live_rec_started;
        let mut consecutive_empty: u32 = 0;
        let mut iterations: u64 = 0;
        loop {
            let batch = if feslive {
                self.raw()
                    .archive()
                    .get_fes_timeline_data(&ArchiveGetFesTimelineDataRequest {
                        archives_id: Some(id.to_string()),
                        play_time_second: Some(play_time_second as i32),
                        timeline_unixtime: Some(timeline_unixtime),
                        ..Default::default()
                    })
                    .await?
                    .timelines
                    .unwrap_or_default()
            } else {
                self.raw()
                    .archive()
                    .withlive_info(
                        id.to_string(),
                        Some(play_time_second as i32),
                        Some(timeline_unixtime),
                    )
                    .await?
                    .timelines
                    .unwrap_or_default()
            };
            iterations += 1;
            if batch.is_empty() {
                // The comment stream may have gaps (before the first comment or
                // between comments), so creep forward one second at a time until
                // comments resume, giving up only after 600 empty seconds.
                consecutive_empty += 1;
                if consecutive_empty == 1 {
                    pb.suspend(|| {
                        tracing::info!(
                            "{label}: timelines empty, waiting for more comments (up to {MAX_EMPTY_STEPS}s)"
                        );
                    });
                }
                pb.set_message(format!(
                    "{label}: timelines empty, waiting ({consecutive_empty}/{MAX_EMPTY_STEPS}s)"
                ));
                pb.tick();
                if consecutive_empty >= MAX_EMPTY_STEPS {
                    break;
                }
                play_time_second += 1;
                timeline_unixtime += 1;
                continue;
            }
            consecutive_empty = 0;
            pb.set_message(format!("{label}: fetched {iterations} page(s)"));
            pb.tick();
            let cur = batch
                .last()
                .and_then(|t| t.submit_time.as_deref())
                .and_then(parse_submit_time);
            all.extend(batch);
            let Some(cur) = cur else {
                break;
            };
            let diff = cur - timeline_unixtime;
            if diff <= 0 {
                // No forward progress from timestamps; nudge both cursors by one
                // second so the next request advances.
                play_time_second += 1;
                timeline_unixtime += 1;
            } else {
                play_time_second += diff;
                timeline_unixtime += diff;
            }
        }
        pb.finish_and_clear();
        Ok(all)
    }

    /// Page through with station comments, following `play_time_ms` (the
    /// `play_time_ms` of the last item in each batch) until an empty batch is
    /// returned.
    async fn fetch_withstation_timelines(
        &self,
        live_id: &str,
    ) -> Result<Vec<crate::model::LiveTimeline>> {
        let pb = Self::comment_progress_bar("withstation");
        let mut all = Vec::new();
        let mut iterations: u64 = 0;
        let mut play_time_ms: Option<i64> = None;
        loop {
            let batch = self
                .raw()
                .withstation()
                .withstation_info(live_id.to_string(), play_time_ms)
                .await?
                .timelines
                .unwrap_or_default();
            iterations += 1;
            pb.set_message(format!("withstation: fetched {iterations} page(s)"));
            pb.tick();
            if batch.is_empty() {
                break;
            }
            let next = batch.last().and_then(|t| t.play_time_ms);
            all.extend(batch);
            if next.is_none() || next == play_time_ms {
                break;
            }
            play_time_ms = next;
        }
        pb.finish_and_clear();
        Ok(all)
    }
}

/// Parse a comment `submit_time` string into a UNIX timestamp in seconds.
fn parse_submit_time(raw: &str) -> Option<i64> {
    if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(raw) {
        return Some(dt.timestamp());
    }
    for fmt in [
        "%Y-%m-%d %H:%M:%S%.f",
        "%Y-%m-%d %H:%M:%S",
        "%Y-%m-%dT%H:%M:%S%.f",
        "%Y-%m-%dT%H:%M:%S",
    ] {
        if let Ok(ndt) = chrono::NaiveDateTime::parse_from_str(raw, fmt) {
            return Some(ndt.and_utc().timestamp());
        }
    }
    None
}

/// A single set of comment timelines belonging to a live.
#[derive(Debug, Clone)]
pub struct LiveCommentsSection {
    /// File-name tags distinguishing this section from others of the same live,
    /// e.g. `["live"]` or `["live", "lobby"]`. Empty for the default file.
    pub tags: Vec<String>,
    /// The `timelines` array extracted from the response.
    pub timelines: serde_json::Value,
}

/// Comment timelines plus the metadata used to name the output file(s).
#[derive(Debug, Clone)]
pub struct LiveComments {
    /// Raw start time string of the archive (e.g. `2024-12-25T19:00:00+09:00`).
    pub start_time: String,
    /// Title of the live.
    pub title: String,
    /// Kind of live: `feslive`, `withlive` or `withstation`.
    pub kind: String,
    /// One section per comment source.
    pub sections: Vec<LiveCommentsSection>,
}
