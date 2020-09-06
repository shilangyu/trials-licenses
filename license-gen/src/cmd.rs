use std::collections::HashSet;
use std::error::Error;
use std::path::PathBuf;
use std::process::Command;

use crate::{assets, bikes::Bike};

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
            magick_geometry: "150%x+1300+400",
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
    args.push("+channel");
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

pub fn apply_ticks(
    to: PathBuf,
    basic: Vec<Bike>,
    advanced: Vec<Bike>,
) -> Result<(), Box<dyn Error>> {
    let green_tick = assets::get_path("green_tick.png");
    let red_tick = assets::get_path("red_tick.png");

    let mut args = Vec::new();
    args.push(to.to_str().unwrap());

    let single_pos = |bike| match bike {
        Bike::Armadillo => "+100+400",
        Bike::Tango => "+100+600",
        Bike::Bronco => "+100+800",
        Bike::Jackal => "+500+400",
        Bike::Mantis => "+500+600",
        Bike::Marauder => "+500+800",
        Bike::Riptide => "+900+400",
        Bike::Berserker => "+900+600",
        Bike::Phantom => "+900+800",
        Bike::Donkey => "+1350+400",
        Bike::Stallion => "+1350+600",
        Bike::Agent => "+1350+800",
    };

    let b: HashSet<_> = basic.iter().cloned().collect();
    let a: HashSet<_> = advanced.iter().cloned().collect();
    let single_bikes: HashSet<_> = b.symmetric_difference(&a).collect();

    for bike in basic {
        args.push(green_tick.to_str().unwrap());
        args.push("-geometry");

        if single_bikes.contains(&bike) {
            args.push(single_pos(bike));
        } else {
            args.push(match bike {
                Bike::Armadillo => "+30+400",
                Bike::Tango => "+30+600",
                Bike::Bronco => "+30+800",
                Bike::Jackal => "+430+400",
                Bike::Mantis => "+430+600",
                Bike::Marauder => "+430+800",
                Bike::Riptide => "+830+400",
                Bike::Berserker => "+830+600",
                Bike::Phantom => "+830+800",
                Bike::Donkey => "+1280+400",
                Bike::Stallion => "+1280+600",
                Bike::Agent => "+1280+800",
            });
        }

        args.push("-channel");
        args.push("A");
        args.push("-evaluate");
        args.push("multiply");
        args.push("0.5");
        args.push("-composite");
    }

    for bike in advanced {
        args.push(red_tick.to_str().unwrap());
        args.push("-geometry");

        if single_bikes.contains(&bike) {
            args.push(single_pos(bike));
        } else {
            args.push(match bike {
                Bike::Armadillo => "+190+400",
                Bike::Tango => "+190+600",
                Bike::Bronco => "+190+800",
                Bike::Jackal => "+590+400",
                Bike::Mantis => "+590+600",
                Bike::Marauder => "+590+800",
                Bike::Riptide => "+990+400",
                Bike::Berserker => "+990+600",
                Bike::Phantom => "+990+800",
                Bike::Donkey => "+1440+400",
                Bike::Stallion => "+1440+600",
                Bike::Agent => "+1440+800",
            });
        }

        args.push("-channel");
        args.push("A");
        args.push("-evaluate");
        args.push("multiply");
        args.push("0.5");
        args.push("-composite");
    }

    args.push("-strip");
    args.push("-quality");
    args.push("85%");
    args.push("-gaussian-blur");
    args.push("0.05");
    args.push(to.to_str().unwrap());
    Command::new("convert").args(args).output()?;

    Ok(())
}
