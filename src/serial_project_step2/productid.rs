use crate::{GenSerialData, InputData};

pub struct ProductID {
    pub data: InputData, // 공통 데이터는 InputData 구조체로 묶어서 관리
                         // 개별 데이터는 구조체의 필드로 선언
}

impl ProductID {
    pub fn new(digit: usize) -> Self {
        ProductID {
            data: InputData {
                name: "ProductID".to_owned(),
                digit,
                rawdata: None,
            },
        }
    }
}

impl GenSerialData for ProductID {
    fn return_input_data(&mut self) -> &mut InputData {
        &mut self.data
    }
}
