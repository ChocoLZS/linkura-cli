use crate::macros::{define_api_struct, post, post_params, use_common_crate};

use_common_crate!();
define_api_struct!(ArchiveApi);

impl<'a> ArchiveApi<'a> {
    // POST /v1/archive/get_archive_list
    post!(
        get_archive_list,
        "/archive/get_archive_list",
        crate::model::GetArchiveListRequest,
        crate::model::GetArchiveListResponse
    );

    // POST /v1/archive/get_channel_list
    post!(
        get_channel_list,
        "/archive/get_channel_list",
        crate::model::GetChannelListResponse
    );

    // POST /v1/archive/get_channel_movie_list
    post!(
        get_channel_movie_list,
        "/archive/get_channel_movie_list",
        crate::model::GetChannelMovieListRequest,
        crate::model::GetChannelMovieListResponse
    );

    // POST /v1/archive/get_fes_archive_data
    post!(
        get_fes_archive_data,
        "/archive/get_fes_archive_data",
        crate::model::GetFesArchiveDataRequest,
        crate::model::GetFesArchiveDataResponse
    );

    // POST /v1/archive/get_fes_timeline_data
    post!(
        get_fes_timeline_data,
        "/archive/get_fes_timeline_data",
        crate::model::GetFesTimelineDataRequest,
        crate::model::GetFesTimelineDataResponse
    );

    // POST /v1/archive/get_home
    post!(
        get_home,
        "/archive/get_home",
        crate::model::GetHomeResponse
    );

    // POST /v1/archive/get_season_list
    post!(
        get_season_list,
        "/archive/get_season_list",
        crate::model::GetSeasonListResponse
    );

    // POST /v1/archive/get_with_archive_data
    post!(
        get_with_archive_data,
        "/archive/get_with_archive_data",
        crate::model::GetWithArchiveDataRequest,
        crate::model::GetWithArchiveDataResponse
    );

    // POST /v1/archive/set_cancel_recommend_channel
    post!(
        set_cancel_recommend_channel,
        "/archive/set_cancel_recommend_channel",
        crate::model::SetCancelRecommendChannelRequest,
        crate::model::ArchiveCommonResponse
    );

    // POST /v1/archive/set_fes_camera
    post!(
        set_fes_camera,
        "/archive/set_fes_camera",
        crate::model::SetFesCameraRequest,
        crate::model::SetFesCameraResponse
    );

    // POST /v1/archive/set_purchase_ticket
    post!(
        set_purchase_ticket,
        "/archive/set_purchase_ticket",
        crate::model::SetPurchaseTicketRequest,
        crate::model::ArchiveCommonResponse
    );

    // POST /v1/archive/set_recommend_channel
    post!(
        set_recommend_channel,
        "/archive/set_recommend_channel",
        crate::model::SetRecommendChannelRequest,
        crate::model::ArchiveCommonResponse
    );

    // POST /v1/archive/withlive_gift
    post!(
        withlive_gift,
        "/archive/withlive_gift",
        crate::model::ArchiveWithliveGiftRequest,
        crate::model::ArchiveWithliveGiftResponse
    );

    // POST /v1/archive/withlive_info
    post_params!(
        withlive_info,
        "/archive/withlive_info",
        crate::model::ArchiveWithliveInfoResponse,
        live_id: String,
        play_time_second: Option<i32>,
        timeline_unixtime: Option<i64>,
    );

    // POST /v1/archive/withlive_prize
    post_params!(
        withlive_prize,
        "/archive/withlive_prize",
        crate::model::ArchiveWithlivePrizeResponse,
        present_box_id: String,
    );

    // POST /v1/archive/withlive_stars
    post!(
        withlive_stars,
        "/archive/withlive_stars",
        crate::model::ArchiveWithliveStarsPostRequest,
        serde_json::Value
    );
}





