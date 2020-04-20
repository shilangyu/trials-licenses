use dirs;
use std::fs::{create_dir, File};
use std::io::prelude::*;
use std::path::PathBuf;

pub fn save() -> std::io::Result<()> {
    let _ = create_dir(get_path(""));

    /// Takes a file from assets/ (at compile time) and puts it into the config directory of the OS + /license-gen (at runtime)
    macro_rules! save_asset {
        ($name:literal) => {
            File::create(get_path($name))?
                .write_all(include_bytes!(concat!("../assets/", $name)))?;
        };
    }

    save_asset!("armadillo.png");
    save_asset!("tango.png");
    save_asset!("bronco.png");
    save_asset!("jackal.png");
    save_asset!("mantis.png");
    save_asset!("marauder.png");
    save_asset!("riptide.png");
    save_asset!("berserker.png");
    save_asset!("phantom.png");
    save_asset!("Comfortaa-Regular.ttf");
    save_asset!("green_tick.png");
    save_asset!("red_tick.png");
    save_asset!("blank.png");

    Ok(())
}

pub fn get_path(asset: &str) -> PathBuf {
    let config_dir = dirs::config_dir().unwrap();

    config_dir.as_path().join(crate::NAME).join(asset)
}
