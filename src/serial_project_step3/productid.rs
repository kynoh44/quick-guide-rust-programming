use crate::util::get_user_input;
use crate::GenSerialData;

pub struct ProductID {
    pub digit: u32,
    pub id: Option<String>,
}

impl ProductID {
    pub fn new() -> Self {
        ProductID { digit: 0, id: None }
    }
}

impl GenSerialData for ProductID {
    fn get_input(&mut self) {
        println!("Please input {}-digits Product ID: ", self.digit);
        self.id = Some(get_user_input());
    }

    fn generate(&self) -> Option<&str> {
        self.id.as_ref().map(|x| x.as_str())
    }
}
