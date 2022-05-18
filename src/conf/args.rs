use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args
{
 /// Use configuration file if set the path. -c path/to/conf.toml
 #[clap(short, long)]
 pub conf: Option<String>,

 /// toml, json, msgpack. -e json
 #[clap(short, long)]
 pub format: Option<super::Format>,

 /// input glob pattern(s). -i aaa -i bbb -i ccc ...
 #[clap(short, long)]
 pub r#in: Vec<String>,

 /// output to the path if set. else, output to stdout.
 #[clap(short, long)]
 pub out: Option<String>,

 /// silet a log messages.
 #[clap(short, long)]
 pub silent: bool,

 /// pretty output. **ONLY TO USE WITH A TEXT FORMAT**
 #[clap(short, long)]
 pub pretty: bool,

 /// `cargo`/Cargo.toml, enabled.
 #[clap(long)]
 pub cargo: bool,

 /// `yarn`/packages.json, enabled.
 #[clap(long)]
 pub yarn: bool
}

impl Args
{
 pub fn init() -> Self
 {
  Self::parse()
 }
}
