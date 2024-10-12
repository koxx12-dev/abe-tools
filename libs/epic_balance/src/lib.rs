use std::fmt::Display;
use std::str::FromStr;
use std::{
    fs::File,
    io::{Read, Write},
};

use anyhow::{anyhow, bail};
use prost::{
    bytes::{Buf, BufMut},
    DecodeError, Message,
};
use serde::Serialize;

use proto::SerializedBalancingDataContainer;

pub mod proto {
    include!(concat!(env!("OUT_DIR"), "/abepic.balancing.rs"));
    include!(concat!(env!("OUT_DIR"), "/abepic.balancing.serde.rs"));
}

pub struct BalancingDataArchive {
    container: SerializedBalancingDataContainer,
}

impl BalancingDataArchive {
    pub fn default() -> Self {
        Self {
            container: SerializedBalancingDataContainer::default(),
        }
    }

    pub fn new_gzipped<B>(buf: B) -> anyhow::Result<Self>
    where
        B: Buf + Read,
    {
        let mut decoder = flate2::read::GzDecoder::new(buf);
        let mut decompressed = Vec::new();
        decoder.read_to_end(&mut decompressed)?;

        Self::new(decompressed.as_slice())
    }

    pub fn new<B>(buf: B) -> anyhow::Result<Self>
    where
        B: Buf,
    {
        let container = SerializedBalancingDataContainer::decode(buf)?;
        Ok(Self { container })
    }

    pub fn get_balaning_data_keys(&self) -> Vec<String> {
        self.container
            .all_balancing_data
            .iter()
            .map(|balancing_data| balancing_data.0.clone())
            .collect()
    }

    pub fn get_data_key(&self, key: &str) -> Option<&Vec<u8>> {
        self.container
            .all_balancing_data
            .iter()
            .find(|balancing_data| balancing_data.0 == key)
            .map(|balancing_data| balancing_data.1)
    }

    pub fn get_data_key_decoded<T>(&self, key: &str) -> Result<T, DecodeError>
    where
        T: Message + Default,
    {
        let data = match self.get_data_key(key) {
            Some(data) => data,
            None => return Err(DecodeError::new("Key not found")),
        };

        T::decode(data.as_slice())
    }

    pub fn get_data_enum_decoded<T>(&self, key: BalancingDataTypes) -> Result<T, DecodeError>
    where
        T: Message + Default,
    {
        self.get_data_key_decoded::<T>(&key.to_string())
    }

    pub fn get_data_key_decoded_json<T>(&self, key: &str) -> anyhow::Result<String>
    where
        T: Message + Serialize + Default,
    {
        let data = self.get_data_key_decoded::<T>(key)?;
        Ok(serde_json::to_string_pretty(&data)?)
    }

    pub fn get_data_enum_decoded_json<T>(&self, key: BalancingDataTypes) -> anyhow::Result<String>
    where
        T: Message + Serialize + Default,
    {
        self.get_data_key_decoded_json::<T>(&key.to_string())
    }

    pub fn set_data_key_raw(&mut self, key: &str, data: Vec<u8>) {
        self.container
            .all_balancing_data
            .insert(key.to_string(), data);
    }

    pub fn set_data_key<T>(&mut self, key: &str, data: T) -> anyhow::Result<()>
    where
        T: Message,
    {
        let mut buf = Vec::new();
        data.encode(&mut buf)?;

        self.set_data_key_raw(key, buf);

        Ok(())
    }

    pub fn set_data_key_json<T>(&mut self, key: &str, json: &str) -> anyhow::Result<()>
    where
        T: Message + Default + serde::de::DeserializeOwned,
    {
        let data: T = serde_json::from_str(json)?;
        self.set_data_key(key, data)
    }

    pub fn set_data_enum<T>(&mut self, key: BalancingDataTypes, data: T) -> anyhow::Result<()>
    where
        T: Message,
    {
        self.set_data_key(key.to_string().as_str(), data)
    }

