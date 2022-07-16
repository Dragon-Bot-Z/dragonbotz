
pub struct BannerModel {
    id: i32,
    name: String,
}

impl BannerModel {

    /// Returns an instance of BannerModel
    /// 
    /// ## Arguments:
    /// * id - the banner id
    /// * name - the banner name
    pub fn new(id: i32, name: String) -> Self {
        Self {
            id,
            name,
        }
    }

    /// Returns the banner id
    pub fn id(&self) -> &i32 {
        &self.id
    }

    /// Returns the banner name
    pub fn name(&self) -> &String {
        &self.name
    }

}
