use clap::{arg, Args, Subcommand};
use epic_prefs::PlayerPrefsData;
use std::path::PathBuf;
use crate::DataFormat;

#[derive(Args, Clone)]
#[command(version, about, long_about = Some("Encode or decode sdkv2 player data"), aliases = ["s", "sdk", "sdkv2", "sdkv2prefs"]
)]
pub(super) struct Sdkv2Args {
    #[command(subcommand)]
    pub sdkv2_action: Sdkv2Action,
}

#[derive(Subcommand, Clone)]
pub(super) enum Sdkv2Action {
    Decode(Sdkv2DecodeArgs),
    Encode(Sdkv2EncodeArgs),
}

#[derive(Args, Clone)]
#[command(version, about, long_about = Some("Decode sdkv2 player data"), aliases = ["d", "unpack", "export"]
)]
pub(super) struct Sdkv2DecodeArgs {
    #[arg(help = "Sdkv2 encoded player data file/contents")]
    pub player_data: String,
    
    #[arg(help = "Data format to output the player prefs in", long, short = 'O', default_value_t=DataFormat::Json)]
    pub output_as: DataFormat,
    
    #[arg(help = "Location to save the decoded player data file")]
    pub output_data_path: PathBuf,
}

#[derive(Args, Clone)]
#[command(version, about, long_about = Some("Encode sdkv2 player data"), aliases = ["e", "pack", "import", "reimport"]
)]
pub(super) struct Sdkv2EncodeArgs {
    #[arg(help = "Path to the player data file to be encoded")]
    pub player_data_file: PathBuf,
    
    #[arg(help = "Location to save the encoded player data file")]
    pub output_data_path: Option<PathBuf>,
}

pub(super) fn encode_sdkv2(_sdkv2_args: Sdkv2Args, args: Sdkv2EncodeArgs) -> anyhow::Result<()> {
    let player_data = std::fs::read_to_string(args.player_data_file)?;
    
    let data_format = if player_data.starts_with("{") { DataFormat::Json } else { DataFormat::Ron };
    
    let decoded_player_data = match data_format {
        DataFormat::Ron => PlayerPrefsData::from_ron(player_data.as_str())?,
        DataFormat::Json => PlayerPrefsData::from_json(player_data.as_str())?
    };
    
    let encoded_player_data = decoded_player_data.to_sdkv2()?;
    
    std::fs::write(args.output_data_path.unwrap(), encoded_player_data.as_bytes()).map_err(anyhow::Error::new)
}

pub(super) fn decode_sdkv2(_sdkv2_args: Sdkv2Args, args: Sdkv2DecodeArgs) -> anyhow::Result<()> {
    let is_file = std::fs::metadata(&args.player_data).is_ok();
    let player_data = if is_file {
        std::fs::read_to_string(args.player_data)?
    } else {
        args.player_data
    };
    
    let decoded_player_data = PlayerPrefsData::from_sdkv2(player_data.as_str())?;
    
    let output_data = match args.output_as {
        DataFormat::Ron => decoded_player_data.to_ron_pretty()?,
        DataFormat::Json => decoded_player_data.to_json_pretty()?
    };
    
    std::fs::write(args.output_data_path, output_data.as_bytes()).map_err(anyhow::Error::new)
}