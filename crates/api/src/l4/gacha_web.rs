use crate::macros::{define_api_struct, define_post_method, use_common_crate};

use_common_crate!();
define_api_struct!(GachaWebApi);

impl<'a> GachaWebApi<'a> {
    // POST /v1/gacha/get_exchange_card_list
    define_post_method!(
        get_exchange_card_list,
        "/gacha/get_exchange_card_list",
        crate::model::GachaGetExchangeCardListRequest
    );

    // POST /v1/gacha/get_lottery_chance
    define_post_method!(
        get_lottery_chance,
        "/gacha/get_lottery_chance",
        crate::model::GachaGetLotteryChanceRequest
    );
}
