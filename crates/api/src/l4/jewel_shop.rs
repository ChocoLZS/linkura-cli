use crate::macros::{define_api_struct, define_post_method, use_common_crate};

use_common_crate!();
define_api_struct!(JewelShopApi);

impl<'a> JewelShopApi<'a> {
    // POST /v1/jewel_shop/get_birthday
    define_post_method!(
        get_birthday,
        "/jewel_shop/get_birthday",
        crate::model::JewelShopGetBirthdayRequest
    );

    // POST /v1/jewel_shop/get_list
    define_post_method!(
        get_list,
        "/jewel_shop/get_list",
        crate::model::JewelShopGetListRequest
    );

    // POST /v1/jewel_shop/get_membership_list
    define_post_method!(
        get_membership_list,
        "/jewel_shop/get_membership_list",
        crate::model::JewelShopGetMembershipListRequest
    );

    // POST /v1/jewel_shop/set_birthday
    define_post_method!(
        set_birthday,
        "/jewel_shop/set_birthday",
        crate::model::JewelShopSetBirthdayRequest
    );

    // POST /v1/jewel_shop/set_membership_purchase
    define_post_method!(
        set_membership_purchase,
        "/jewel_shop/set_membership_purchase",
        crate::model::JewelShopSetMembershipPurchaseRequest
    );

    // POST /v1/jewel_shop/set_purchase
    define_post_method!(
        set_purchase,
        "/jewel_shop/set_purchase",
        crate::model::JewelShopSetPurchaseRequest
    );
}
