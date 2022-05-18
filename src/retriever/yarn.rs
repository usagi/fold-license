use crate::license_type::LicenseDatum;
use itertools::Itertools;
use serde::Deserialize;
use std::collections::HashMap;

const COMMAND: &str = "yarn";
const ARGS: &[&str] = &["licenses", "--json", "--no-progress", "list"];

pub fn yarn() -> Vec<LicenseDatum>
{
 match std::process::Command::new(COMMAND).args(ARGS).output()
 {
  Err(e) =>
  {
   log::trace!("🐱‍👤 `{COMMAND}` failed, but it is not an error; e={e:?}");
   None
  },
  Ok(o) =>
  {
   let mut r = vec![];

   || -> Option<()> {
    let yl = serde_json::from_slice::<YarnLicenses>(&o.stdout)
     .map_err(|e| log::error!("🔥 `{COMMAND}` succeeded, but output parser was fail; e={e:?}"))
     .ok()?;

    let header = HashMap::<&str, usize>::from_iter(yl.data.head.iter().enumerate().map(|(n, key)| (key.as_ref(), n)));

    let index_of_name = *header.get(KEY_NAME).or_else(|| {
     log::error!("🔥 `{COMMAND}` failed to parse, {KEY_NAME:?} key is required.");
     None
    })?;
    let index_of_version = *header.get(KEY_VERSION).or_else(|| {
     log::error!("🔥 `{COMMAND}` failed to parse, {KEY_VERSION:?} key is required.");
     None
    })?;
    let index_of_author_name = *header.get(KEY_VENDOR_NAME).or_else(|| {
     log::error!("🔥 `{COMMAND}` failed to parse, {KEY_VENDOR_NAME:?} key is required.");
     None
    })?;
    let index_of_author_url = *header.get(KEY_VENDOR_URL).or_else(|| {
     log::error!("🔥 `{COMMAND}` failed to parse, {KEY_VENDOR_URL:?} key is required.");
     None
    })?;
    let index_of_repository = *header.get(KEY_URL).or_else(|| {
     log::error!("🔥 `{COMMAND}` failed to parse, {KEY_URL:?} key is required.");
     None
    })?;
    let index_of_license = *header.get(KEY_LICENSE).or_else(|| {
     log::error!("🔥 `{COMMAND}` failed to parse, {KEY_LICENSE:?} key is required.");
     None
    })?;

    for datum in yl.data.body
    {
     r.push(LicenseDatum {
      name: datum[index_of_name].clone(),
      version: datum[index_of_version].clone(),
      authors: match (datum[index_of_author_name].as_str(), datum[index_of_author_url].as_str())
      {
       ("Unknown", "Unknown") => String::new(),
       ("Unknown", url) => format!("<{url}>"),
       (name, "Unknown") => name.to_string(),
       (name, url) => format!("{name} <{url}>")
      },
      repository: datum[index_of_repository].clone(),
      license: datum[index_of_license].clone(),
      from: COMMAND.into()
     });
    }

    Some(())
   }();
   Some(r)
  }
 }
 .unwrap_or_default()
 .into_iter()
 .map(|datum| datum.with_from(COMMAND))
 .collect_vec()
}

pub const KEY_NAME: &str = "Name";
pub const KEY_VERSION: &str = "Version";
pub const KEY_VENDOR_NAME: &str = "VendorName";
pub const KEY_VENDOR_URL: &str = "VendorUrl";
pub const KEY_URL: &str = "URL";
pub const KEY_LICENSE: &str = "License";

#[derive(Deserialize)]
pub struct YarnLicenses
{
 pub r#type: String,
 pub data: YarnLicensesData
}

#[derive(Deserialize)]
pub struct YarnLicensesData
{
 pub head: Vec<String>,
 pub body: Vec<Vec<String>>
}
