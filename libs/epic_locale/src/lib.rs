use crate::proto::SerializedLocalizedTexts;
use anyhow::anyhow;
use prost::bytes::{Buf, BufMut};
use prost::Message;
use std::io::{Read, Write};

pub mod proto {
    include!(concat!(env!("OUT_DIR"), "/abepic.locale.rs"));
    include!(concat!(env!("OUT_DIR"), "/abepic.locale.serde.rs"));
}

pub struct LocaleDataContainer {
    locale: SerializedLocalizedTexts,
}

impl LocaleDataContainer {
    pub fn default() -> Self {
        Self {
            locale: SerializedLocalizedTexts::default(),
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
        let locale = SerializedLocalizedTexts::decode(buf)?;
        Ok(Self { locale })
    }

    pub fn get_language_id(self) -> Option<String> {
        self.locale.language_id
    }

    pub fn get_locale_text_ids(self) -> Vec<Option<String>> {
        self.locale
            .texts
            .iter()
            .map(|locale| locale.name_id.clone())
            .collect()
    }

    pub fn get_translated_text(self, key: &str) -> Option<String> {
        self.locale
            .texts
            .iter()
            .find(|locale| locale.name_id.as_deref() == Some(key))
            .and_then(|locale| locale.translated_text.clone())
    }

    pub fn set_translated_text(mut self, key: &str, text: &str) -> anyhow::Result<()> {
        if let Some(locale) = self
            .locale
            .texts
            .iter_mut()
            .find(|locale| locale.name_id.as_deref() == Some(key))
        {
            locale.translated_text = Some(text.to_string());
        }
        Ok(())
    }

    pub fn from_json(json_contents: &str) -> anyhow::Result<Self> {
        let locale = serde_json::from_str::<SerializedLocalizedTexts>(json_contents)?;
        Ok(Self { locale })
    }

    pub fn to_json_pretty(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty::<SerializedLocalizedTexts>(&self.locale)
    }

    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string::<SerializedLocalizedTexts>(&self.locale)
    }

    pub fn get_locale(&self) -> &SerializedLocalizedTexts {
        &self.locale
    }

    pub fn get_locale_mut(&mut self) -> &mut SerializedLocalizedTexts {
        &mut self.locale
    }

    pub fn set_locale(&mut self, data: SerializedLocalizedTexts) {
        self.locale = data;
    }

    pub fn write_gzipped<W>(&self, writer: &mut W) -> anyhow::Result<()>
    where
        W: BufMut + Write,
    {
        let mut encoder = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());

        let mut buf = Vec::new();
        self.locale.encode(&mut buf)?;
        encoder.write_all(&buf)?;

        writer
            .write_all(&encoder.finish()?)
            .map_err(|err| anyhow!(err))
    }

    pub fn write<W>(&self, writer: &mut W) -> anyhow::Result<()>
    where
        W: BufMut + Write,
    {
        self.locale.encode(writer).map_err(|err| anyhow!(err))
    }
}
