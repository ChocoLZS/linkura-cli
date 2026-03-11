use crate::macros::{define_api_struct, post, use_common_crate};

use_common_crate!();
define_api_struct!(FriendApi);

impl<'a> FriendApi<'a> {
    // POST /v1/friend/get_list
    post!(
        get_list,
        "/friend/get_list",
        crate::model::FriendGetListRequest,
        crate::model::FriendGetListResponse
    );

    // POST /v1/friend/get_request_list
    post!(
        get_request_list,
        "/friend/get_request_list",
        crate::model::FriendGetRequestListResponse
    );

    // POST /v1/friend/search_name
    post!(
        search_name,
        "/friend/search_name",
        crate::model::FriendSearchNameRequest,
        crate::model::FriendSearchNameResponse
    );

    // POST /v1/friend/search_player_id
    post!(
        search_player_id,
        "/friend/search_player_id",
        crate::model::FriendSearchPlayerIdRequest,
        crate::model::FriendSearchPlayerIdResponse
    );

    // POST /v1/friend/search_recommend
    post!(
        search_recommend,
        "/friend/search_recommend",
        crate::model::FriendSearchRecommendResponse
    );

    // POST /v1/friend/set_approval_request
    post!(
        set_approval_request,
        "/friend/set_approval_request",
        crate::model::FriendSetApprovalRequestRequest,
        crate::model::FriendSetCommonResponse
    );

    // POST /v1/friend/set_approval_request_all
    post!(
        set_approval_request_all,
        "/friend/set_approval_request_all",
        crate::model::FriendSetApprovalRequestAllRequest,
        crate::model::FriendSetCommonResponse
    );

    // POST /v1/friend/set_break_off
    post!(
        set_break_off,
        "/friend/set_break_off",
        crate::model::FriendSetBreakOffRequest,
        crate::model::FriendSetCommonResponse
    );

    // POST /v1/friend/set_refuse_request_all
    post!(
        set_refuse_request_all,
        "/friend/set_refuse_request_all",
        crate::model::FriendSetRefuseRequestAllRequest,
        crate::model::FriendSetCommonResponse
    );

    // POST /v1/friend/set_request
    post!(
        set_request,
        "/friend/set_request",
        crate::model::FriendSetRequestRequest,
        crate::model::FriendSetCommonResponse
    );

    // POST /v1/friend/set_request_cancel
    post!(
        set_request_cancel,
        "/friend/set_request_cancel",
        crate::model::FriendSetRequestCancelRequest,
        crate::model::FriendSetCommonResponse
    );

    // POST /v1/friend/update_request_view_history
    post!(
        update_request_view_history,
        "/friend/update_request_view_history",
        serde_json::Value
    );
}




