macro_rules! use_common_crate {
    () => {
        #[allow(unused)]
        use anyhow::Result;
        #[allow(unused)]
        use reqwest::blocking::Response;
        #[allow(unused)]
        use crate::{ApiClient, gen_random_idempotency_key, API_BASE};
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

pub(crate) use use_common_crate;
pub(crate) use define_api_struct;