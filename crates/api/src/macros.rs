use anyhow::Result;
use reqwest::Response;
use serde::de::DeserializeOwned;

pub(crate) async fn parse_response<T: DeserializeOwned>(res: Response, path: &str) -> Result<T> {
    let status = res.status();
    let body = res.text().await.unwrap_or_default();
    if !status.is_success() {
        return Err(anyhow::anyhow!(
            "POST {} failed: {} {}",
            path,
            status,
            body
        ));
    }
    match serde_json::from_str::<T>(&body) {
        Ok(parsed) => Ok(parsed),
        Err(err) => {
            let preview = if body.len() > 8192 {
                format!("{}... (truncated, {} bytes)", &body[..8192], body.len())
            } else {
                body
            };
            Err(anyhow::anyhow!(
                "error decoding response body from {}: {} (status: {}) raw body: {}",
                path,
                err,
                status,
                preview
            ))
        }
    }
}

macro_rules! use_common_crate {
    () => {
        #[allow(unused)]
        use crate::{API_BASE, ApiClient, gen_random_idempotency_key};
        #[allow(unused)]
        use anyhow::Result;
        #[allow(unused)]
        use reqwest::Response;
        use std::ops::Deref;
    };
}

macro_rules! define_api_struct {
    ($name:ident) => {
        pub struct $name<'a> {
            pub(super) api: &'a ApiClient,
        }

        impl<'a> Deref for $name<'a> {
            type Target = ApiClient;

            fn deref(&self) -> &Self::Target {
                self.api
            }
        }
    };
}

macro_rules! post {
    ($name:ident, $path:expr, $response_ty:ty) => {
        pub async fn $name(&self) -> Result<$response_ty> {
            let url = format!("{API_BASE}{}", $path);
            let req = self
                .client
                .post(url)
                .headers(self.runtime_header.clone())
                .header("x-idempotency-key", gen_random_idempotency_key());
            // Some endpoints have no logical payload but still require Content-Length.
            let res = req.json(&serde_json::json!({})).send().await?;
            crate::macros::parse_response(res, $path).await
        }
    };

    ($name:ident, $path:expr, $request_ty:ty, $response_ty:ty) => {
        pub async fn $name(&self, request: &$request_ty) -> Result<$response_ty> {
            let url = format!("{API_BASE}{}", $path);
            let req = self
                .client
                .post(url)
                .headers(self.runtime_header.clone())
                .header("x-idempotency-key", gen_random_idempotency_key());
            let res = req.json(request).send().await?;
            crate::macros::parse_response(res, $path).await
        }
    };
}

macro_rules! post_params {
    ($name:ident, $path:expr, $response_ty:ty, $( $param:ident : $param_ty:ty ),+ $(,)?) => {
        pub async fn $name(&self, $( $param: $param_ty ),+ ) -> Result<$response_ty> {
            let url = format!("{API_BASE}{}", $path);
            let req = self
                .client
                .post(url)
                .headers(self.runtime_header.clone())
                .header("x-idempotency-key", gen_random_idempotency_key());
            let payload = serde_json::json!({ $( stringify!($param): $param ),+ });
            let res = req.json(&payload).send().await?;
            crate::macros::parse_response(res, $path).await
        }
    };
}

pub(crate) use define_api_struct;
pub(crate) use post;
pub(crate) use post_params;
pub(crate) use use_common_crate;
