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
        })
    }
}
