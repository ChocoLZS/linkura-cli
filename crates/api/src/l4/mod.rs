use crate::macros::{define_api_struct, use_common_crate};

pub mod account;
pub mod activity_record;
pub mod announce;
pub mod archive;
pub mod box_event;
pub mod circle;
pub mod collection;
pub mod common;
pub mod feslive;
pub mod friend;
pub mod gacha;
pub mod gacha_web;
pub mod gift_shop;
pub mod gp_prize_exchange;
pub mod home;
pub mod in_quest_live;
pub mod item_exchange;
pub mod item_store;
pub mod jewel_shop;
pub mod mission;
pub mod out_quest_live;
pub mod out_quest_live_daily;
pub mod out_quest_live_dream;
pub mod out_quest_live_grade;
pub mod out_quest_live_grade_challenge;
pub mod out_quest_live_grand_prix;
pub mod out_quest_live_music_learning;
pub mod out_quest_live_raid_event;
pub mod petal_exchange;
pub mod present_box;
pub mod profile;
pub mod register;
pub mod rhythm_game;
pub mod rhythm_game_deck;
pub mod rhythm_game_grand_prix;
pub mod rhythm_game_live;
pub mod select_ticket_exchange;
pub mod serial_code;
pub mod shop;
pub mod sisca_store;
pub mod sticker_exchange;
pub mod tutorial;
pub mod user;
pub mod webview;
pub mod webview_live;
pub mod withlive;

use_common_crate!();
define_api_struct!(LinkuraApi);

impl<'a> LinkuraApi<'a> {
    pub fn account(&self) -> account::AccountApi {
        account::AccountApi { api: self }
    }

    pub fn activity_record(&self) -> activity_record::ActivityRecordApi {
        activity_record::ActivityRecordApi { api: self }
    }

    pub fn announce(&self) -> announce::AnnounceApi {
        announce::AnnounceApi { api: self }
    }

    pub fn archive(&self) -> archive::ArchiveApi {
        archive::ArchiveApi { api: self }
    }

    pub fn box_event(&self) -> box_event::BoxEventApi {
        box_event::BoxEventApi { api: self }
    }

    pub fn circle(&self) -> circle::CircleApi {
        circle::CircleApi { api: self }
    }

    pub fn collection(&self) -> collection::CollectionApi {
        collection::CollectionApi { api: self }
    }

    pub fn common(&self) -> common::CommonApi {
        common::CommonApi { api: self }
    }

    pub fn feslive(&self) -> feslive::FesliveApi {
        feslive::FesliveApi { api: self }
    }

    pub fn friend(&self) -> friend::FriendApi {
        friend::FriendApi { api: self }
    }

    pub fn gacha(&self) -> gacha::GachaApi {
        gacha::GachaApi { api: self }
    }

    pub fn gacha_web(&self) -> gacha_web::GachaWebApi {
        gacha_web::GachaWebApi { api: self }
    }

    pub fn gift_shop(&self) -> gift_shop::GiftShopApi {
        gift_shop::GiftShopApi { api: self }
    }

    pub fn gp_prize_exchange(&self) -> gp_prize_exchange::GpPrizeExchangeApi {
        gp_prize_exchange::GpPrizeExchangeApi { api: self }
    }

    pub fn home(&self) -> home::HomeApi {
        home::HomeApi { api: self }
    }

    pub fn in_quest_live(&self) -> in_quest_live::InQuestLiveApi {
        in_quest_live::InQuestLiveApi { api: self }
    }

    pub fn item_exchange(&self) -> item_exchange::ItemExchangeApi {
        item_exchange::ItemExchangeApi { api: self }
    }

    pub fn item_store(&self) -> item_store::ItemStoreApi {
        item_store::ItemStoreApi { api: self }
    }

    pub fn jewel_shop(&self) -> jewel_shop::JewelShopApi {
        jewel_shop::JewelShopApi { api: self }
    }

    pub fn mission(&self) -> mission::MissionApi {
        mission::MissionApi { api: self }
    }

    pub fn out_quest_live(&self) -> out_quest_live::OutQuestLiveApi {
        out_quest_live::OutQuestLiveApi { api: self }
    }

