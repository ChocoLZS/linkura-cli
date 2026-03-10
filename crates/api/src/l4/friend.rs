use crate::macros::{define_api_struct, define_post_method, use_common_crate};

use_common_crate!();
define_api_struct!(FriendApi);

impl<'a> FriendApi<'a> {
    // POST /v1/friend/get_list
    define_post_method!(
        get_list,
        "/friend/get_list",
        crate::model::FriendGetListRequest
    );

    // POST /v1/friend/get_request_list
    define_post_method!(
        get_request_list,
        "/friend/get_request_list",
        crate::model::FriendGetRequestListRequest
    );

    // POST /v1/friend/search_name
    define_post_method!(
        search_name,
        "/friend/search_name",
        crate::model::FriendSearchNameRequest
    );

    // POST /v1/friend/search_player_id
    define_post_method!(
        search_player_id,
        "/friend/search_player_id",
        crate::model::FriendSearchPlayerIdRequest
    );

    // POST /v1/friend/search_recommend
    define_post_method!(
        search_recommend,
        "/friend/search_recommend",
        crate::model::FriendSearchRecommendRequest
    );

    // POST /v1/friend/set_approval_request
    define_post_method!(
        set_approval_request,
        "/friend/set_approval_request",
        crate::model::FriendSetApprovalRequestRequest
    );

    // POST /v1/friend/set_approval_request_all
    define_post_method!(
        set_approval_request_all,
        "/friend/set_approval_request_all",
        crate::model::FriendSetApprovalRequestAllRequest
    );

    // POST /v1/friend/set_break_off
    define_post_method!(
        set_break_off,
        "/friend/set_break_off",
        crate::model::FriendSetBreakOffRequest
    );

    // POST /v1/friend/set_refuse_request_all
    define_post_method!(
        set_refuse_request_all,
        "/friend/set_refuse_request_all",
        crate::model::FriendSetRefuseRequestAllRequest
    );

    // POST /v1/friend/set_request
    define_post_method!(
        set_request,
        "/friend/set_request",
        crate::model::FriendSetRequestRequest
    );

    // POST /v1/friend/set_request_cancel
    define_post_method!(
        set_request_cancel,
        "/friend/set_request_cancel",
        crate::model::FriendSetRequestCancelRequest
    );

    // POST /v1/friend/update_request_view_history
    define_post_method!(
        update_request_view_history,
        "/friend/update_request_view_history",
        crate::model::FriendUpdateRequestViewHistoryRequest
    );
}
