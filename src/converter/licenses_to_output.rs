use std::io::Write;
use write_to_file::WriteToFile;

use crate::{
 conf::Format,
 license_type::LicenseDatum
};

pub fn licenses_to_output(licenses: &[LicenseDatum], format: Format, pretty: bool, out: Option<String>) -> Option<()>
{
 match format
 {
  Format::Msgpack =>
  {
   let payload = rmp_serde::to_vec(licenses).ok()?;
   match out
   {
    Some(path) => payload.write_to_file(path).ok(),
    None => std::io::stdout().write_all(&payload).ok()
   }
  },
  _ =>
  {
   let payload = match (format, pretty)
   {
    (Format::Toml, false) => toml::to_string(licenses).ok()?,
    (Format::Toml, true) => toml::to_string_pretty(licenses).ok()?,
    (Format::Json, false) => serde_json::to_string(licenses).ok()?,
    (Format::Json, true) => serde_json::to_string_pretty(licenses).ok()?,
    _ => panic!("should be unreachable; #1")
   };
   match out
   {
    Some(path) => payload.write_to_file(path).ok(),
    None => Some(println!("{payload}"))
   }
  }
 }
}
