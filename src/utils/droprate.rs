
pub enum DropRate {
    UNCOMMON,
    SUPER,
    EXTREME,
    ORIGINS,
}

impl DropRate {
    pub fn value(&self) -> f32 {
        match &self {
            DropRate::UNCOMMON => 50.0,
            DropRate::SUPER => 16.5,
            DropRate::EXTREME => 5.5,
            DropRate::ORIGINS => 2.2,
        }
    }
}
