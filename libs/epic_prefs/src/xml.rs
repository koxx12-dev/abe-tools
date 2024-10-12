use yaserde_derive::{YaDeserialize, YaSerialize};
#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(rename = "map")]
pub struct PlayerPrefsXml {
    #[yaserde(child, rename = "string")]
    pub strings: Vec<StringPrefXml>,
    #[yaserde(child, rename = "int")]
    pub ints: Vec<IntPrefXml>,
}

#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
pub struct StringPrefXml {
    #[yaserde(attribute)]
    pub name: String,
    #[yaserde(text)]
    pub value: String,
}

#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
pub struct IntPrefXml {
    #[yaserde(attribute)]
    pub name: String,
    #[yaserde(attribute)]
    pub value: String,
}
