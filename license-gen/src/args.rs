use std::path::PathBuf;
use structopt::StructOpt;

use crate::bikes::Bike;

/// A license generator for Trials Licenses
#[derive(StructOpt, Debug)]
#[structopt(name = "license-gen")]
pub struct Opt {
    /// Player's nickname
    #[structopt(short, long)]
    pub nickname: String,

    /// Path to the profile picture
    #[structopt(short, long, parse(from_os_str))]
    pub profile_picture: Option<PathBuf>,

    /// Basic license bikes
    #[structopt(short, long)]
    pub basic: Vec<Bike>,

    /// Advanced license bikes
    #[structopt(short, long)]
    pub advanced: Vec<Bike>,

    /// Output image path [default: $nickname.png]
    #[structopt(short, long, parse(from_os_str))]
    pub output: Option<PathBuf>,
}
