#[allow(dead_code)]
pub struct Budget {
    pub id: i32,
    pub category: String,
    pub subcategory: String,
    pub start_date: String,
    pub end_date: String,
    pub frequency: String,
    pub cost: f32,
}

impl Budget {
    pub fn get_end_date(&self) -> String{
        let edate = &self.end_date;
        return edate.to_string()
    }

    pub fn get_start_date(&self) -> String{
        let sdate = &self.start_date;
        return sdate.to_string()
    }
}