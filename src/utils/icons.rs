
// crate
use crate::utils::rarity::Rarity;


pub enum Icons {
    // others
    UNKNOWN,

    // items
    SummonTicketBase,

    // rarity
    COMMON,
    UNCOMMON,
    SUPER,
    EXTREME,
    ULTRA,
    KAMI,
    ORIGINS,
}

impl std::fmt::Display for Icons {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let content = match &self {
            // others
            Icons::UNKNOWN => "<:unknown:998265915437240390>",

            // rarity
            Icons::COMMON => "<:rarity_common:996897790334599198>",
            Icons::UNCOMMON => "<:rarity_uncommon:996897775348351057>",
            Icons::SUPER => "<:rarity_super:996897783225270323>",
            Icons::EXTREME => "<:rarity_extreme:996897788543631370>",
            Icons::ULTRA => "<:rarity_ultra:996897780314419291>",
            Icons::KAMI => "<:rarity_kami:996897785532141618>",
            Icons::ORIGINS => "<:rarity_origins:998240766700814376>",

            _ => "<:unknown:998265915437240390>",
        };

        write!(f, "{}", content)
    }
}

impl Icons {
    pub fn from_rarity(rarity: &Rarity) -> Icons {
        match rarity {
            Rarity::COMMON => Icons::COMMON,
            Rarity::UNCOMMON => Icons::UNCOMMON,
            Rarity::SUPER => Icons::SUPER,
            Rarity::EXTREME => Icons::EXTREME,
            Rarity::ULTRA => Icons::ULTRA,
            Rarity::KAMI => Icons::KAMI,
            _ => Icons::UNKNOWN,
        }
    }
}
