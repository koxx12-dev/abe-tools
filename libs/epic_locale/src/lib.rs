use crate::proto::{LocaleBalancingDataBase, SerializedLocalizedTexts};
use prost::bytes::{Buf, BufMut};
use prost::Message;
use std::io::{Read, Write};

#[cfg(feature = "ron")]
use ron::{
    extensions::Extensions,
    Options
};

pub mod proto {
    include!(concat!(env!("OUT_DIR"), "/abepic.locale.rs"));
    #[cfg(feature = "serde")]
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

    #[cfg(feature = "json")]
    pub fn from_json(contents: &str) -> anyhow::Result<Self> {
        let locale = serde_json::from_str::<SerializedLocalizedTexts>(contents)?;
        Ok(Self { locale })
    }

    #[cfg(feature = "json")]
    pub fn to_json_pretty(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty::<SerializedLocalizedTexts>(&self.locale)
    }

    #[cfg(feature = "json")]
    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string::<SerializedLocalizedTexts>(&self.locale)
    }

    #[cfg(feature = "ron")]
    pub fn from_ron(contents: &str) -> anyhow::Result<Self> {
        //todo: there is def a less stupid way of doing this
        let options = Options::default().with_default_extension(Extensions::IMPLICIT_SOME);
        let locale = options.from_str::<SerializedLocalizedTexts>(contents)?;
        Ok(Self { locale })
    }

    #[cfg(feature = "ron")]
    pub fn to_ron(&self) -> ron::Result<String> {
        ron::ser::to_string::<SerializedLocalizedTexts>(&self.locale)
    }

    #[cfg(feature = "ron")]
    pub fn to_ron_pretty(&self) -> ron::Result<String> {
        ron::ser::to_string_pretty::<SerializedLocalizedTexts>(&self.locale, Default::default())
    }
    
    #[cfg(feature = "csv")]
    pub fn from_csv(contents: &str) -> anyhow::Result<Self> {
        let mut reader = csv::Reader::from_reader(contents.as_bytes());
        let mut locale = SerializedLocalizedTexts::default();
        for result in reader.deserialize() {
            let record: LocaleBalancingDataBase = result?;
            locale.texts.push(record);
        }
        Ok(Self { locale })
    }
    
    #[cfg(feature = "csv")]
    pub fn to_csv(&self) -> anyhow::Result<String> {
        let mut wtr = csv::Writer::from_writer(vec![]);
        for record in &self.locale.texts {
            wtr.serialize(record)?;
        }
        Ok(String::from_utf8(wtr.into_inner()?)?)
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
    pub fn sort(&mut self) {
        self.locale.texts.sort_by(|a, b| a.name_id.cmp(&b.name_id));
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
            .map_err(anyhow::Error::new)
    }

    pub fn write<W>(&self, writer: &mut W) -> anyhow::Result<()>
    where
        W: BufMut + Write,
    {
        self.locale.encode(writer).map_err(anyhow::Error::new)
    }
}
