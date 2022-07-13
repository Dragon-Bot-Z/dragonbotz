
pub enum Rarity {
    COMMON,
    UNCOMMON,
    SUPER,
    EXTREME,
    ULTRA,
    KAMI,
}

impl std::fmt::Display for Rarity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rarity = match &self {
            Rarity::COMMON => "<:rarity_common:996897790334599198> **Common**",
            Rarity::UNCOMMON => "<:rarity_uncommon:996897775348351057> **Uncommon**",
            Rarity::SUPER => "<:rarity_super:996897783225270323> **Super**",
            Rarity::EXTREME => "<:rarity_extreme:996897788543631370> **Extreme",
            Rarity::ULTRA => "<:rarity_ultra:996897780314419291> **Ultra**",
            Rarity::KAMI => "<:rarity_kami:996897785532141618> **Kami**",
        };

        write!(f, "{}", rarity)
    }
}
