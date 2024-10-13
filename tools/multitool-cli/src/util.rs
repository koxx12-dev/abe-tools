use crate::generate_key_to_json_match;
use epic_balance::{proto, BalancingDataArchive, BalancingDataTypes};
use std::str::FromStr;

pub(crate) fn key_to_json(
    key: BalancingDataTypes,
    reader: &BalancingDataArchive,
) -> anyhow::Result<String> {
    Ok(generate_key_to_json_match!(reader, key,
        BattleBalancingData => proto::BattleBalancingData,
        ChronicleCaveFloorBalancingData => proto::ChronicleCaveFloorBalancingData,
        CustomMessageBalancingData => proto::CustomMessageBalancingData,
        HotspotBalancingData => proto::HotspotBalancingData,
        ExperienceScalingBalancingData => proto::ExperienceScalingBalancingData,
        CollectionGroupBalancingData => proto::CollectionGroupBalancingData,
        ExperienceLevelBalancingData => proto::ExperienceLevelBalancingData,
        InventoryBalancingData => proto::InventoryBalancingData,
        ShopBalancingData => proto::ShopBalancingData,
        BattleParticipantTableBalancingData => proto::BattleParticipantTableBalancingData,
        ClassSkinBalancingData => proto::ClassSkinBalancingData,
        ConditionalInventoryBalancingData => proto::ConditionalInventoryBalancingData,
        ChronicleCaveBattleBalancingData => proto::BattleBalancingData,
        ExperienceMasteryBalancingData => proto::ExperienceMasteryBalancingData,
        BirdBalancingData => proto::BirdBalancingData,
        CraftingRecipeBalancingData => proto::CraftingRecipeBalancingData,
        MiniCampaignBalancingData => proto::MiniCampaignBalancingData,
        LootTableBalancingData => proto::LootTableBalancingData,
        SocialEnvironmentBalancingData => proto::SocialEnvironmentBalancingData,
        BannerItemBalancingData => proto::BannerItemBalancingData,
        PremiumShopOfferBalancingData => proto::BasicShopOfferBalancingData,
        GachaShopOfferBalancingData => proto::BasicShopOfferBalancingData,
        PowerLevelBalancingData => proto::PowerLevelBalancingData,
        PigBalancingData => proto::PigBalancingData,
        EnchantingBalancingData => proto::EnchantingBalancingData,
        BuyableShopOfferBalancingData => proto::BasicShopOfferBalancingData,
        ClientConfigBalancingData => proto::ClientConfigBalancingData,
        ConsumableItemBalancingData => proto::ConsumableItemBalancingData,
        ResourceCostPerLevelBalancingData => proto::ResourceCostPerLevelBalancingData,
        SkillBalancingData => proto::SkillBalancingData,
        EventItemBalancingData => proto::EventItemBalancingData,
        GameConstantsBalancingData => proto::GameConstantsBalancingData,
        CraftingItemBalancingData => proto::CraftingItemBalancingData,
        DailyLoginGiftsBalancingData => proto::DailyLoginGiftsBalancingData,
        LoadingHintBalancingData => proto::LoadingHintBalancingData,
        SetFusionBalancingData => proto::SetFusionBalancingData,
        EquipmentBalancingData => proto::EquipmentBalancingData,
        ChronicleCaveHotspotBalancingData => proto::HotspotBalancingData,
        MasteryItemBalancingData => proto::MasteryItemBalancingData,
        SplashScreenBalancingData => proto::SplashScreenBalancingData,
        PigTypePowerLevelBalancingData => proto::PigTypePowerLevelBalancingData,
        PvPObjectivesBalancingData => proto::PvPObjectivesBalancingData,
        ThirdPartyIdBalancingData => proto::ThirdPartyIdBalancingData,
        BasicItemBalancingData => proto::BasicItemBalancingData,
        BattleHintBalancingData => proto::BattleHintBalancingData,
        BossBalancingData => proto::BossBalancingData,
        ScoreBalancingData => proto::ScoreBalancingData,
        SalesManagerBalancingData => proto::SalesManagerBalancingData,
        BannerBalancingData => proto::BannerBalancingData,
        ChronicleCaveBattleParticipantTableBalancingData => proto::BattleParticipantTableBalancingData,
        ClassItemBalancingData => proto::ClassItemBalancingData,
        EventBalancingData => proto::EventBalancingData,
        BonusEventBalancingData => proto::BonusEventBalancingData,
        PvPSeasonManagerBalancingData => proto::PvPSeasonManagerBalancingData,
        EventPlacementBalancingData => proto::EventPlacementBalancingData,
        EventManagerBalancingData => proto::EventManagerBalancingData,
    ))
}
pub(crate) fn get_key_from_name(container_name: &str) -> anyhow::Result<BalancingDataTypes> {
    Ok(match BalancingDataTypes::from_str(container_name) {
        Ok(key) => key,
        Err(_) => {
            let container_name = container_name
                .split('.')
                .last()
                .ok_or(anyhow!("Invalid container name"))?;
            
            match BalancingDataTypes::from_str(format!("ABH.Shared.BalancingData.{}", container_name).as_str()) {
                Ok(v) => v,
                Err(_) => BalancingDataTypes::from_str(format!("ABH.Shared.Events.BalancingData.{}", container_name).as_str())?
            }
        }
    })
}