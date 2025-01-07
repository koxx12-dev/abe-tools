mod datetime;
mod xml;

use crate::proto::prefs::PlayerData;
use crate::xml::PlayerPrefsXml;
use anyhow::anyhow;
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use prost::bytes::Buf;
use prost::Message;
use url_escape::NON_ALPHANUMERIC;
use yaserde::ser::Config;

#[cfg(feature = "ron")]
use ron::{
    extensions::Extensions,
    Options
};

pub mod proto {
    pub mod prefs {
        include!(concat!(env!("OUT_DIR"), "/abepic.prefs.rs"));
        #[cfg(feature = "serde")]
        include!(concat!(env!("OUT_DIR"), "/abepic.prefs.serde.rs"));
    }

    pub mod bcl {
        include!(concat!(env!("OUT_DIR"), "/abepic.bcl.rs"));
    }
}

pub struct PlayerPrefsData {
    data: PlayerData,
}

impl PlayerPrefsData {
    pub fn default() -> Self {
        Self {
            data: PlayerData::default(),
        }
    }

    pub fn from_prefs_xml(xml_contents: &str) -> anyhow::Result<Self> {
        let mut xml =
            yaserde::de::from_str::<PlayerPrefsXml>(xml_contents).map_err(anyhow::Error::msg)?;
        let player_key = xml
            .strings
            .iter_mut()
            .filter(|x| x.name == "player")
            .next()
            .ok_or(anyhow!("no player key"))?;
        let player_data = &player_key.value;
        let decoded = BASE64_STANDARD.decode(url_escape::decode(player_data).as_bytes())?;

        let data = PlayerData::decode(decoded.as_slice())?;
        Ok(Self { data })
    }

    #[cfg(feature = "json")]
    pub fn from_json(contents: &str) -> anyhow::Result<Self> {
        let data = serde_json::from_str::<PlayerData>(contents)?;
        Ok(Self { data })
    }

    #[cfg(feature = "json")]
    pub fn to_json_pretty(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty::<PlayerData>(&self.data)
    }

    #[cfg(feature = "json")]
    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string::<PlayerData>(&self.data)
    }

    #[cfg(feature = "ron")]
    pub fn from_ron(contents: &str) -> anyhow::Result<Self> {
        //todo: there is def a less stupid way of doing this
        let options = Options::default().with_default_extension(Extensions::IMPLICIT_SOME);
        let data = options.from_str::<PlayerData>(contents)?;
        Ok(Self { data })
    }

    #[cfg(feature = "ron")]
    pub fn to_ron(&self) -> ron::Result<String> {
        ron::ser::to_string::<PlayerData>(&self.data)
    }

    #[cfg(feature = "ron")]
    pub fn to_ron_pretty(&self) -> ron::Result<String> {
        ron::ser::to_string_pretty::<PlayerData>(&self.data, Default::default())
    }

    pub fn to_prefs_xml(&self, xml_contents: &str, config: Option<Config>) -> anyhow::Result<String> {
        let config = config.unwrap_or(Config {
            perform_indent: true,
            ..Default::default()
        });

        let mut xml =
            yaserde::de::from_str::<PlayerPrefsXml>(xml_contents).map_err(anyhow::Error::msg)?;
        let xml_player_key = xml
            .strings
            .iter_mut()
            .filter(|x| x.name == "player")
            .next()
            .ok_or(anyhow!("no player key"))?;

        let mut encoded_player_data = Vec::new();
        self.data.encode(&mut encoded_player_data)?;
        let encoded_player_data = url_escape::encode(
            BASE64_STANDARD.encode(&encoded_player_data).as_str(),
            NON_ALPHANUMERIC,
        )
        .to_string();

        xml_player_key.value = encoded_player_data;

        Ok(yaserde::ser::to_string_with_config(&xml, &config).map_err(anyhow::Error::msg)?)
    }

    pub fn get_data(&self) -> &PlayerData {
        &self.data
    }

    pub fn get_data_mut(&mut self) -> &mut PlayerData {
        &mut self.data
    }

    pub fn set_data(&mut self, data: PlayerData) {
        self.data = data;
    }

    pub fn new<B>(buf: B) -> anyhow::Result<Self>
    where
        B: Buf,
    {
        let data = PlayerData::decode(buf)?;
        Ok(Self { data })
    }
}
