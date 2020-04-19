use std::error::Error;
use std::path::PathBuf;
use std::process::Command;

pub fn generate_nickname_pic(nickname: &str) -> Result<PathBuf, Box<dyn Error>> {
    let label_string = format!("label:{}", nickname);
    let out_string = format!("/tmp/{}.png", nickname);

    Command::new("convert")
        .args(&[
            "-background",
            "none",
            "-fill",
            "black",
            "-font",
            "/home/shilangyu/.local/share/fonts/JetBrainsMono-Regular.ttf",
            "-pointsize",
            "72",
            label_string.as_str(),
            out_string.as_str(),
        ])
        .output()?;

    Ok(PathBuf::from(out_string))
}

pub fn generate_version_pic() -> Result<PathBuf, Box<dyn Error>> {
    let label_string = format!("label:v{}", crate::VERSION);
    let out_string = format!("/tmp/v{}.png", crate::VERSION);

    Command::new("convert")
        .args(&[
            "-background",
            "none",
            "-fill",
            "black",
            "-font",
            "/home/shilangyu/.local/share/fonts/JetBrainsMono-Regular.ttf",
            "-pointsize",
            "72",
            label_string.as_str(),
            out_string.as_str(),
        ])
        .output()?;

    Ok(PathBuf::from(out_string))
}

pub fn generate_blank_license(
    nickname_path: PathBuf,
    version_path: PathBuf,
    profile_picture_path: Option<String>,
    output_path: PathBuf,
) -> Result<(), Box<dyn Error>> {
    use crate::magick::geometry::*;

    let mut args = Vec::new();
    args.push("assets/license.png");

    struct Bike {
        name: &'static str,
        magick_geometry: Geometry,
    }

    let bike_data: Vec<_> = [
        Bike {
            name: "armadillo",
            magick_geometry: Geometry {
                resize: None,
                position: Some(Position { x: 0, y: 500 }),
            },
        },
        Bike {
            name: "tango",
            magick_geometry: Geometry {
                resize: None,
                position: Some(Position { x: 0, y: 500 }),
            },
        },
        Bike {
            name: "bronco",
            magick_geometry: Geometry {
                resize: None,
                position: Some(Position { x: 0, y: 500 }),
            },
        },
        Bike {
            name: "jackal",
            magick_geometry: Geometry {
                resize: None,
                position: Some(Position { x: 0, y: 500 }),
            },
        },
        Bike {
            name: "mantis",
            magick_geometry: Geometry {
                resize: None,
                position: Some(Position { x: 0, y: 500 }),
            },
        },
        Bike {
            name: "marauder",
            magick_geometry: Geometry {
                resize: None,
                position: Some(Position { x: 0, y: 500 }),
            },
        },
        Bike {
            name: "riptide",
            magick_geometry: Geometry {
                resize: None,
                position: Some(Position { x: 0, y: 500 }),
            },
        },
        Bike {
            name: "berzerker",
            magick_geometry: Geometry {
                resize: None,
                position: Some(Position { x: 0, y: 500 }),
            },
        },
        Bike {
            name: "phantom",
            magick_geometry: Geometry {
                resize: None,
                position: Some(Position { x: 0, y: 500 }),
            },
        },
    ]
    .iter()
    .map(|b| {
        (
            format!("assets/{}.png", b.name),
            b.magick_geometry.to_string(),
        )
    })
    .collect();

    for (path, geo) in &bike_data {
        args.push(path.as_str());
        args.push("-geometry");
        args.push(geo.as_str());
        args.push("-composite");
    }

    args.push(nickname_path.to_str().unwrap());
    args.push("-geometry");
    args.push("+0+0");
    args.push("-composite");

    args.push(version_path.to_str().unwrap());
    args.push("-geometry");
    args.push("+100+100");
    args.push("-composite");

    args.push(output_path.to_str().unwrap());

    Command::new("convert").args(args).output()?;

    Ok(())
}
