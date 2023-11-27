pub enum Data {
    Hotels,
    Reviews,
}

impl Data {
    pub fn copy(&self) -> Data {
        match self {
            Data::Hotels => Data::Hotels,
            Data::Reviews => Data::Reviews,
        }
    }
}