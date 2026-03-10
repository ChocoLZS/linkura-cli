macro_rules! use_common_crate {
    () => {
        #[allow(unused)]
        use crate::{API_BASE, ApiClient, gen_random_idempotency_key};
        #[allow(unused)]
        use anyhow::Result;
        #[allow(unused)]
        use reqwest::blocking::Response;
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

macro_rules! define_post_method {
    ($name:ident, $path:expr) => {
        pub fn $name(&self) -> Result<Response> {
            let url = format!("{API_BASE}{}", $path);
            let req = self
                .client
                .post(url)
                .headers(self.runtime_header.clone())
                .header("x-idempotency-key", gen_random_idempotency_key());
            // Some endpoints have no logical payload but still require Content-Length.
            Ok(req.json(&serde_json::json!({})).send()?)
        }
    };

    ($name:ident, $path:expr, $request_ty:ty) => {
        pub fn $name(&self, request: &$request_ty) -> Result<Response> {
            let url = format!("{API_BASE}{}", $path);
            let req = self
                .client
                .post(url)
                .headers(self.runtime_header.clone())
                .header("x-idempotency-key", gen_random_idempotency_key());
            Ok(req.json(request).send()?)
        }
    };
}

pub(crate) use define_api_struct;
pub(crate) use define_post_method;
pub(crate) use use_common_crate;
