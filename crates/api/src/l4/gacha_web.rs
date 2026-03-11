use crate::macros::{define_api_struct, post, use_common_crate};

use_common_crate!();
define_api_struct!(GachaWebApi);

impl<'a> GachaWebApi<'a> {
    // POST /v1/gacha/get_exchange_card_list
    post!(
        get_exchange_card_list,
        "/gacha/get_exchange_card_list",
        crate::model::GachaGetExchangeCardListRequest,
        crate::model::GachaGetExchangeCardListResponse
    );

    // POST /v1/gacha/get_lottery_chance
    post!(
        get_lottery_chance,
        "/gacha/get_lottery_chance",
        crate::model::GachaGetLotteryChanceRequest,
        crate::model::GachaGetLotteryChanceResponse
    );
}




