use clap::{arg, Args, Subcommand};
use epic_prefs::PlayerPrefsData;
use std::path::PathBuf;
use crate::DataFormat;

#[derive(Args, Clone)]
#[command(version, about, long_about = Some("Encode or decode a xml player prefs file"), aliases = ["p", "pref", "preferences"]
)]
pub(super) struct PrefsArgs {
    #[arg(help = "Location of the serialized player prefs xml file")]
    pub player_prefs_path: PathBuf,
    #[arg(help = "Path to the player data file to be encoded/decoded")]
    pub player_data_file: PathBuf,

    #[command(subcommand)]
    pub prefs_action: PrefsAction,
}

#[derive(Subcommand, Clone)]
pub(super) enum PrefsAction {
    Decode(PrefsDecodeArgs),
    Encode(PrefsEncodeArgs),
}

#[derive(Args, Clone)]
#[command(version, about, long_about = Some("Decode a xml player prefs file"), aliases = ["d", "unpack", "export"]
)]
pub(super) struct PrefsDecodeArgs {
    #[arg(help = "Data format to output the player prefs in", long, short = 'O', default_value_t=DataFormat::Ron)]
    pub output_as: DataFormat,
}

#[derive(Args, Clone)]
#[command(version, about, long_about = Some("Encode a xml player prefs file"), aliases = ["e", "pack", "import", "reimport"]
)]
pub(super) struct PrefsEncodeArgs {
    #[arg(
        help = "Location to save the encoded player prefs file",
        value_name = "FILE"
    )]
    pub output_prefs_path: PathBuf,
}

pub(super) fn encode_prefs(prefs_args: PrefsArgs, args: PrefsEncodeArgs) -> anyhow::Result<()> {
    let json_file = std::fs::read_to_string(prefs_args.player_data_file)?;
    let xml_file = std::fs::read_to_string(prefs_args.player_prefs_path)?;
    
    let data_format = if json_file.starts_with("{") { DataFormat::Json } else { DataFormat::Ron };
    
    let prefs = match data_format {
        DataFormat::Ron => PlayerPrefsData::from_ron(json_file.as_str())?,
        DataFormat::Json => PlayerPrefsData::from_json(json_file.as_str())?
    };

    let xml_file = prefs.to_prefs_xml(xml_file.as_str(), None)?;

    std::fs::write(args.output_prefs_path, xml_file.as_bytes()).map_err(anyhow::Error::new)
}

pub(super) fn decode_prefs(prefs_args: PrefsArgs, args: PrefsDecodeArgs) -> anyhow::Result<()> {
    let xml_file = std::fs::read_to_string(prefs_args.player_prefs_path)?;

    let prefs = PlayerPrefsData::from_prefs_xml(xml_file.as_str())?;

    let data = match args.output_as {
        DataFormat::Ron => prefs.to_ron_pretty()?,
        DataFormat::Json => prefs.to_json_pretty()?
    };

    std::fs::write(prefs_args.player_data_file, data.as_bytes()).map_err(anyhow::Error::new)
}