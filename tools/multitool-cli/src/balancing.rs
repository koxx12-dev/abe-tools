use crate::{generate_container_encode_match, DataFormat};
use crate::util::{get_key_from_name, key_to_string};
use clap::{Args, Subcommand};
use epic_balance::{proto, BalancingDataArchive, BalancingDataTypes};
use std::fs::File;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Args, Clone)]
#[command(version, about, long_about = Some("Encode or decode a serialized balancing data container"), aliases = ["b", "bal", "balance", "balancing-data"]
)]
pub(super) struct BalancingArgs {
    #[arg(help = "Location of the serialized balancing data container")]
    pub live_data_path: PathBuf,
    #[arg(help = "Name of the balancing data container to be encoded/decoded")]
    pub container_name: Option<String>,

    #[command(subcommand)]
    pub balancing_action: BalancingAction,
}

#[derive(Subcommand, Clone)]
pub(super) enum BalancingAction {
    Decode(BalancingDecodeArgs),
    Encode(BalancingEncodeArgs),
}

#[derive(Args, Clone)]
#[command(version, about, long_about = Some("Decode a serialized balancing data container"), aliases = ["d", "unpack", "export"]
)]
pub(super) struct BalancingDecodeArgs {
    #[arg(
        long,
        short,
        help = "Location to save the decoded container file, does not apply if --all is used",
        value_name = "FILE"
    )]
    pub output_file_path: Option<PathBuf>,
    #[arg(long = "all", short = 'A', help = "Export all keys in the container")]
    pub export_all: bool,

    #[arg(help = "Data format to output the container file in", long, short = 'O', default_value_t=DataFormat::Json)]
    pub output_as: DataFormat,
}
#[derive(Args, Clone)]
#[command(version, about, long_about = Some("Encode a serialized balancing data container"), aliases = ["e", "pack", "import", "reimport"]
)]
pub(super) struct BalancingEncodeArgs {
    #[arg(help = "Location of the container file to encode into the container")]
    pub container_file_path: PathBuf,
    #[arg(help = "Location to save the encoded container")]
    pub output_file_path: PathBuf,
}

pub(super) fn decode_container(
    balancing_args: BalancingArgs,
    args: BalancingDecodeArgs,
) -> anyhow::Result<()> {
    let data = std::fs::read(balancing_args.live_data_path)?;

    let reader = match BalancingDataArchive::new_gzipped(data.as_slice()) {
        Ok(reader) => reader,
        Err(_) => BalancingDataArchive::new(data.as_slice())?,
    };

    if args.export_all {
        let keys = reader.get_balaning_data_keys();

        for key in keys {
            let enum_key = BalancingDataTypes::from_str(&key)?;
            let data = key_to_string(enum_key, &reader, args.output_as)?;

            std::fs::write(format!("{}.{}", key.to_string(), args.output_as.to_string()), data)?;
        }
    } else {
        let key = get_key_from_name(
            &balancing_args
                .container_name
                .ok_or(anyhow!("No container name provided"))?,
        )?;

        let data = key_to_string(key, &reader, args.output_as)?;

        if let Some(output_file_path) = args.output_file_path {
            if !output_file_path.exists() {
                File::create(&output_file_path)?;
            }

            std::fs::write(output_file_path, data)?;
        } else {
            std::fs::write(format!("{}.{}", key.to_string(), args.output_as.to_string()), data)?;
        }
    }

    Ok(())
}
pub(super) fn encode_container(
    balancing_args: BalancingArgs,
    args: BalancingEncodeArgs,
) -> anyhow::Result<()> {
    let key = get_key_from_name(
        &balancing_args
            .container_name
            .ok_or(anyhow!("No container name provided"))?,
    )?;

    let data = std::fs::read(balancing_args.live_data_path)?;
    let mut archive = match BalancingDataArchive::new(data.as_slice()) {
        Ok(reader) => reader,
        Err(_) => BalancingDataArchive::new_gzipped(data.as_slice())?,
    };

    let data = std::fs::read_to_string(args.container_file_path)?;
    let data_format = if data.starts_with("{") { DataFormat::Json } else { DataFormat::Ron };

    generate_container_encode_match!(archive, key, &data, data_format,
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
    );

    let file = File::create(args.output_file_path)?;
    archive.save_gzipped(file)?;

    Ok(())
}
