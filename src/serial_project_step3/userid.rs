use crate::util::get_user_input;
use crate::GenSerialData;
pub struct UserID {
    pub digit: u32,
    pub id: Option<String>,
}

impl UserID {
    pub fn new() -> Self {
        UserID { digit: 0, id: None }
    }
}

impl GenSerialData for UserID {
    fn get_input(&mut self) {
        println!("Please input {}-digits User ID: ", self.digit);
        self.id = Some(get_user_input());
    }

    fn generate(&self) -> Option<&str> {
        self.id.as_ref().map(|x| x.as_str())
    }
}
