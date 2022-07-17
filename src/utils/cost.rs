
pub enum Cost {
    SUMMON_BASIC,
}

impl Cost {
    /// Returns the cost value
    pub fn value(&self) -> i64 {
        match &self {
            Cost::SUMMON_BASIC => 1,
            _ => 0,
        }
    }
}
