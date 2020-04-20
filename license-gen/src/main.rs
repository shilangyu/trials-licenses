use dirs;
use std::path::PathBuf;
use structopt::StructOpt;

mod args;
mod bikes;
mod cmd;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const NAME: &'static str = env!("CARGO_PKG_NAME");

fn main() {
    save_bins().expect("Failed to save license assets");

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

fn save_bins() -> std::io::Result<()> {
    use std::fs::{create_dir, File};
    use std::io::prelude::*;

    let config_dir = dirs::config_dir().unwrap();
    let config_dir = config_dir.as_path().join(NAME);

    let _ = create_dir(&config_dir);

    /// Takes a png from assets/ (at compile time) and puts it into the config directory of the OS + /license-gen (at runtime)
    macro_rules! save_asset {
        ($name:ident) => {
            File::create(config_dir.join(concat!(stringify!($name), ".png")))?.write_all(
                include_bytes!(concat!("../assets/", stringify!($name), ".png")),
            )?;
        };
    }

    save_asset!(armadillo);
    save_asset!(tango);
    save_asset!(bronco);
    save_asset!(jackal);
    save_asset!(mantis);
    save_asset!(marauder);
    save_asset!(riptide);
    save_asset!(berserker);
    save_asset!(phantom);

    Ok(())
}
