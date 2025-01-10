mod productid;
mod userid;
mod util;

use magic_crypt::{new_magic_crypt, MagicCryptTrait};
use productid::ProductID;
use userid::UserID;

trait GenSerialData {
    fn get_input(&mut self);
    fn generate(&self) -> Option<&str>;
}

fn collect_data(items: &mut Vec<Box<dyn GenSerialData>>) {
    for item in items.iter_mut() {
        item.get_input();
    }
}

fn generate_serial(items: &Vec<Box<dyn GenSerialData>>) -> String {
    let mut data = String::new();
    for item in items.iter() {
        data.push_str(item.generate().unwrap());
    }
    data
}

//fn register_input_data(item: &Box<dyn GenSerialData>) {}

fn main() {
    let productid = ProductID::new();
    let userid = UserID::new();
    let mut items: Vec<Box<dyn GenSerialData>> = vec![Box::new(userid), Box::new(productid)];

    collect_data(&mut items);
    let serial = generate_serial(&items);
    println!("Plain serial: {}", serial);

    let mc = new_magic_crypt!("magickey", 256); // AES256 알고리즘을 사용하는 MagicCrypt256타입의 객체 생성
    let base64 = mc.encrypt_str_to_base64(&serial); // 암호화 후 BASE64로 인코딩
    println!("Encrypted serial: {}", base64);

    let dec = mc.decrypt_base64_to_string(base64).unwrap(); // BASE64로 인코딩된 데이터를 디코딩 후 암호 해제
    println!("Decrypted serial: {}", dec);
}
