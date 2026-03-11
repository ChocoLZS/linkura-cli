use crate::macros::{define_api_struct, post, use_common_crate};

use_common_crate!();
define_api_struct!(StickerExchangeApi);

impl<'a> StickerExchangeApi<'a> {
    // POST /v1/sticker_exchange/get_list
    post!(
        get_list,
        "/sticker_exchange/get_list",
        crate::model::StickerExchangeGetListResponse
    );

    // POST /v1/sticker_exchange/set_purchase
    post!(
        set_purchase,
        "/sticker_exchange/set_purchase",
        crate::model::StickerExchangeSetPurchaseRequest,
        crate::model::StickerExchangeSetPurchaseResponse
    );
}




