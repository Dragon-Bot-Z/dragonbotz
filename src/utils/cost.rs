
pub enum Cost {
    SummonBase,
}

impl Cost {
    /// Returns the cost value
    pub fn value(&self) -> i64 {
        match &self {
            Cost::SummonBase => 1,
            _ => 0,
        }
    }
}
