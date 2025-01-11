use crate::InputData;
use crate::{get_user_input, GenSerialData};

pub struct UserID {
    pub digit: i32,
    pub id: Option<String>,
}

impl UserID {
    pub fn new(digit: i32) -> InputData {
        InputData {
            name: "UserID".to_owned(),
            digit,
            id: None,
        }
    }
}

impl GenSerialData for UserID {
    fn get_input(&mut self) {
        println!("Please input {}-digits User ID: ", self.digit);
        self.id = Some(get_user_input());
    }

    fn get_data(&self) -> Option<&str> {
        self.id.as_ref().map(|x| x.as_str())
    }
}
