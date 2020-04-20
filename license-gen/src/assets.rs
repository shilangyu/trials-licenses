use dirs;
use std::fs::{create_dir, File};
use std::io::prelude::*;
use std::path::PathBuf;

pub fn save() -> std::io::Result<()> {
    let _ = create_dir(get_path(""));

    /// Takes a png from assets/ (at compile time) and puts it into the config directory of the OS + /license-gen (at runtime)
    macro_rules! save_png {
        ($name:ident) => {
            File::create(get_path(concat!(stringify!($name), ".png")))?.write_all(
                include_bytes!(concat!("../assets/", stringify!($name), ".png")),
            )?;
        };
    }

    save_png!(armadillo);
    save_png!(tango);
    save_png!(bronco);
    save_png!(jackal);
    save_png!(mantis);
    save_png!(marauder);
    save_png!(riptide);
    save_png!(berserker);
    save_png!(phantom);

    Ok(())
}

pub fn get_path(asset: &str) -> PathBuf {
    let config_dir = dirs::config_dir().unwrap();

    config_dir.as_path().join(crate::NAME).join(asset)
}
