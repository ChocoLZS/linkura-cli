use crate::macros::{define_api_struct, post, use_common_crate};

use_common_crate!();
define_api_struct!(JewelShopApi);

impl<'a> JewelShopApi<'a> {
    // POST /v1/jewel_shop/get_birthday
    post!(
        get_birthday,
        "/jewel_shop/get_birthday",
        crate::model::JewelShopGetBirthdayResponse
    );

    // POST /v1/jewel_shop/get_list
    post!(
        get_list,
        "/jewel_shop/get_list",
        crate::model::JewelShopGetListResponse
    );

    // POST /v1/jewel_shop/get_membership_list
    post!(
        get_membership_list,
        "/jewel_shop/get_membership_list",
        crate::model::JewelShopGetMembershipListResponse
    );

    // POST /v1/jewel_shop/set_birthday
    post!(
        set_birthday,
        "/jewel_shop/set_birthday",
        crate::model::JewelShopSetBirthdayRequest,
        crate::model::JewelShopSetBirthdayResponse
    );

    // POST /v1/jewel_shop/set_membership_purchase
    post!(
        set_membership_purchase,
        "/jewel_shop/set_membership_purchase",
        crate::model::JewelShopSetPurchaseRequest,
        crate::model::JewelShopSetPurchaseResponse
    );

    // POST /v1/jewel_shop/set_purchase
    post!(
        set_purchase,
        "/jewel_shop/set_purchase",
        crate::model::JewelShopSetPurchaseRequest,
        crate::model::JewelShopSetPurchaseResponse
    );
}




