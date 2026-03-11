use crate::macros::{define_api_struct, post, use_common_crate};

use_common_crate!();
define_api_struct!(ItemExchangeApi);

impl<'a> ItemExchangeApi<'a> {
    // POST /v1/item_exchange/get_limit_break_material_convert_list
    post!(
        get_limit_break_material_convert_list,
        "/item_exchange/get_limit_break_material_convert_list",
        crate::model::ItemExchangeGetLimitBreakMaterialConvertListResponse
    );

    // POST /v1/item_exchange/get_list
    post!(
        get_list,
        "/item_exchange/get_list",
        crate::model::ItemExchangeGetListResponse
    );

    // POST /v1/item_exchange/get_list_new
    post!(
        get_list_new,
        "/item_exchange/get_list_new",
        crate::model::ItemExchangeGetListRequest,
        crate::model::ItemExchangeGetListResponse
    );

    // POST /v1/item_exchange/set_limit_break_material_convert
    post!(
        set_limit_break_material_convert,
        "/item_exchange/set_limit_break_material_convert",
        crate::model::ItemExchangeSetLimitBreakMaterialConvertResponse
    );

    // POST /v1/item_exchange/set_purchase
    post!(
        set_purchase,
        "/item_exchange/set_purchase",
        crate::model::ItemExchangeSetPurchaseRequest,
        crate::model::ItemExchangeSetPurchaseResponse
    );
}




