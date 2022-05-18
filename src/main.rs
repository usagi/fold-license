pub mod conf;
pub mod converter;
pub mod license_type;
pub mod retriever;

fn main()
{
 let conf = conf::Conf::init().unwrap();

 let log_level = conf
  .silent
  .and_then(|s| s.then(|| log::LevelFilter::Off))
  .unwrap_or(log::LevelFilter::Trace);
 femme::with_level(log_level);

 let target_dirs = converter::patterns_to_target_dirs(&conf);
 let licenses = converter::target_dirs_to_licenses(target_dirs.as_slice(), &conf);

 converter::licenses_to_output(
  licenses.as_slice(),
  conf.format.unwrap_or_default(),
  conf.pretty.unwrap_or_default(),
  conf.out
 )
 .unwrap();
}
