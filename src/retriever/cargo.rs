use crate::license_type::LicenseDatum;
use itertools::Itertools;

const COMMAND: &str = "cargo";
const ARGS: &[&str] = &["license", "-j"];

pub fn cargo() -> Vec<LicenseDatum>
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
   Some(
    serde_json::from_slice::<Vec<LicenseDatum>>(&o.stdout).unwrap_or_else(|e| {
     log::error!("🔥 `{COMMAND}` succeeded, but output parser was fail; e={e:?}");
     return vec![];
    })
   )
  },
 }
 .unwrap_or_default()
 .into_iter()
 .map(|datum| datum.with_from(COMMAND))
 .collect_vec()
}
