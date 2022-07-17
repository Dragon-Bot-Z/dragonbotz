
pub enum Rarity {
    COMMON,
    UNCOMMON,
    SUPER,
    EXTREME,
    ULTRA,
    KAMI,
    ORIGINS,
}

impl Rarity {
    pub fn id(&self) -> i16 {
        match &self {
            Rarity::COMMON => 0,
            Rarity::UNCOMMON => 1,
            Rarity::SUPER => 2,
            Rarity::EXTREME => 3,
            Rarity::ULTRA => 4,
            Rarity::KAMI => 5,
            _ => 0,
        }
    }
}

impl std::fmt::Display for Rarity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rarity = match &self {
            Rarity::COMMON => "**Common**",
            Rarity::UNCOMMON => "**Uncommon**",
            Rarity::SUPER => "**Super**",
            Rarity::EXTREME => "**Extreme**",
            Rarity::ULTRA => "**Ultra**",
            Rarity::KAMI => "**Kami**",
            Rarity::ORIGINS => "*Origins*"
        };

        write!(f, "{}", rarity)
    }
}

impl Rarity {
    /// Returns the rarity enum value according to the rarity id
    /// 
    /// ## Arguments:
    /// * id - the rarity id
    pub fn from_id(rarity: &i16) -> Rarity {
        match rarity {
            0 => Rarity::COMMON,
            1 => Rarity::UNCOMMON,
            2 => Rarity::SUPER,
            3 => Rarity::EXTREME,
            4 => Rarity::ULTRA,
            5 => Rarity::KAMI,
            _ => Rarity::COMMON,
        }
    }
}
