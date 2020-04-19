use std::path::PathBuf;
use structopt::StructOpt;

/// A license generator for Trials Licenses
#[derive(StructOpt, Debug)]
#[structopt(name = "license-gen")]
pub struct Opt {
    /// Player's nickname
    #[structopt(short, long)]
    pub nickname: String,

    /// Path to profile picture
    #[structopt(short, long, parse(from_os_str))]
    pub profile_picture: Option<PathBuf>,

    /// Output image path
    #[structopt(short, long, parse(from_os_str))]
    pub output: PathBuf,
}
