use crate::get_user_input;
use crate::GenSerialData;

#[derive(Clone)]
enum CustomerKind {
    Business,
    Individual,
    Company,
}

impl From<CustomerKind> for usize {
    fn from(item: CustomerKind) -> usize {
        match item {
            CustomerKind::Business => 1,
            CustomerKind::Individual => 2,
            CustomerKind::Company => 3,
        }
    }
}

pub struct CustomerType {
    customer_type: Option<CustomerKind>,
    digit: usize,
    name: String,
}

impl CustomerType {
    pub fn new() -> Self {
        CustomerType {
            name: "CustomerType".to_owned(),
            digit: 1,
            customer_type: None,
        }
    }
}

impl GenSerialData for CustomerType {
    fn get_input_from_user(&mut self) {
        let input: String;

        println!("Please input customer type: 1-Business, 2-Individual, 3-Company");
        input = get_user_input();
        assert_eq!(input.len(), self.get_length());
        self.put_rawdata(input);
    }

    fn get_length(&mut self) -> usize {
        self.digit
    }

    fn get_rawdata(&self) -> String {
        if let Some(kind) = &self.customer_type {
            return format!("{}", usize::from((*kind).clone()));
        } else {
            return "0".to_owned();
        }
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn put_rawdata(&mut self, data: String) {
        let kind = match data.as_str() {
            "1" => CustomerKind::Business,
            "2" => CustomerKind::Individual,
            "3" => CustomerKind::Company,
            _ => CustomerKind::Business,
        };
        self.customer_type = Some(kind);
    }
}
