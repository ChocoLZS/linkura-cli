use crate::macros::{define_api_struct, define_post_method, use_common_crate};

use_common_crate!();
define_api_struct!(StickerExchangeApi);

impl<'a> StickerExchangeApi<'a> {
    // POST /v1/sticker_exchange/get_list
    define_post_method!(
        get_list,
        "/sticker_exchange/get_list",
        crate::model::StickerExchangeGetListRequest
    );

    // POST /v1/sticker_exchange/set_purchase
    define_post_method!(
        set_purchase,
        "/sticker_exchange/set_purchase",
        crate::model::StickerExchangeSetPurchaseRequest
    );
}
