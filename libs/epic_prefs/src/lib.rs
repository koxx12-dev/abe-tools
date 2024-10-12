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

pub mod proto {
    pub mod prefs {
        include!(concat!(env!("OUT_DIR"), "/abepic.prefs.rs"));
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

    pub fn from_xml(xml_contents: &str) -> anyhow::Result<Self> {
        let mut xml =
            yaserde::de::from_str::<PlayerPrefsXml>(xml_contents).map_err(|err| anyhow!(err))?;
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

    pub fn from_json(json_contents: &str) -> anyhow::Result<Self> {
        let data = serde_json::from_str::<PlayerData>(json_contents)?;
        Ok(Self { data })
    }

    pub fn to_json_pretty(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty::<PlayerData>(&self.data)
    }

    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string::<PlayerData>(&self.data)
    }

    pub fn to_xml(&self, xml_contents: &str, config: Option<Config>) -> anyhow::Result<String> {
        let config = config.unwrap_or(Config {
            perform_indent: true,
            ..Default::default()
        });

        let mut xml =
            yaserde::de::from_str::<PlayerPrefsXml>(xml_contents).map_err(|err| anyhow!(err))?;
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

        Ok(yaserde::ser::to_string_with_config(&xml, &config).map_err(|err| anyhow!(err))?)
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
