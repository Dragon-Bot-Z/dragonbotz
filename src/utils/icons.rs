
// lib
    // serenity
use serenity::model::id::EmojiId;

// crate
use crate::utils::rarity::Rarity;


pub enum Icons {
    // others
    UNKNOWN,

    // items
    BaseSummonTicket,

    // buttons
    CLOSE,
    ArrowLeftEnd,
    ArrowLeft,
    ArrowRight,
    ArrowRightEnd,

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

            // icons
            Icons::BaseSummonTicket => "<:base_summon_ticket:999418886988505088>",

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

    pub fn emoji_id(&self) -> EmojiId {
        match &self {
            // others
            Icons::UNKNOWN => EmojiId(998265915437240390),

            // items
            Icons::BaseSummonTicket => EmojiId(999418886988505088),

            // buttons
            Icons::CLOSE => EmojiId(1001875591215206531),
            Icons::ArrowLeftEnd => EmojiId(1001908289602334842),
            Icons::ArrowLeft => EmojiId(1001908287236755516),
            Icons::ArrowRight => EmojiId(1001908291271675955),
            Icons::ArrowRightEnd => EmojiId(1001908292928417802),

            // rarity
            Icons::COMMON => EmojiId(996897790334599198),
            Icons::UNCOMMON => EmojiId(996897775348351057),
            Icons::SUPER => EmojiId(996897783225270323),
            Icons::EXTREME => EmojiId(996897788543631370),
            Icons::ULTRA => EmojiId(996897780314419291),
            Icons::KAMI => EmojiId(996897785532141618),
            Icons::ORIGINS => EmojiId(998240766700814376),

            _ => EmojiId(998265915437240390)
        }
    }
}
