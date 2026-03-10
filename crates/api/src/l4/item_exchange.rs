use crate::macros::{define_api_struct, define_post_method, use_common_crate};

use_common_crate!();
define_api_struct!(ItemExchangeApi);

impl<'a> ItemExchangeApi<'a> {
    // POST /v1/item_exchange/get_limit_break_material_convert_list
    define_post_method!(
        get_limit_break_material_convert_list,
        "/item_exchange/get_limit_break_material_convert_list",
        crate::model::ItemExchangeGetLimitBreakMaterialConvertListRequest
    );

    // POST /v1/item_exchange/get_list
    define_post_method!(
        get_list,
        "/item_exchange/get_list",
        crate::model::ItemExchangeGetListRequest
    );

    // POST /v1/item_exchange/get_list_new
    define_post_method!(
        get_list_new,
        "/item_exchange/get_list_new",
        crate::model::ItemExchangeGetListNewRequest
    );

    // POST /v1/item_exchange/set_limit_break_material_convert
    define_post_method!(
        set_limit_break_material_convert,
        "/item_exchange/set_limit_break_material_convert",
        crate::model::ItemExchangeSetLimitBreakMaterialConvertRequest
    );

    // POST /v1/item_exchange/set_purchase
    define_post_method!(
        set_purchase,
        "/item_exchange/set_purchase",
        crate::model::ItemExchangeSetPurchaseRequest
    );
}