    pub fn out_quest_live_daily(&self) -> out_quest_live_daily::OutQuestLiveDailyApi {
        out_quest_live_daily::OutQuestLiveDailyApi { api: self }
    }

    pub fn out_quest_live_dream(&self) -> out_quest_live_dream::OutQuestLiveDreamApi {
        out_quest_live_dream::OutQuestLiveDreamApi { api: self }
    }

    pub fn out_quest_live_grade(&self) -> out_quest_live_grade::OutQuestLiveGradeApi {
        out_quest_live_grade::OutQuestLiveGradeApi { api: self }
    }

    pub fn out_quest_live_grade_challenge(
        &self,
    ) -> out_quest_live_grade_challenge::OutQuestLiveGradeChallengeApi {
        out_quest_live_grade_challenge::OutQuestLiveGradeChallengeApi { api: self }
    }

    pub fn out_quest_live_grand_prix(&self) -> out_quest_live_grand_prix::OutQuestLiveGrandPrixApi {
        out_quest_live_grand_prix::OutQuestLiveGrandPrixApi { api: self }
    }

    pub fn out_quest_live_music_learning(
        &self,
    ) -> out_quest_live_music_learning::OutQuestLiveMusicLearningApi {
        out_quest_live_music_learning::OutQuestLiveMusicLearningApi { api: self }
    }

    pub fn out_quest_live_raid_event(&self) -> out_quest_live_raid_event::OutQuestLiveRaidEventApi {
        out_quest_live_raid_event::OutQuestLiveRaidEventApi { api: self }
    }

    pub fn petal_exchange(&self) -> petal_exchange::PetalExchangeApi {
        petal_exchange::PetalExchangeApi { api: self }
    }

    pub fn present_box(&self) -> present_box::PresentBoxApi {
        present_box::PresentBoxApi { api: self }
    }

    pub fn profile(&self) -> profile::ProfileApi {
        profile::ProfileApi { api: self }
    }

    pub fn register(&self) -> register::RegisterApi {
        register::RegisterApi { api: self }
    }

    pub fn rhythm_game(&self) -> rhythm_game::RhythmGameApi {
        rhythm_game::RhythmGameApi { api: self }
    }

    pub fn rhythm_game_deck(&self) -> rhythm_game_deck::RhythmGameDeckApi {
        rhythm_game_deck::RhythmGameDeckApi { api: self }
    }

    pub fn rhythm_game_grand_prix(&self) -> rhythm_game_grand_prix::RhythmGameGrandPrixApi {
        rhythm_game_grand_prix::RhythmGameGrandPrixApi { api: self }
    }

    pub fn rhythm_game_live(&self) -> rhythm_game_live::RhythmGameLiveApi {
        rhythm_game_live::RhythmGameLiveApi { api: self }
    }

    pub fn select_ticket_exchange(&self) -> select_ticket_exchange::SelectTicketExchangeApi {
        select_ticket_exchange::SelectTicketExchangeApi { api: self }
    }

    pub fn serial_code(&self) -> serial_code::SerialCodeApi {
        serial_code::SerialCodeApi { api: self }
    }

    pub fn shop(&self) -> shop::ShopApi {
        shop::ShopApi { api: self }
    }

    pub fn sisca_store(&self) -> sisca_store::SiscaStoreApi {
        sisca_store::SiscaStoreApi { api: self }
    }

    pub fn sticker_exchange(&self) -> sticker_exchange::StickerExchangeApi {
        sticker_exchange::StickerExchangeApi { api: self }
    }

    pub fn tutorial(&self) -> tutorial::TutorialApi {
        tutorial::TutorialApi { api: self }
    }

    pub fn user(&self) -> user::UserApi {
        user::UserApi { api: self }
    }

    pub fn webview(&self) -> webview::WebviewApi {
        webview::WebviewApi { api: self }
    }

    pub fn webview_live(&self) -> webview_live::WebviewLiveApi {
        webview_live::WebviewLiveApi { api: self }
    }

    pub fn withlive(&self) -> withlive::WithliveApi {
        withlive::WithliveApi { api: self }
    }

    pub fn with_live(&self) -> withlive::WithliveApi {
        self.withlive()
    }

    pub fn fes_live(&self) -> feslive::FesliveApi {
        self.feslive()
    }
}
