mod args;
use anyhow::Result;
use itertools::Itertools;
use serde::Deserialize;

#[derive(Deserialize, strum::EnumString, Debug)]
pub enum Format
{
 #[strum(ascii_case_insensitive)]
 Json,
 #[strum(ascii_case_insensitive)]
 Toml,
 #[strum(ascii_case_insensitive)]
 Msgpack
}

impl Default for Format
{
 fn default() -> Self
 {
  Self::Toml
 }
}

#[derive(Deserialize, Debug, Default)]
pub struct Conf
{
 pub format: Option<Format>,
 pub r#in: Option<Vec<String>>,
 pub out: Option<String>,
 pub silent: Option<bool>,
 pub pretty: Option<bool>,
 pub cargo: Option<bool>,
 pub yarn: Option<bool>
}

impl Conf
{
 pub fn init() -> Result<Self>
 {
  let args = args::Args::init();
  let mut c = match args.conf
  {
   Some(args_conf) =>
   {
    let cont_toml_maybe = std::fs::read_to_string(args_conf)?;
    toml::from_str::<Self>(&cont_toml_maybe)?
   },
   None => Self::default()
  };
  c.r#in = Some(
   c.r#in
    .unwrap_or_default()
    .into_iter()
    .chain(args.r#in.into_iter())
    .sorted()
    .unique()
    .collect()
  );
  args.out.and_then(|o| Some(c.out = Some(o)));
  args.format.and_then(|t| Some(c.format = Some(t)));
  args.silent.then(|| c.silent = Some(true));
  args.pretty.then(|| c.pretty = Some(true));
  args.cargo.then(|| c.cargo = Some(true));
  args.yarn.then(|| c.yarn = Some(true));
  Ok(c)
 }
}
