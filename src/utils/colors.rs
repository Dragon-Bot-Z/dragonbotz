
pub enum Colors {
    DEFAULT,
}

impl Colors {
    pub fn value(&self) -> u32 {
        match &self {
            Colors::DEFAULT => 0xfb8019,
        }
    }
}
