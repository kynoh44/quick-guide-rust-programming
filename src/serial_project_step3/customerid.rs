use crate::{GenSerialData, InputData};

pub struct CustomerID {
    pub data: InputData, // 공통 데이터는 InputData 구조체로 묶어서 관리
                         // 개별 데이터는 구조체의 필드로 선언
}

impl CustomerID {
    pub fn new(digit: usize) -> Self {
        CustomerID {
            data: InputData {
                name: "UserID".to_owned(),
                digit,
                id: None,
            },
        }
    }
}

impl GenSerialData for CustomerID {
    fn return_input_data(&mut self) -> &mut InputData {
        &mut self.data
    }
}
