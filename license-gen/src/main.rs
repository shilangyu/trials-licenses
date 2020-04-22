use std::path::PathBuf;
use structopt::StructOpt;

mod args;
mod assets;
mod bikes;
mod cmd;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const NAME: &'static str = env!("CARGO_PKG_NAME");

fn main() {
    assets::save().expect("Failed to save license assets");

    let mut args = args::Opt::from_args();

    if let None = args.output {
        args.output = Some(PathBuf::from(args.nickname.clone() + ".jpg"));
    }

    println!("Generating nickname...");
    let nickname_path = cmd::generate_nickname_pic(args.nickname.as_str())
        .expect("Failed to generate nickname png");

    println!("Generating version...");
    let version_path = cmd::generate_version_pic().expect("Failed to generate version png");

    println!("Generating blank license...");
    cmd::generate_blank_license(
        nickname_path,
        version_path,
        args.profile_picture,
        args.output.clone().unwrap(),
    )
    .expect("Failed to generate blank license");

    println!("Applying ticks...");
    cmd::apply_ticks(args.output.unwrap(), args.basic, args.advanced)
        .expect("Failed to apply ticks");
}
