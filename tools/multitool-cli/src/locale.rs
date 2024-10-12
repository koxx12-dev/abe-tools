use clap::{Args, Subcommand};
use epic_locale::LocaleDataContainer;
use std::path::PathBuf;

#[derive(Args, Clone)]
#[command(version, about, long_about = Some("Encode or decode a language locale file"), aliases = ["l", "loca", "localization"]
)]
pub(super) struct LocaleArgs {
    #[command(subcommand)]
    pub locale_action: LocaleAction,
}

#[derive(Subcommand, Clone)]
pub(super) enum LocaleAction {
    Decode(LocaleDecodeArgs),
    Encode(LocaleEncodeArgs),
}

#[derive(Args, Clone)]
#[command(version, about, long_about = Some("Decode a language locale file"), aliases = ["d", "unpack", "export"]
)]
pub(super) struct LocaleDecodeArgs {
    #[arg(help = "Location of the serialized language locale file")]
    pub language_locale_path: PathBuf,

    #[arg(help = "Location to save the decoded language locale file")]
    pub output_locale_path: PathBuf,
}

#[derive(Args, Clone)]
#[command(version, about, long_about = Some("Encode a language locale file"), aliases = ["e", "pack", "import", "reimport"]
)]
pub(super) struct LocaleEncodeArgs {
    #[arg(help = "Path to the locale json file to be encoded")]
    pub locale_json_file: PathBuf,

    #[arg(help = "Location to save the encoded language locale file")]
    pub output_locale_path: PathBuf,
}

pub(super) fn decode_locale(_: LocaleArgs, args: LocaleDecodeArgs) -> anyhow::Result<()> {
    let locale_file = std::fs::read(args.language_locale_path)?;

    let locale_container = match LocaleDataContainer::new_gzipped(locale_file.as_slice()) {
        Ok(reader) => reader,
        Err(_) => LocaleDataContainer::new(locale_file.as_slice())?,
    };

    std::fs::write(args.output_locale_path, locale_container.to_json_pretty()?)
        .map_err(|err| anyhow!(err))
}

pub(super) fn encode_locale(_: LocaleArgs, args: LocaleEncodeArgs) -> anyhow::Result<()> {
    let json_file = std::fs::read_to_string(args.locale_json_file)?;

    let locale_container = LocaleDataContainer::from_json(json_file.as_str())?;

    let mut buf = Vec::new();
    locale_container.write_gzipped(&mut buf)?;

    std::fs::write(args.output_locale_path, buf).map_err(|err| anyhow!(err))
}
