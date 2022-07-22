
// crate
use crate::utils::icons::Icons;


pub enum Items {
    BaseSummonTicket,
}

impl std::fmt::Display for Items {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let content = match &self {
            Items::BaseSummonTicket => format!("{} **Base summon ticket**", Icons::BaseSummonTicket),
        };

        write!(f, "{}", content)
    }
}
