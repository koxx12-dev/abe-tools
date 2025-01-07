use clap::{Args, Subcommand, ValueEnum};
use epic_locale::LocaleDataContainer;
use std::fmt::Display;
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

#[derive(ValueEnum, Copy, Clone)]
pub(super) enum DataFormat {
    Ron,
    Json,
    Csv,
}

#[derive(Args, Clone)]
#[command(version, about, long_about = Some("Decode a language locale file"), aliases = ["d", "unpack", "export"]
)]
pub(super) struct LocaleDecodeArgs {
    #[arg(help = "Location of the serialized language locale file")]
    pub language_locale_path: PathBuf,

    #[arg(help = "Location to save the decoded language locale file")]
    pub output_locale_path: PathBuf,

    #[arg(help = "Sort the locale file by key alphabetically", long, short = 's')]
    pub sort: bool,

    #[arg(help = "Data format to output the language locale file in", long, short = 'O', default_value_t=DataFormat::Json)]
    pub output_as: DataFormat,
}

#[derive(Args, Clone)]
#[command(version, about, long_about = Some("Encode a language locale file"), aliases = ["e", "pack", "import", "reimport"]
)]
pub(super) struct LocaleEncodeArgs {
    #[arg(help = "Path to the locale file to be encoded")]
    pub locale_json_file: PathBuf,

    #[arg(help = "Location to save the encoded language locale file")]
    pub output_locale_path: PathBuf,
}

pub(super) fn decode_locale(_: LocaleArgs, args: LocaleDecodeArgs) -> anyhow::Result<()> {
    let locale_file = std::fs::read(args.language_locale_path)?;

    let mut locale_container = match LocaleDataContainer::new_gzipped(locale_file.as_slice()) {
        Ok(reader) => reader,
        Err(_) => LocaleDataContainer::new(locale_file.as_slice())?,
    };
    
    if args.sort {
        locale_container.sort();
    }

    let data = match args.output_as {
        DataFormat::Ron => locale_container.to_ron_pretty()?,
        DataFormat::Json => locale_container.to_json_pretty()?,
        DataFormat::Csv => locale_container.to_csv()?,
    };

    std::fs::write(args.output_locale_path, data).map_err(anyhow::Error::new)
}

pub(super) fn encode_locale(_: LocaleArgs, args: LocaleEncodeArgs) -> anyhow::Result<()> {
    let data = std::fs::read_to_string(args.locale_json_file)?;

    let data_format = if data.starts_with("{") {
        DataFormat::Json
    } else if data.starts_with("(") {
        DataFormat::Ron
    } else {
        DataFormat::Csv
    };

    let locale_container = match data_format {
        DataFormat::Ron => LocaleDataContainer::from_ron(data.as_str())?,
        DataFormat::Json => LocaleDataContainer::from_json(data.as_str())?,
        DataFormat::Csv => LocaleDataContainer::from_csv(data.as_str())?,
    };

    let mut buf = Vec::new();
    locale_container.write_gzipped(&mut buf)?;

    std::fs::write(args.output_locale_path, buf).map_err(anyhow::Error::new)
}

impl Display for DataFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataFormat::Ron => write!(f, "ron"),
            DataFormat::Json => write!(f, "json"),
            DataFormat::Csv => write!(f, "csv"),
        }
    }
}
