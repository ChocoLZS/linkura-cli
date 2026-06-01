use crate::macros::{define_api_struct, get_params, use_common_crate};

use_common_crate!();
define_api_struct!(WithstationApi);

impl<'a> WithstationApi<'a> {
    // GET /v1/withstation/withstation_info
    get_params!(
        withstation_info,
        "/withstation/withstation_info",
        crate::model::WithstationInfoResponse,
        live_id: String,
        play_time_ms: Option<i64>,
    );
}
