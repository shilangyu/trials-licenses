use std::path::PathBuf;
use structopt::StructOpt;

mod args;
mod cmd;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
    let mut opt = args::Opt::from_args();

    if let None = opt.output {
        opt.output = Some(PathBuf::from(opt.nickname.clone() + ".png"));
    }

    println!("Generating nickname...");
    let nickname_path =
        cmd::generate_nickname_pic(opt.nickname.as_str()).expect("Failed to generate nickname png");

    println!("Generating version...");
    let version_path = cmd::generate_version_pic().expect("Failed to generate version png");

    println!("Generating blank license...");
    cmd::generate_blank_license(
        nickname_path,
        version_path,
        opt.profile_picture,
        opt.output.unwrap(),
    )
    .expect("Failed to generate blank license");
}
