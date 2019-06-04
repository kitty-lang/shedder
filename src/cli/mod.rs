use std::path::PathBuf;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Opt {
    #[structopt(parse(from_os_str))]
    pub file: PathBuf,
}
