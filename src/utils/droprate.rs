
pub enum DropRate {
    COMMON,
    UNCOMMON,
    SUPER,
    EXTREME,
    ORIGINS,
}

impl DropRate {
    pub fn value(&self) -> f32 {
        match &self {
            DropRate::COMMON => 100.0,
            DropRate::UNCOMMON => 50.0,
            DropRate::SUPER => 16.5,
            DropRate::EXTREME => 5.5,
            DropRate::ORIGINS => 2.2,
        }
    }
}
