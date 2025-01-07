#[macro_use]
extern crate anyhow;

mod balancing;
mod locale;
mod macros;
mod prefs;
mod util;

use std::fmt::Display;
use crate::balancing::{decode_container, encode_container, BalancingAction, BalancingArgs};
use crate::locale::{decode_locale, encode_locale, LocaleAction, LocaleArgs};
use crate::prefs::{decode_prefs, encode_prefs, PrefsAction, PrefsArgs};
use clap::{Parser, ValueEnum};

#[cfg(feature = "dump")]
use {
    crate::util::key_to_string,
    epic_balance::{proto, BalancingDataArchive, BalancingDataTypes},
    prost::DecodeError,
    std::cmp::Ordering,
    std::collections::HashMap,
    std::str::FromStr,
};


#[derive(Parser)]
#[command(
    version,
    long_about = "Cli based multi-tool for a 2014 game developed by Rovio called Angry Birds Epic"
)]
enum Cli {
    Balancing(BalancingArgs),
    Prefs(PrefsArgs),
    Locale(LocaleArgs),
}

#[derive(ValueEnum, Copy, Clone)]
pub(crate) enum DataFormat {
    Ron,
    Json,
}

fn main() {
    if cfg!(feature = "dump") {
        dump_balancing()
    }

    let result = match Cli::parse() {
        Cli::Balancing(balancing_args) => match balancing_args.clone().balancing_action {
            BalancingAction::Decode(args) => decode_container(balancing_args, args),
            BalancingAction::Encode(args) => encode_container(balancing_args, args),
        },
        Cli::Prefs(prefs_args) => match prefs_args.clone().prefs_action {
            PrefsAction::Decode(args) => decode_prefs(prefs_args, args),
            PrefsAction::Encode(args) => encode_prefs(prefs_args, args),
        },
        Cli::Locale(locale_args) => match locale_args.clone().locale_action {
            LocaleAction::Decode(args) => decode_locale(locale_args, args),
            LocaleAction::Encode(args) => encode_locale(locale_args, args),
        },
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

//just dumps everything from a balancing data container and does some debug stuff, excluded in normal builds
#[cfg(not(feature = "dump"))]
fn dump_balancing() {
    eprintln!("How? How tf did you even run this function?")
}

#[cfg(feature = "dump")]
fn dump_balancing() {
    const DEBUG_FILE: &str = "live_EnglishLocaBalancingData.bytes";

    let data = std::fs::read(DEBUG_FILE).unwrap();

    let reader = BalancingDataArchive::new_gzipped(data.as_slice()).unwrap();

    let mut old_size = HashMap::new();
    let mut new_size = HashMap::new();

    //get every key and try to decode it
    for key in reader.get_balaning_data_keys() {
        //try decoding every key and if any one of them fails, print the error
        let result: Result<(), DecodeError> = match BalancingDataTypes::from_str(&key) {
            Ok(data_type) => {
                //return a serializable struct based on the key
                old_size.insert(
                    key.clone(),
                    reader.get_data_key(key.as_str()).unwrap().len(),
                );
                let serilize: String = key_to_string(data_type, &reader, DataFormat::Ron).unwrap();
                std::fs::write(format!("{}.Ron", key), serilize).unwrap();
                std::fs::write(
                    format!("{}.bin", key),
                    reader.get_data_key(key.as_str()).unwrap(),
                )
                .unwrap();
                Ok(())
            }
            Err(e) => {
                println!("Error decoding {}: {}", key, e);
                Err(DecodeError::new("Error decoding"))
            }
        };

        if result.is_err() {
            panic!("{:?}", result)
        }
    }

    //reconstruct the data and then gzip under a new nameb nm

    let mut new_data = BalancingDataArchive::default();

    for key in reader.get_balaning_data_keys() {
        let data_type = BalancingDataTypes::from_str(&key).unwrap();
        generate_reencode_container_data_unsafe!(reader, new_data, data_type,
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
    }

    for key in new_data.get_balaning_data_keys() {
        new_size.insert(
            key.clone(),
            new_data.get_data_key(key.as_str()).unwrap().len(),
        );

        let result: Result<(), DecodeError> = match BalancingDataTypes::from_str(&key) {
            Ok(data_type) => {
                let serilize: String = key_to_string(data_type, &reader, DataFormat::Ron).unwrap();
                std::fs::write(format!("{}_new.ron", key), serilize).unwrap();
                std::fs::write(
                    format!("{}_new.bin", key),
                    new_data.get_data_key(key.as_str()).unwrap(),
                )
                .unwrap();
                Ok(())
            }
            Err(e) => {
                println!("Error decoding {}: {}", key, e);
                Err(DecodeError::new("Error decoding"))
            }
        };

        if result.is_err() {
            panic!("{:?}", result)
        }
    }

    new_data.set_version(reader.get_version());

    println!("Version {}", new_data.get_version());

    //iterate over the old size and new size and print the difference
    for (key, old) in old_size.iter() {
        let new = new_size.get(key).unwrap();

        if old != new {
            let (val, status) = match new.cmp(old) {
                Ordering::Equal => (0, String::from("no change")),
                Ordering::Greater => (new - old, String::from("gained")),
                Ordering::Less => (old - new, String::from("lost")),
            };

            println!(
                "{}: Size doesn't match. ({} -> {} = {} {})",
                key, old, new, val, status
            );
        }
    }
    let mut buf = Vec::new();
    new_data.write_gzipped(&mut buf).unwrap();

    std::fs::write(DEBUG_FILE, buf).unwrap();

    println!("Done");
}

impl Display for DataFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            DataFormat::Ron => "ron",
            DataFormat::Json => "json"
        };
        write!(f, "{}", str)
    }
}
