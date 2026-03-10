use crate::macros::{define_api_struct, define_post_method, use_common_crate};

use_common_crate!();
define_api_struct!(CircleApi);

impl<'a> CircleApi<'a> {
    // POST /v1/circle/get_chat_log_list
    define_post_method!(
        get_chat_log_list,
        "/circle/get_chat_log_list",
        crate::model::CircleGetChatLogListRequest
    );

    // POST /v1/circle/get_circle_top_info
    define_post_method!(
        get_circle_top_info,
        "/circle/get_circle_top_info",
        crate::model::CircleGetCircleTopInfoRequest
    );

    // POST /v1/circle/get_detail
    define_post_method!(
        get_detail,
        "/circle/get_detail",
        crate::model::CircleGetDetailRequest
    );

    // POST /v1/circle/get_info
    define_post_method!(
        get_info,
        "/circle/get_info",
        crate::model::CircleGetInfoRequest
    );

    // POST /v1/circle/get_invite_and_join_info
    define_post_method!(
        get_invite_and_join_info,
        "/circle/get_invite_and_join_info",
        crate::model::CircleGetInviteAndJoinInfoRequest
    );

    // POST /v1/circle/get_invite_list
    define_post_method!(
        get_invite_list,
        "/circle/get_invite_list",
        crate::model::CircleGetInviteListRequest
    );

    // POST /v1/circle/get_list
    define_post_method!(
        get_list,
        "/circle/get_list",
        crate::model::CircleGetListRequest
    );

    // POST /v1/circle/set_approve_invite
    define_post_method!(
        set_approve_invite,
        "/circle/set_approve_invite",
        crate::model::CircleSetApproveInviteRequest
    );

    // POST /v1/circle/set_approve_join
    define_post_method!(
        set_approve_join,
        "/circle/set_approve_join",
        crate::model::CircleSetApproveJoinRequest
    );

    // POST /v1/circle/set_cancel_invite
    define_post_method!(
        set_cancel_invite,
        "/circle/set_cancel_invite",
        crate::model::CircleSetCancelInviteRequest
    );

    // POST /v1/circle/set_cancel_join
    define_post_method!(
        set_cancel_join,
        "/circle/set_cancel_join",
        crate::model::CircleSetCancelJoinRequest
    );

    // POST /v1/circle/set_chat_message
    define_post_method!(
        set_chat_message,
        "/circle/set_chat_message",
        crate::model::CircleSetChatMessageRequest
    );

    // POST /v1/circle/set_create
    define_post_method!(
        set_create,
        "/circle/set_create",
        crate::model::CircleSetCreateRequest
    );

    // POST /v1/circle/set_dismissal
    define_post_method!(
        set_dismissal,
        "/circle/set_dismissal",
        crate::model::CircleSetDismissalRequest
    );

    // POST /v1/circle/set_dissolution
    define_post_method!(
        set_dissolution,
        "/circle/set_dissolution",
        crate::model::CircleSetDissolutionRequest
    );

    // POST /v1/circle/set_donation
    define_post_method!(
        set_donation,
        "/circle/set_donation",
        crate::model::CircleSetDonationRequest
    );

    // POST /v1/circle/set_expulsion
    define_post_method!(
        set_expulsion,
        "/circle/set_expulsion",
        crate::model::CircleSetExpulsionRequest
    );

    // POST /v1/circle/set_invite
    define_post_method!(
        set_invite,
        "/circle/set_invite",
        crate::model::CircleSetInviteRequest
    );

    // POST /v1/circle/set_item_request
    define_post_method!(
        set_item_request,
        "/circle/set_item_request",
        crate::model::CircleSetItemRequestRequest
    );

    // POST /v1/circle/set_join
    define_post_method!(
        set_join,
        "/circle/set_join",
        crate::model::CircleSetJoinRequest
    );

    // POST /v1/circle/set_out
    define_post_method!(
        set_out,
        "/circle/set_out",
        crate::model::CircleSetOutRequest
    );

    // POST /v1/circle/set_position
    define_post_method!(
        set_position,
        "/circle/set_position",
        crate::model::CircleSetPositionRequest
    );

    // POST /v1/circle/set_reject_invite
    define_post_method!(
        set_reject_invite,
        "/circle/set_reject_invite",
        crate::model::CircleSetRejectInviteRequest
    );

    // POST /v1/circle/set_reject_join
    define_post_method!(
        set_reject_join,
        "/circle/set_reject_join",
        crate::model::CircleSetRejectJoinRequest
    );

    // POST /v1/circle/set_setting
    define_post_method!(
        set_setting,
        "/circle/set_setting",
        crate::model::CircleSetSettingRequest
    );

    // POST /v1/circle/set_transfer_leader
    define_post_method!(
        set_transfer_leader,
        "/circle/set_transfer_leader",
        crate::model::CircleSetTransferLeaderRequest
    );
}
