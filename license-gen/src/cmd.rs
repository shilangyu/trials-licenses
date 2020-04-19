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
    profile_picture_path: Option<PathBuf>,
    output_path: PathBuf,
) -> Result<(), Box<dyn Error>> {
    let mut args = Vec::new();
    args.push("assets/license.png");

    struct Bike {
        name: &'static str,
        magick_geometry: &'static str,
    }

    let bike_data: Vec<_> = [
        Bike {
            name: "armadillo",
            magick_geometry: "+0+500",
        },
        Bike {
            name: "tango",
            magick_geometry: "+0+700",
        },
        Bike {
            name: "bronco",
            magick_geometry: "+0+900",
        },
        Bike {
            name: "jackal",
            magick_geometry: "+300+500",
        },
        Bike {
            name: "mantis",
            magick_geometry: "+300+700",
        },
        Bike {
            name: "marauder",
            magick_geometry: "+300+900",
        },
        Bike {
            name: "riptide",
            magick_geometry: "+600+500",
        },
        Bike {
            name: "berzerker",
            magick_geometry: "+600+700",
        },
        Bike {
            name: "phantom",
            magick_geometry: "+600+900",
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

    if let Some(path) = &profile_picture_path {
        args.push(path.to_str().unwrap());
        args.push("-geometry");
        args.push("100x100!+100+100");
        args.push("-composite");
    }

    args.push(output_path.to_str().unwrap());

    Command::new("convert").args(args).output()?;

    Ok(())
}
