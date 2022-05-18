use serde::{
 Deserialize,
 Deserializer,
 Serialize
};

#[derive(Serialize, Deserialize, Default, Debug, Hash, PartialEq, Eq, Clone)]
pub struct LicenseDatum
{
 pub name: String,
 #[serde(deserialize_with = "null_to_default")]
 pub version: String,
 #[serde(deserialize_with = "null_to_default")]
 pub authors: String,
 #[serde(deserialize_with = "null_to_default")]
 pub repository: String,
 #[serde(deserialize_with = "null_to_default")]
 pub license: String,
 #[serde(default)]
 pub from: String
}

impl LicenseDatum
{
 pub fn with_from(mut self, f: &str) -> Self
 {
  self.from = f.into();
  self
 }
}

pub type LicenseData = Vec<LicenseDatum>;

fn null_to_default<'de, D, T>(d: D) -> Result<T, D::Error>
where
 T: Default + Deserialize<'de>,
 D: Deserializer<'de>
{
 let o = Option::deserialize(d)?;
 Ok(o.unwrap_or_default())
}
