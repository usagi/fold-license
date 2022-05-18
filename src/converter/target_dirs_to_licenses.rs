use crate::{
 conf::Conf,
 license_type::LicenseData,
 retriever
};
use itertools::Itertools;

pub fn target_dirs_to_licenses(target_dirs: &[std::path::PathBuf], conf: &Conf) -> LicenseData
{
 log::debug!("💁 Begin to retrieve the licenses to the given dirs.");

 let enabled_cargo = conf.cargo.unwrap_or_default();
 let enabled_yarn = conf.yarn.unwrap_or_default();

 let original_working_dir = std::env::current_dir().unwrap_or_else(|e| {
  log::error!("🔥 Could not get the current working dir; {e:?}");
  panic!("Could not get the current working dir.")
 });

 let licenses = target_dirs
  .iter()
  .filter_map(|dir| {
   match std::env::set_current_dir(dir)
   {
    Err(e) =>
    {
     log::error!("  💥 Could not set the working dir; dir={dir:?}, e={e:?}");
     return None;
    },
    _ => log::debug!("  📂 Into dir; dir={dir:?}")
   }

   let mut from = vec![];

   if enabled_cargo
   {
    let mut from_cargo = retriever::cargo();
    match from_cargo.len()
    {
     0 => log::trace!("    ⚠ cargo, could not retrieved the licenses."),
     1 => log::info!("    ➕ cargo, retrieved 1 license."),
     n => log::info!("    ➕ cargo, retrieved {n} licenses.")
    }
    from.append(&mut from_cargo);
   }

   if enabled_yarn
   {
    let mut from_yarn = retriever::yarn();
    match from_yarn.len()
    {
     0 => log::trace!("    ⚠ yarn, could not retrieved the licenses."),
     1 => log::info!("    ➕ yarn, retrieved 1 license."),
     n => log::info!("    ➕ yarn, retrieved {n} licenses.")
    }
    from.append(&mut from_yarn);
   }

   Some(from)
  })
  .flatten()
  .sorted_by(|a, b| a.name.cmp(&b.name))
  .sorted_by(|a, b| a.license.cmp(&b.license))
  .unique()
  .collect_vec();

 match std::env::set_current_dir(&original_working_dir)
 {
  Err(e) =>
  {
   log::error!("  🔥 Could not back to the original working dir; dir={original_working_dir:?}, e={e:?}");
   panic!("Could not back to the original working dir.")
  },
  _ => log::debug!("  📁 Back to the original working dir; dir={original_working_dir:?}")
 }

 match licenses.len()
 {
  0 => log::warn!("  ⚠ It is not an error, but a license datum is nothing in the folder."),
  1 => log::info!("  💖 Total 1 license collected."),
  n => log::info!("  💖 Total {n} licenses collected.")
 }

 licenses
}