    pub fn set_data_enum_json<T>(
        &mut self,
        key: BalancingDataTypes,
        json: &str,
    ) -> anyhow::Result<()>
    where
        T: Message + Default + serde::de::DeserializeOwned,
    {
        self.set_data_key_json::<T>(key.to_string().as_str(), json)
    }

    pub fn write_gzipped<W>(&self, writer: &mut W) -> anyhow::Result<()>
    where
        W: BufMut + Write,
    {
        let mut encoder = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());

        let mut buf = Vec::new();
        self.container.encode(&mut buf)?;
        encoder.write_all(&buf)?;

        writer
            .write_all(&encoder.finish()?)
            .map_err(|err| anyhow!(err))
    }

    pub fn write<W>(&self, writer: &mut W) -> anyhow::Result<()>
    where
        W: BufMut + Write,
    {
        self.container.encode(writer).map_err(|err| anyhow!(err))
    }

    pub fn save(&self, mut file: File) -> anyhow::Result<()> {
        let mut vec = Vec::new();
        self.write(&mut vec)?;

        Ok(file.write_all(vec.as_slice())?)
    }

    pub fn save_gzipped(&self, mut file: File) -> anyhow::Result<()> {
        let mut vec = Vec::new();
        self.write_gzipped(&mut vec)?;

        Ok(file.write_all(vec.as_slice())?)
    }

    pub fn set_version(&mut self, version: &str) {
        self.container.version = Some(version.to_string());
    }

    pub fn get_version(&self) -> &str {
        self.container.version()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BalancingDataTypes {
    //NORMAL BALANCING
    BattleBalancingData,
    ChronicleCaveFloorBalancingData,
    CustomMessageBalancingData,
    HotspotBalancingData,
    ExperienceScalingBalancingData,
    CollectionGroupBalancingData,
    ExperienceLevelBalancingData,
    InventoryBalancingData,
    ShopBalancingData,
    BattleParticipantTableBalancingData,
    ClassSkinBalancingData,
    ConditionalInventoryBalancingData,
    ChronicleCaveBattleBalancingData,
    ExperienceMasteryBalancingData,
    BirdBalancingData,
    CraftingRecipeBalancingData,
    MiniCampaignBalancingData,
    LootTableBalancingData,
    SocialEnvironmentBalancingData,
    BannerItemBalancingData,
    PremiumShopOfferBalancingData,
    GachaShopOfferBalancingData,
    PowerLevelBalancingData,
    PigBalancingData,
    EnchantingBalancingData,
    BuyableShopOfferBalancingData,
    ClientConfigBalancingData,
    ConsumableItemBalancingData,
    ResourceCostPerLevelBalancingData,
    SkillBalancingData,
    EventItemBalancingData,
    GameConstantsBalancingData,
    CraftingItemBalancingData,
    DailyLoginGiftsBalancingData,
    LoadingHintBalancingData,
    SetFusionBalancingData,
    EquipmentBalancingData,
    ChronicleCaveHotspotBalancingData,
    MasteryItemBalancingData,
    SplashScreenBalancingData,
    PigTypePowerLevelBalancingData,
    PvPObjectivesBalancingData,
    ThirdPartyIdBalancingData,
    BasicItemBalancingData,
    BattleHintBalancingData,
    BossBalancingData,
    ScoreBalancingData,
    SalesManagerBalancingData,
    BannerBalancingData,
    ChronicleCaveBattleParticipantTableBalancingData,
    ClassItemBalancingData,
    //EVENT STUFF
    EventBalancingData,
    BonusEventBalancingData,
    PvPSeasonManagerBalancingData,
    EventPlacementBalancingData,
    EventManagerBalancingData,
}

impl FromStr for BalancingDataTypes {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ABH.Shared.BalancingData.BattleBalancingData" => Ok(BalancingDataTypes::BattleBalancingData),
            "ABH.Shared.BalancingData.ChronicleCaveFloorBalancingData" => Ok(BalancingDataTypes::ChronicleCaveFloorBalancingData),
            "ABH.Shared.BalancingData.CustomMessageBalancingData" => Ok(BalancingDataTypes::CustomMessageBalancingData),
            "ABH.Shared.BalancingData.HotspotBalancingData" => Ok(BalancingDataTypes::HotspotBalancingData),
            "ABH.Shared.BalancingData.ExperienceScalingBalancingData" => Ok(BalancingDataTypes::ExperienceScalingBalancingData),
            "ABH.Shared.BalancingData.CollectionGroupBalancingData" => Ok(BalancingDataTypes::CollectionGroupBalancingData),
            "ABH.Shared.BalancingData.ExperienceLevelBalancingData" => Ok(BalancingDataTypes::ExperienceLevelBalancingData),
            "ABH.Shared.BalancingData.InventoryBalancingData" => Ok(BalancingDataTypes::InventoryBalancingData),
            "ABH.Shared.BalancingData.ShopBalancingData" => Ok(BalancingDataTypes::ShopBalancingData),
            "ABH.Shared.BalancingData.BattleParticipantTableBalancingData" => Ok(BalancingDataTypes::BattleParticipantTableBalancingData),
            "ABH.Shared.BalancingData.ClassSkinBalancingData" => Ok(BalancingDataTypes::ClassSkinBalancingData),
            "ABH.Shared.BalancingData.ConditionalInventoryBalancingData" => Ok(BalancingDataTypes::ConditionalInventoryBalancingData),
            "ABH.Shared.BalancingData.ChronicleCaveBattleBalancingData" => Ok(BalancingDataTypes::ChronicleCaveBattleBalancingData),
            "ABH.Shared.BalancingData.ExperienceMasteryBalancingData" => Ok(BalancingDataTypes::ExperienceMasteryBalancingData),
            "ABH.Shared.BalancingData.BirdBalancingData" => Ok(BalancingDataTypes::BirdBalancingData),
            "ABH.Shared.BalancingData.CraftingRecipeBalancingData" => Ok(BalancingDataTypes::CraftingRecipeBalancingData),
            "ABH.Shared.BalancingData.MiniCampaignBalancingData" => Ok(BalancingDataTypes::MiniCampaignBalancingData),
            "ABH.Shared.BalancingData.LootTableBalancingData" => Ok(BalancingDataTypes::LootTableBalancingData),
            "ABH.Shared.BalancingData.SocialEnvironmentBalancingData" => Ok(BalancingDataTypes::SocialEnvironmentBalancingData),
            "ABH.Shared.BalancingData.BannerItemBalancingData" => Ok(BalancingDataTypes::BannerItemBalancingData),
            "ABH.Shared.BalancingData.PremiumShopOfferBalancingData" => Ok(BalancingDataTypes::PremiumShopOfferBalancingData),
            "ABH.Shared.BalancingData.GachaShopOfferBalancingData" => Ok(BalancingDataTypes::GachaShopOfferBalancingData),
            "ABH.Shared.BalancingData.PowerLevelBalancingData" => Ok(BalancingDataTypes::PowerLevelBalancingData),
            "ABH.Shared.BalancingData.PigBalancingData" => Ok(BalancingDataTypes::PigBalancingData),
            "ABH.Shared.BalancingData.EnchantingBalancingData" => Ok(BalancingDataTypes::EnchantingBalancingData),
            "ABH.Shared.BalancingData.BuyableShopOfferBalancingData" => Ok(BalancingDataTypes::BuyableShopOfferBalancingData),
            "ABH.Shared.BalancingData.ClientConfigBalancingData" => Ok(BalancingDataTypes::ClientConfigBalancingData),
            "ABH.Shared.BalancingData.ConsumableItemBalancingData" => Ok(BalancingDataTypes::ConsumableItemBalancingData),
            "ABH.Shared.BalancingData.ResourceCostPerLevelBalancingData" => Ok(BalancingDataTypes::ResourceCostPerLevelBalancingData),
            "ABH.Shared.BalancingData.SkillBalancingData" => Ok(BalancingDataTypes::SkillBalancingData),
            "ABH.Shared.BalancingData.EventItemBalancingData" => Ok(BalancingDataTypes::EventItemBalancingData),
            "ABH.Shared.BalancingData.GameConstantsBalancingData" => Ok(BalancingDataTypes::GameConstantsBalancingData),
            "ABH.Shared.BalancingData.CraftingItemBalancingData" => Ok(BalancingDataTypes::CraftingItemBalancingData),
            "ABH.Shared.BalancingData.DailyLoginGiftsBalancingData" => Ok(BalancingDataTypes::DailyLoginGiftsBalancingData),
            "ABH.Shared.BalancingData.LoadingHintBalancingData" => Ok(BalancingDataTypes::LoadingHintBalancingData),
            "ABH.Shared.BalancingData.SetFusionBalancingData" => Ok(BalancingDataTypes::SetFusionBalancingData),
            "ABH.Shared.BalancingData.EquipmentBalancingData" => Ok(BalancingDataTypes::EquipmentBalancingData),
            "ABH.Shared.BalancingData.ChronicleCaveHotspotBalancingData" => Ok(BalancingDataTypes::ChronicleCaveHotspotBalancingData),
            "ABH.Shared.BalancingData.MasteryItemBalancingData" => Ok(BalancingDataTypes::MasteryItemBalancingData),
            "ABH.Shared.BalancingData.SplashScreenBalancingData" => Ok(BalancingDataTypes::SplashScreenBalancingData),
            "ABH.Shared.BalancingData.PigTypePowerLevelBalancingData" => Ok(BalancingDataTypes::PigTypePowerLevelBalancingData),
            "ABH.Shared.BalancingData.PvPObjectivesBalancingData" => Ok(BalancingDataTypes::PvPObjectivesBalancingData),
            "ABH.Shared.BalancingData.ThirdPartyIdBalancingData" => Ok(BalancingDataTypes::ThirdPartyIdBalancingData),
            "ABH.Shared.BalancingData.BasicItemBalancingData" => Ok(BalancingDataTypes::BasicItemBalancingData),
            "ABH.Shared.BalancingData.BattleHintBalancingData" => Ok(BalancingDataTypes::BattleHintBalancingData),
            "ABH.Shared.BalancingData.BossBalancingData" => Ok(BalancingDataTypes::BossBalancingData),
            "ABH.Shared.BalancingData.ScoreBalancingData" => Ok(BalancingDataTypes::ScoreBalancingData),
            "ABH.Shared.BalancingData.SalesManagerBalancingData" => Ok(BalancingDataTypes::SalesManagerBalancingData),
            "ABH.Shared.BalancingData.BannerBalancingData" => Ok(BalancingDataTypes::BannerBalancingData),
            "ABH.Shared.BalancingData.ChronicleCaveBattleParticipantTableBalancingData" => Ok(BalancingDataTypes::ChronicleCaveBattleParticipantTableBalancingData),
            "ABH.Shared.BalancingData.ClassItemBalancingData" => Ok(BalancingDataTypes::ClassItemBalancingData),
            "ABH.Shared.Events.BalancingData.EventBalancingData" => Ok(BalancingDataTypes::EventBalancingData),
            "ABH.Shared.Events.BalancingData.BonusEventBalancingData" => Ok(BalancingDataTypes::BonusEventBalancingData),
            "ABH.Shared.Events.BalancingData.PvPSeasonManagerBalancingData" => Ok(BalancingDataTypes::PvPSeasonManagerBalancingData),
            "ABH.Shared.Events.BalancingData.EventPlacementBalancingData" => Ok(BalancingDataTypes::EventPlacementBalancingData),
            "ABH.Shared.Events.BalancingData.EventManagerBalancingData" => Ok(BalancingDataTypes::EventManagerBalancingData),
            _ => bail!("Unknown BalancingData type: {}", s),
        }
    }
}

