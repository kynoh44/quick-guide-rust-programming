use crate::get_user_input;
use crate::GenSerialData;

pub struct ExpireDate {
    year: u32,
    month: u32,
    day: u32,
    name: String,
}

impl ExpireDate {
    pub fn new(digit: usize) -> Self {
        ExpireDate {
            name: "ExpireDate".to_owned(),
            year: 0,
            month: 0,
            day: 0,
        }
    }
}

impl GenSerialData for ExpireDate {
    fn get_input_from_user(&mut self) {
        println!("Please input the expiration date (YYYY/MM/DD) (e.g. 2025/01/23) : ",);
        let rawdata = get_user_input();
        assert_eq!(rawdata.len(), 10); // 입력받은 데이터의 길이가 10인지 검증, YYYYMMDD에 /가 2개 들어가므로 10개임

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
    }

    fn verify(&mut self, data: &str) -> bool {
        let year = data[0..4].parse().unwrap();
        let month = data[4..6].parse().unwrap();
        let day = data[6..8].parse().unwrap();

        self.year == year && self.month == month && self.day == day
    }

    fn get_length(&mut self) -> usize {
        8
    }

    fn get_rawdata(&self) -> String {
        format!("{:04}{:02}{:02}", self.year, self.month, self.day)
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }
}
