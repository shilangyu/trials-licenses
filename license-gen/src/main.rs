use dirs;
use std::path::PathBuf;
use structopt::StructOpt;

mod args;
mod bikes;
mod cmd;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

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
    let config_dir = config_dir.as_path().join("license-gen");

    let _ = create_dir(&config_dir);

    {
        static ARMADILLO: &'static [u8] = include_bytes!("../assets/armadillo.png");
        let mut file = File::create(config_dir.join("armadillo.png"))?;
        file.write_all(ARMADILLO)?;
    }
    {
        static TANGO: &'static [u8] = include_bytes!("../assets/tango.png");
        let mut file = File::create(config_dir.join("tango.png"))?;
        file.write_all(TANGO)?;
    }
    {
        static BRONCO: &'static [u8] = include_bytes!("../assets/bronco.png");
        let mut file = File::create(config_dir.join("bronco.png"))?;
        file.write_all(BRONCO)?;
    }
    {
        static JACKAL: &'static [u8] = include_bytes!("../assets/jackal.png");
        let mut file = File::create(config_dir.join("jackal.png"))?;
        file.write_all(JACKAL)?;
    }
    {
        static MARAUDER: &'static [u8] = include_bytes!("../assets/marauder.png");
        let mut file = File::create(config_dir.join("MARAUDER.png"))?;
        file.write_all(MARAUDER)?;
    }
    {
        static RIPTIDE: &'static [u8] = include_bytes!("../assets/riptide.png");
        let mut file = File::create(config_dir.join("riptide.png"))?;
        file.write_all(RIPTIDE)?;
    }
    {
        static BERSERKER: &'static [u8] = include_bytes!("../assets/berserker.png");
        let mut file = File::create(config_dir.join("berserker.png"))?;
        file.write_all(BERSERKER)?;
    }
    {
        static PHANTOM: &'static [u8] = include_bytes!("../assets/phantom.png");
        let mut file = File::create(config_dir.join("phantom.png"))?;
        file.write_all(PHANTOM)?;
    }

    Ok(())
}
