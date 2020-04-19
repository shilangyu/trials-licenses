use structopt::StructOpt;

mod args;
mod cmd;
mod magick;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
    let opt = args::Opt::from_args();

    println!("Generating nickname...");
    let nickname_path =
        cmd::generate_nickname_pic(opt.nickname.as_str()).expect("Failed to generate nickname png");

    println!("Generating version...");
    let version_path = cmd::generate_version_pic().expect("Failed to generate version png");

    println!("Generating blank license...");
    cmd::generate_blank_license(nickname_path, version_path, None, opt.output)
        .expect("Failed to generate blank license");
}
