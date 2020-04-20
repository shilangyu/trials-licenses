use crate::assets;
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
            assets::get_path("Comfortaa-Regular.ttf").to_str().unwrap(),
            "-pointsize",
            "60",
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
            assets::get_path("Comfortaa-Regular.ttf").to_str().unwrap(),
            "-pointsize",
            "36",
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
    use crate::bikes::Bike;

    let mut args = Vec::new();
    let blank_path = assets::get_path("blank.png");
    args.push(blank_path.to_str().unwrap());

    struct BikeImg {
        name: Bike,
        magick_geometry: &'static str,
    }

    let bike_data: Vec<_> = [
        BikeImg {
            name: Bike::Armadillo,
            magick_geometry: "150%x+30+400",
        },
        BikeImg {
            name: Bike::Tango,
            magick_geometry: "150%x+30+600",
        },
        BikeImg {
            name: Bike::Bronco,
            magick_geometry: "150%x+30+800",
        },
        BikeImg {
            name: Bike::Jackal,
            magick_geometry: "150%x+430+400",
        },
        BikeImg {
            name: Bike::Mantis,
            magick_geometry: "150%x+430+600",
        },
        BikeImg {
            name: Bike::Marauder,
            magick_geometry: "150%x+430+800",
        },
        BikeImg {
            name: Bike::Riptide,
            magick_geometry: "150%x+830+400",
        },
        BikeImg {
            name: Bike::Berserker,
            magick_geometry: "150%x+830+600",
        },
        BikeImg {
            name: Bike::Phantom,
            magick_geometry: "150%x+830+800",
        },
        BikeImg {
            name: Bike::Donkey,
            magick_geometry: "150%x+1280+400",
        },
        BikeImg {
            name: Bike::Stallion,
            magick_geometry: "150%x+1320+600",
        },
        BikeImg {
            name: Bike::Agent,
            magick_geometry: "150%x+1280+800",
        },
    ]
    .iter()
    .map(|b| (b.name.path_to(), b.magick_geometry.to_string()))
    .collect();

    for (path, geo) in &bike_data {
        args.push(path.to_str().unwrap());
        args.push("-geometry");
        args.push(geo.as_str());
        args.push("-composite");
    }

    let logo_path = assets::get_path("trials_frontier_logo.png");
    args.push(logo_path.to_str().unwrap());
    args.push("-geometry");
    args.push("140%x+870+0");
    args.push("-channel");
    args.push("A");
    args.push("-evaluate");
    args.push("multiply");
    args.push("0.5");
    args.push("-composite");

    args.push(version_path.to_str().unwrap());
    args.push("-composite");

    args.push(nickname_path.to_str().unwrap());
    args.push("-geometry");
    if let Some(path) = &profile_picture_path {
        args.push("+400+200");
        args.push("-composite");

        args.push(path.to_str().unwrap());
        args.push("-geometry");
        args.push("250x250!+100+100");
        args.push("-composite");
    } else {
        args.push("+200+200");
        args.push("-composite");
    }

    args.push(output_path.to_str().unwrap());

    Command::new("convert").args(args).output()?;

    Ok(())
}
