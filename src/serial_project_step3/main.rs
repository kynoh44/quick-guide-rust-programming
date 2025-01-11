mod productid;
mod userid;

use magic_crypt::{new_magic_crypt, MagicCryptTrait};
use productid::ProductID;
use std::io::{stdin, stdout, Write};
use userid::UserID;

pub struct InputData {
    pub name: String,
    pub digit: usize,
    pub id: Option<String>,
}

trait GenSerialData {
    fn get_input_from_user(&mut self) {
        let inputdata = self.return_input_data();
        println!(
            "Please input {}-digits for {}: ",
            inputdata.digit, inputdata.name
        );
        inputdata.id = Some(get_user_input());
    }

    fn get_data_from_struct(&mut self) -> Option<&str> {
        let inputdata = self.return_input_data();
        inputdata.id.as_ref().map(|x| x.as_str())
    }

    fn get_length(&mut self) -> usize {
        self.return_input_data().digit
    }

    fn return_input_data(&mut self) -> &mut InputData;
}

pub fn get_user_input() -> String {
    let mut s = String::new();
    let _ = stdout().flush();
    stdin()
        .read_line(&mut s)
        .expect("Did not enter a correct string");
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }
    s
}

fn collect_data(items: &mut Vec<Box<dyn GenSerialData>>) {
    for item in items.iter_mut() {
        item.get_input_from_user();
    }
}

fn generate_serial(items: &mut Vec<Box<dyn GenSerialData>>) -> String {
    let mut data = String::new();
    for item in items.iter_mut() {
        data.push_str(item.get_data_from_struct().unwrap());
    }
    data
}

fn main() {
    let productid = ProductID::new(8);
    let userid = UserID::new(4);
    let mut items: Vec<Box<dyn GenSerialData>> = vec![Box::new(userid), Box::new(productid)];

    collect_data(&mut items);
    let plain_serial = generate_serial(&mut items);
    println!("Plain serial: {}", plain_serial);

    let mc = new_magic_crypt!("magickey", 256); // AES256 알고리즘을 사용하는 MagicCrypt256타입의 객체 생성
    let serial = mc.encrypt_str_to_base64(&plain_serial); // 암호화 후 BASE64로 인코딩
    println!("Encrypted serial: {}", serial);

    let dec = mc.decrypt_base64_to_string(serial).unwrap(); // BASE64로 인코딩된 데이터를 디코딩 후 암호 해제
    println!("Decrypted serial: {}", dec);

    let userid_len = items[0].get_length();
    let productid_len = items[1].get_length();
    let verify_userid = &dec[0..userid_len];
    let verify_productid = &dec[userid_len..userid_len + productid_len];
    println!("Verify User ID: {}", verify_userid);
    println!("Verify Product ID: {}", verify_productid);
}
