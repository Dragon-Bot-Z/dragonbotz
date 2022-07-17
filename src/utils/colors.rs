
// crate
use crate::utils::rarity::Rarity;


pub enum Colors {
    // others
    DEFAULT,

    // rarity
    COMMON,
    UNCOMMON,
    SUPER,
    EXTREME,
    ULTRA,
    KAMI,
}

impl Colors {
    /// Returns the color value
    pub fn value(&self) -> u32 {
        match &self {
            Colors::DEFAULT => 0xfb8019,

            // rarity
            Colors::COMMON => 0x242424,
            Colors::UNCOMMON => 0x33903a,
            Colors::SUPER => 0x0aabe1,
            Colors::EXTREME => 0xd11a1a,
            Colors::ULTRA => 0xf1e24c,
            Colors::KAMI => 0x9362da,
        }
    }

    /// Returns the color value of the rarity
    /// 
    /// ## Arguments:
    /// * rarity - the rarity
    pub fn from_rarity(rarity: &Rarity) -> u32 {
        let color = match rarity {
            Rarity::COMMON => Colors::COMMON,
            Rarity::UNCOMMON => Colors::UNCOMMON,
            Rarity::SUPER => Colors::SUPER,
            Rarity::EXTREME => Colors::EXTREME,
            Rarity::ULTRA => Colors::ULTRA,
            Rarity::KAMI => Colors::KAMI,
            _ => Colors::DEFAULT,
        };

        color.value()
    }
}
