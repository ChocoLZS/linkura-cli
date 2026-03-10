use crate::macros::{define_api_struct, define_post_method, use_common_crate};

use_common_crate!();
define_api_struct!(ArchiveApi);

impl<'a> ArchiveApi<'a> {
    // POST /v1/archive/get_archive_list
    define_post_method!(
        get_archive_list,
        "/archive/get_archive_list",
        crate::model::ArchiveGetArchiveListRequest
    );

    // POST /v1/archive/get_channel_list
    define_post_method!(get_channel_list, "/archive/get_channel_list");

    // POST /v1/archive/get_channel_movie_list
    define_post_method!(
        get_channel_movie_list,
        "/archive/get_channel_movie_list",
        crate::model::ArchiveGetChannelMovieListRequest
    );

    // POST /v1/archive/get_fes_archive_data
    define_post_method!(
        get_fes_archive_data,
        "/archive/get_fes_archive_data",
        crate::model::ArchiveGetFesArchiveDataRequest
    );

    // POST /v1/archive/get_fes_timeline_data
    define_post_method!(
        get_fes_timeline_data,
        "/archive/get_fes_timeline_data",
        crate::model::ArchiveGetFesTimelineDataRequest
    );

    // POST /v1/archive/get_home
    define_post_method!(get_home, "/archive/get_home");

    // POST /v1/archive/get_season_list
    define_post_method!(get_season_list, "/archive/get_season_list");

    // POST /v1/archive/get_with_archive_data
    define_post_method!(
        get_with_archive_data,
        "/archive/get_with_archive_data",
        crate::model::ArchiveGetWithArchiveDataRequest
    );

    // POST /v1/archive/set_cancel_recommend_channel
    define_post_method!(
        set_cancel_recommend_channel,
        "/archive/set_cancel_recommend_channel",
        crate::model::ArchiveSetCancelRecommendChannelRequest
    );

    // POST /v1/archive/set_fes_camera
    define_post_method!(
        set_fes_camera,
        "/archive/set_fes_camera",
        crate::model::ArchiveSetFesCameraRequest
    );

    // POST /v1/archive/set_purchase_ticket
    define_post_method!(
        set_purchase_ticket,
        "/archive/set_purchase_ticket",
        crate::model::ArchiveSetPurchaseTicketRequest
    );

    // POST /v1/archive/set_recommend_channel
    define_post_method!(
        set_recommend_channel,
        "/archive/set_recommend_channel",
        crate::model::ArchiveSetRecommendChannelRequest
    );

    // POST /v1/archive/withlive_gift
    define_post_method!(
        withlive_gift,
        "/archive/withlive_gift",
        crate::model::ArchiveWithliveGiftRequest
    );

    // POST /v1/archive/withlive_info
    define_post_method!(
        withlive_info,
        "/archive/withlive_info",
        crate::model::ArchiveWithliveInfoRequest
    );

    // POST /v1/archive/withlive_prize
    define_post_method!(
        withlive_prize,
        "/archive/withlive_prize",
        crate::model::ArchiveWithlivePrizeRequest
    );

    // POST /v1/archive/withlive_stars
    define_post_method!(
        withlive_stars,
        "/archive/withlive_stars",
        crate::model::ArchiveWithliveStarsRequest
    );
}
