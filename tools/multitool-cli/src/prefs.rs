use clap::{Args, Subcommand};
use epic_prefs::PlayerPrefsData;
use std::path::PathBuf;

#[derive(Args, Clone)]
#[command(version, about, long_about = Some("Encode or decode a xml player prefs file"), aliases = ["p", "pref", "preferences"]
)]
pub(super) struct PrefsArgs {
    #[arg(help = "Location of the serialized player prefs xml file")]
    pub player_prefs_path: PathBuf,
    #[arg(help = "Path to the player json file to be encoded/decoded")]
    pub player_json_file: PathBuf,

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
pub(super) struct PrefsDecodeArgs {}

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
    let json_file = std::fs::read_to_string(prefs_args.player_json_file)?;
    let xml_file = std::fs::read_to_string(prefs_args.player_prefs_path)?;

    let prefs = PlayerPrefsData::from_json(json_file.as_str())?;

    let xml_file = prefs.to_xml(xml_file.as_str(), None)?;

    std::fs::write(args.output_prefs_path, xml_file.as_bytes()).map_err(|err| anyhow!(err))
}

pub(super) fn decode_prefs(prefs_args: PrefsArgs, _: PrefsDecodeArgs) -> anyhow::Result<()> {
    let xml_file = std::fs::read_to_string(prefs_args.player_prefs_path)?;

    let prefs = PlayerPrefsData::from_xml(xml_file.as_str())?;
    let json = prefs.to_json_pretty()?;

    std::fs::write(prefs_args.player_json_file, json.as_bytes()).map_err(|err| anyhow!(err))
}