impl Display for BalancingDataTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            BalancingDataTypes::BattleBalancingData => "ABH.Shared.BalancingData.BattleBalancingData",
            BalancingDataTypes::ChronicleCaveFloorBalancingData => "ABH.Shared.BalancingData.ChronicleCaveFloorBalancingData",
            BalancingDataTypes::CustomMessageBalancingData => "ABH.Shared.BalancingData.CustomMessageBalancingData",
            BalancingDataTypes::HotspotBalancingData => "ABH.Shared.BalancingData.HotspotBalancingData",
            BalancingDataTypes::ExperienceScalingBalancingData => "ABH.Shared.BalancingData.ExperienceScalingBalancingData",
            BalancingDataTypes::CollectionGroupBalancingData => "ABH.Shared.BalancingData.CollectionGroupBalancingData",
            BalancingDataTypes::ExperienceLevelBalancingData => "ABH.Shared.BalancingData.ExperienceLevelBalancingData",
            BalancingDataTypes::InventoryBalancingData => "ABH.Shared.BalancingData.InventoryBalancingData",
            BalancingDataTypes::ShopBalancingData => "ABH.Shared.BalancingData.ShopBalancingData",
            BalancingDataTypes::BattleParticipantTableBalancingData => "ABH.Shared.BalancingData.BattleParticipantTableBalancingData",
            BalancingDataTypes::ClassSkinBalancingData => "ABH.Shared.BalancingData.ClassSkinBalancingData",
            BalancingDataTypes::ConditionalInventoryBalancingData => "ABH.Shared.BalancingData.ConditionalInventoryBalancingData",
            BalancingDataTypes::ChronicleCaveBattleBalancingData => "ABH.Shared.BalancingData.ChronicleCaveBattleBalancingData",
            BalancingDataTypes::ExperienceMasteryBalancingData => "ABH.Shared.BalancingData.ExperienceMasteryBalancingData",
            BalancingDataTypes::BirdBalancingData => "ABH.Shared.BalancingData.BirdBalancingData",
            BalancingDataTypes::CraftingRecipeBalancingData => "ABH.Shared.BalancingData.CraftingRecipeBalancingData",
            BalancingDataTypes::MiniCampaignBalancingData => "ABH.Shared.BalancingData.MiniCampaignBalancingData",
            BalancingDataTypes::LootTableBalancingData => "ABH.Shared.BalancingData.LootTableBalancingData",
            BalancingDataTypes::SocialEnvironmentBalancingData => "ABH.Shared.BalancingData.SocialEnvironmentBalancingData",
            BalancingDataTypes::BannerItemBalancingData => "ABH.Shared.BalancingData.BannerItemBalancingData",
            BalancingDataTypes::PremiumShopOfferBalancingData => "ABH.Shared.BalancingData.PremiumShopOfferBalancingData",
            BalancingDataTypes::GachaShopOfferBalancingData => "ABH.Shared.BalancingData.GachaShopOfferBalancingData",
            BalancingDataTypes::PowerLevelBalancingData => "ABH.Shared.BalancingData.PowerLevelBalancingData",
            BalancingDataTypes::PigBalancingData => "ABH.Shared.BalancingData.PigBalancingData",
            BalancingDataTypes::EnchantingBalancingData => "ABH.Shared.BalancingData.EnchantingBalancingData",
            BalancingDataTypes::BuyableShopOfferBalancingData => "ABH.Shared.BalancingData.BuyableShopOfferBalancingData",
            BalancingDataTypes::ClientConfigBalancingData => "ABH.Shared.BalancingData.ClientConfigBalancingData",
            BalancingDataTypes::ConsumableItemBalancingData => "ABH.Shared.BalancingData.ConsumableItemBalancingData",
            BalancingDataTypes::ResourceCostPerLevelBalancingData => "ABH.Shared.BalancingData.ResourceCostPerLevelBalancingData",
            BalancingDataTypes::SkillBalancingData => "ABH.Shared.BalancingData.SkillBalancingData",
            BalancingDataTypes::EventItemBalancingData => "ABH.Shared.BalancingData.EventItemBalancingData",
            BalancingDataTypes::GameConstantsBalancingData => "ABH.Shared.BalancingData.GameConstantsBalancingData",
            BalancingDataTypes::CraftingItemBalancingData => "ABH.Shared.BalancingData.CraftingItemBalancingData",
            BalancingDataTypes::DailyLoginGiftsBalancingData => "ABH.Shared.BalancingData.DailyLoginGiftsBalancingData",
            BalancingDataTypes::LoadingHintBalancingData => "ABH.Shared.BalancingData.LoadingHintBalancingData",
            BalancingDataTypes::SetFusionBalancingData => "ABH.Shared.BalancingData.SetFusionBalancingData",
            BalancingDataTypes::EquipmentBalancingData => "ABH.Shared.BalancingData.EquipmentBalancingData",
            BalancingDataTypes::ChronicleCaveHotspotBalancingData => "ABH.Shared.BalancingData.ChronicleCaveHotspotBalancingData",
            BalancingDataTypes::MasteryItemBalancingData => "ABH.Shared.BalancingData.MasteryItemBalancingData",
            BalancingDataTypes::SplashScreenBalancingData => "ABH.Shared.BalancingData.SplashScreenBalancingData",
            BalancingDataTypes::PigTypePowerLevelBalancingData => "ABH.Shared.BalancingData.PigTypePowerLevelBalancingData",
            BalancingDataTypes::PvPObjectivesBalancingData => "ABH.Shared.BalancingData.PvPObjectivesBalancingData",
            BalancingDataTypes::ThirdPartyIdBalancingData => "ABH.Shared.BalancingData.ThirdPartyIdBalancingData",
            BalancingDataTypes::BasicItemBalancingData => "ABH.Shared.BalancingData.BasicItemBalancingData",
            BalancingDataTypes::BattleHintBalancingData => "ABH.Shared.BalancingData.BattleHintBalancingData",
            BalancingDataTypes::BossBalancingData => "ABH.Shared.BalancingData.BossBalancingData",
            BalancingDataTypes::ScoreBalancingData => "ABH.Shared.BalancingData.ScoreBalancingData",
            BalancingDataTypes::SalesManagerBalancingData => "ABH.Shared.BalancingData.SalesManagerBalancingData",
            BalancingDataTypes::BannerBalancingData => "ABH.Shared.BalancingData.BannerBalancingData",
            BalancingDataTypes::ChronicleCaveBattleParticipantTableBalancingData => "ABH.Shared.BalancingData.ChronicleCaveBattleParticipantTableBalancingData",
            BalancingDataTypes::ClassItemBalancingData => "ABH.Shared.BalancingData.ClassItemBalancingData",
            BalancingDataTypes::EventBalancingData => "ABH.Shared.Events.BalancingData.EventBalancingData",
            BalancingDataTypes::BonusEventBalancingData => "ABH.Shared.Events.BalancingData.BonusEventBalancingData",
            BalancingDataTypes::PvPSeasonManagerBalancingData => "ABH.Shared.Events.BalancingData.PvPSeasonManagerBalancingData",
            BalancingDataTypes::EventPlacementBalancingData => "ABH.Shared.Events.BalancingData.EventPlacementBalancingData",
            BalancingDataTypes::EventManagerBalancingData => "ABH.Shared.Events.BalancingData.EventManagerBalancingData"
        };
        write!(f, "{}", str)
    }
}
