use crate::macros::{define_api_struct, post, post_params, use_common_crate};

use_common_crate!();
define_api_struct!(CircleApi);

impl<'a> CircleApi<'a> {
    // POST /v1/circle/get_chat_log_list
    post_params!(
        get_chat_log_list,
        "/circle/get_chat_log_list",
        crate::model::CircleGetChatLogListResponse,
        diff_order_id: Option<i64>,
        is_item_request: Option<bool>,
        already_read_order_id: Option<i64>,
    );

    // POST /v1/circle/get_circle_top_info
    post!(
        get_circle_top_info,
        "/circle/get_circle_top_info",
        crate::model::CircleTopGetInfoResponse
    );

    // POST /v1/circle/get_detail
    post!(
        get_detail,
        "/circle/get_detail",
        crate::model::CircleGetDetailRequest,
        crate::model::CircleGetDetailResponse
    );

    // POST /v1/circle/get_info
    post!(
        get_info,
        "/circle/get_info",
        crate::model::CircleGetInfoRequest,
        crate::model::CircleGetInfoResponse
    );

    // POST /v1/circle/get_invite_and_join_info
    post!(
        get_invite_and_join_info,
        "/circle/get_invite_and_join_info",
        crate::model::CircleGetInviteAndJoinInfoRequest,
        crate::model::CircleGetInviteAndJoinInfoResponse
    );

    // POST /v1/circle/get_invite_list
    post!(
        get_invite_list,
        "/circle/get_invite_list",
        crate::model::CircleGetInviteListResponse
    );

    // POST /v1/circle/get_list
    post!(
        get_list,
        "/circle/get_list",
        crate::model::CircleGetListRequest,
        crate::model::CircleGetListResponse
    );

    // POST /v1/circle/set_approve_invite
    post!(
        set_approve_invite,
        "/circle/set_approve_invite",
        crate::model::CircleSetApproveInviteRequest,
        crate::model::CircleSetApproveInviteResponse
    );

    // POST /v1/circle/set_approve_join
    post!(
        set_approve_join,
        "/circle/set_approve_join",
        crate::model::CircleSetApproveJoinRequest,
        crate::model::CircleSetApproveJoinResponse
    );

    // POST /v1/circle/set_cancel_invite
    post!(
        set_cancel_invite,
        "/circle/set_cancel_invite",
        crate::model::CircleSetCancelInviteRequest,
        crate::model::CircleSetCancelInviteResponse
    );

    // POST /v1/circle/set_cancel_join
    post!(
        set_cancel_join,
        "/circle/set_cancel_join",
        crate::model::CircleSetCancelJoinRequest,
        crate::model::CircleSetCancelJoinResponse
    );

    // POST /v1/circle/set_chat_message
    post!(
        set_chat_message,
        "/circle/set_chat_message",
        crate::model::CircleSetChatMessageRequest,
        crate::model::CircleSetChatMessageResponse
    );

    // POST /v1/circle/set_create
    post!(
        set_create,
        "/circle/set_create",
        crate::model::CircleSetCreateRequest,
        crate::model::CircleSetCreateResponse
    );

    // POST /v1/circle/set_dismissal
    post!(
        set_dismissal,
        "/circle/set_dismissal",
        crate::model::CircleSetDismissalRequest,
        crate::model::CircleSetDismissalResponse
    );

    // POST /v1/circle/set_dissolution
    post!(
        set_dissolution,
        "/circle/set_dissolution",
        crate::model::CircleSetDissolutionResponse
    );

    // POST /v1/circle/set_donation
    post!(
        set_donation,
        "/circle/set_donation",
        crate::model::CircleSetDonationRequest,
        crate::model::CircleSetDonationResponse
    );

    // POST /v1/circle/set_expulsion
    post!(
        set_expulsion,
        "/circle/set_expulsion",
        crate::model::CircleSetExpulsionRequest,
        crate::model::CircleSetExpulsionResponse
    );

    // POST /v1/circle/set_invite
    post!(
        set_invite,
        "/circle/set_invite",
        crate::model::CircleSetInviteRequest,
        crate::model::CircleSetInviteResponse
    );

    // POST /v1/circle/set_item_request
    post!(
        set_item_request,
        "/circle/set_item_request",
        crate::model::CircleSetItemRequestRequest,
        crate::model::CircleSetItemRequestResponse
    );

    // POST /v1/circle/set_join
    post!(
        set_join,
        "/circle/set_join",
        crate::model::CircleSetJoinRequest,
        crate::model::CircleSetJoinResponse
    );

    // POST /v1/circle/set_out
    post!(
        set_out,
        "/circle/set_out",
        crate::model::CircleSetOutResponse
    );

    // POST /v1/circle/set_position
    post!(
        set_position,
        "/circle/set_position",
        crate::model::CircleSetPositionRequest,
        crate::model::CircleSetPositionResponse
    );

    // POST /v1/circle/set_reject_invite
    post!(
        set_reject_invite,
        "/circle/set_reject_invite",
        crate::model::CircleSetRejectInviteRequest,
        crate::model::CircleSetRejectInviteResponse
    );

    // POST /v1/circle/set_reject_join
    post!(
        set_reject_join,
        "/circle/set_reject_join",
        crate::model::CircleSetRejectJoinRequest,
        crate::model::CircleSetRejectJoinResponse
    );

    // POST /v1/circle/set_setting
    post!(
        set_setting,
        "/circle/set_setting",
        crate::model::CircleSetSettingRequest,
        crate::model::CircleSetSettingResponse
    );

    // POST /v1/circle/set_transfer_leader
    post!(
        set_transfer_leader,
        "/circle/set_transfer_leader",
        crate::model::CircleSetTransferLeaderRequest,
        crate::model::CircleSetTransferLeaderResponse
    );
}





