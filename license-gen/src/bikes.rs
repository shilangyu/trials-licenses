use std::path::PathBuf;

#[derive(Debug)]
pub enum Bike {
    Armadillo,
    Tango,
    Bronco,
    Jackal,
    Mantis,
    Marauder,
    Riptide,
    Berserker,
    Phantom,
    Donkey,
    Stallion,
    Agent,
}

impl std::str::FromStr for Bike {
    type Err = &'static str;

    fn from_str(name: &str) -> Result<Self, Self::Err> {
        Ok(match name {
            "armadillo" => Self::Armadillo,
            "tango" => Self::Tango,
            "bronco" => Self::Bronco,
            "jackal" => Self::Jackal,
            "mantis" => Self::Mantis,
            "marauder" => Self::Marauder,
            "riptide" => Self::Riptide,
            "berserker" => Self::Berserker,
            "phantom" => Self::Phantom,
            "donkey" => Self::Donkey,
            "stallion" => Self::Stallion,
            "agent" => Self::Agent,
            _ => return Err("Bike does not exist"),
        })
    }
}

impl Bike {
    pub fn to_string(&self) -> String {
        String::from(match self {
            Self::Armadillo => "armadillo",
            Self::Tango => "tango",
            Self::Bronco => "bronco",
            Self::Jackal => "jackal",
            Self::Mantis => "mantis",
            Self::Marauder => "marauder",
            Self::Riptide => "riptide",
            Self::Berserker => "berserker",
            Self::Phantom => "phantom",
            Self::Donkey => "donkey",
            Self::Stallion => "stallion",
            Self::Agent => "agent",
        })
    }

    pub fn path_to(&self) -> PathBuf {
        let asset = self.to_string() + ".png";
        crate::assets::get_path(asset.as_str())
    }
}
