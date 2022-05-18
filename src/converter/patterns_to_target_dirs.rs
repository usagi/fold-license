use crate::conf::Conf;
use itertools::Itertools;

pub fn patterns_to_target_dirs(conf: &Conf) -> Vec<std::path::PathBuf>
{
 let targets = conf
  .r#in
  .clone()
  .unwrap_or_default()
  .iter()
  .filter_map(|pattern| {
   let paths = match glob::glob(pattern.as_str())
   {
    Err(e) =>
    {
     log::error!("⛔ `-i {pattern:?}`, glob this pattern is 🆖. to skip this pattern; e={e:?}");
     return None;
    },
    Ok(paths) =>
    {
     log::debug!("💁 `-i {pattern:?}`, glob this pattern is 🆗.");
     paths
    }
   };

   let dirs = paths
    .into_iter()
    .filter_map(|path_maybe| {
     match path_maybe
     {
      Err(e) =>
      {
       log::error!("  💥 {e:?}");
       None
      },
      Ok(symlink) if symlink.is_symlink() =>
      {
       log::trace!("  🔗 {symlink:?} is a symlink, then skip it.");
       None
      },

      Ok(file) if file.is_file() =>
      {
       log::trace!("  🗃️ {file:?} is a file, then skip it.");
       None
      },
      Ok(dir) if dir.is_dir() =>
      {
       log::debug!("  📁 {dir:?} is a dir, then target it.");
       match dir.is_absolute()
       {
        true => Some(dir),
        false =>
        {
         match std::env::current_dir()
         {
          Ok(dir0) => Some(path_clean::PathClean::clean(&dir0.join(dir))),
          Err(e) =>
          {
           log::error!("Could not get the current path, then it cannot resolve to an absolute path; e={e:?}");
           None
          }
         }
        },
       }
      },
      unknown =>
      {
       log::error!("  💥 unknown error; unknown={unknown:?}");
       None
      }
     }
    })
    .collect::<Vec<_>>();
   match dirs.len()
   {
    0 => log::warn!("    ⚠ It is not an error, but this pattern has no folders."),
    1 => log::info!("    ➕ Added 1 folder to the targets."),
    n => log::info!("    ➕ Added {n} folders to the targets.")
   }
   Some(dirs)
  })
  .flatten()
  .sorted()
  .unique()
  .collect::<Vec<_>>();
 match targets.len()
 {
  0 => log::warn!("  ⚠ It is not an error, but all of patterns has not folders."),
  1 => log::info!("  💖 Total 1 folder to the targets."),
  n => log::info!("  💖 Total {n} folders to the targets.")
 }
 targets
}
