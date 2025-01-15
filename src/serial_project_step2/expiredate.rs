use crate::get_user_input;
use crate::{GenSerialData, InputData};

pub struct ExpireDate {
    pub data: InputData, // 공통 데이터는 InputData 구조체로 묶어서 관리
    year: u32,
    month: u32,
    day: u32,
}

impl ExpireDate {
    pub fn new(digit: usize) -> Self {
        ExpireDate {
            data: InputData {
                name: "ExpireDate".to_owned(),
                digit,
                rawdata: None,
            },
            year: 0,
            month: 0,
            day: 0,
        }
    }
}

impl GenSerialData for ExpireDate {
    fn return_input_data(&mut self) -> &mut InputData {
        &mut self.data
    }

    fn get_input_from_user(&mut self) {
        println!("Please input the expiration date (YYYY/MM/DD) : ",);
        let rawdata = get_user_input();

        // 입력받은 날짜를 분리해서 year, month, day 필드에 저장
        // 동시에 year, month, day 필드에 저장된 값이 올바른지 검증
        let date: Vec<&str> = rawdata.split('/').collect();
        self.year = date[0].parse().unwrap();
        assert!(self.year >= 2021, "The year must be 2021 or later.");
        self.month = date[1].parse().unwrap();
        assert!(
            self.month >= 1 && self.month <= 12,
            "The month must be between 1 and 12."
        );
        self.day = date[2].parse().unwrap();
        assert!(
            self.day >= 1 && self.day <= 31,
            "The day must be between 1 and 31."
        );

        let inputdata = self.return_input_data();
        inputdata.rawdata = Some(rawdata);
    }
}
