
// crate
use crate::utils::icons::Icons;


pub enum Items {
    SummonTicketBase,
}

impl std::fmt::Display for Items {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let content = match &self {
            Items::SummonTicketBase => format!("{} **Base summon ticket**", Icons::SummonTicketBase),
        };

        write!(f, "{}", content)
    }
}
