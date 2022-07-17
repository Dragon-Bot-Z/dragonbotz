
// crate
use crate::utils::icons::Icons;


pub enum Items {
    SUMMON_TICKET_BASIC,
}

impl std::fmt::Display for Items {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let content = match &self {
            Items::SUMMON_TICKET_BASIC => format!("{} **Basic summon ticket**", Icons::SUMMON_TICKET_BASIC),
        };

        write!(f, "{}", content)
    }
}
