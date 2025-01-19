mod customerid;
mod customertype;
mod expiredate;
mod productid;

use clap::{Arg, Command};
use customerid::CustomerID;
use customertype::CustomerType;
use magic_crypt::{new_magic_crypt, MagicCryptTrait};
use productid::ProductID;
use std::io::{stdin, stdout, Write};

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

trait GenSerialData {
    fn verify(&self, data: &str) -> bool {
        self.get_length() == data.len() && self.get_rawdata() == data
    }

    fn get_length(&self) -> usize;
    fn get_rawdata(&self) -> String;
    fn get_name(&self) -> &str;
    fn put_rawdata(&mut self, data: &str);
    // 명령행 인자 처리를 위해 추가된 함수들
    fn get_arg_name(&self) -> &str;
    fn get_help(&self) -> String;
    fn get_mandatory(&self) -> bool;
}

///
/// $ serial --help
/// serial 0.1.0
/// Serial number generator
/// --productid <productid> Product ID
/// --customerid <customerid> Customer ID
/// --customertype <customertype> Customer Type
/// --expiredate <expiredate> Expire Date
/// --help Prints help information
///
/// $ serial --productid qwerasdf --customerid 12344 --customertype 1 --expiredate 20221231
///

fn main() {
    let productid = ProductID::new(8);
    let customerid = CustomerID::new(4);
    let customertype = CustomerType::new();
    let expiredate = expiredate::ExpireDate::new();
    let mut items: Vec<Box<dyn GenSerialData>> = vec![
        Box::new(customerid),
        Box::new(productid),
        Box::new(customertype),
        Box::new(expiredate),
    ];
    let plain_serial: String = String::new();

    // 더 이상 사용자에게 입력을 받지 않고 명령행 인자로 처리
    let mut command = Command::new("serial")
        .version("0.1.0")
        .about("Serial number generator");

    for item in items.iter() {
        // Cargo.toml에서도 Clap의 string feature를 사용하고 있기 때문에 String을 넣어줘야 함
        // Arg의 new, long, help, required 함수에 String을 넣어주기 위해 to_owned()를 사용
        // 만약에 Clap의 string feature를 사용하지 않는다면 &str을 넣어주면 됨
        // 그런데 &str은 item.get_name()이 &'static str이 아니기 때문에 to_owned()를 사용해야 함
        // 예를 들어 curstomerid 객체의 name은 String이다. 그런데 get_name()으로 name의 레퍼런스를 받아서
        // Arg의 long 함수에 넣어주게되면, Arg가 customerid에 대한 레퍼런스를 가지게 된다.
        // 그러면 Arg와 customerid는 서로 다른 라이프타임을 가지게 되어서 컴파일 에러가 발생한다.
        // 개발자는 Customerid가 arg보다 더 오래 존재한다고 생각하지만, 사실상 Rust 컴파일러가 customerid와 arg 중에
        // 어느 것을 먼저 해지할지는 알 수 없다. 따라서 이러한 의존성으로 생기는 라이프타임 문제를 해결하기 위해서는
        // Arg에 레퍼런스를 넣어주는 것이 아니라 String 객체를 넣어주어서 라이프타임에 대한 의존성을 없애야만한다.
        command = command.arg(
            Arg::new(item.get_name().to_owned())
                .long(item.get_arg_name().to_owned())
                .help(item.get_help().to_owned())
                .required(item.get_mandatory().to_owned()),
        );
    }

    let matches = command.get_matches();

    for item in items.iter_mut() {
        if let Some(data) = matches.get_one::<String>(item.get_name()) {
            item.put_rawdata(data.as_str());
        }
    }

    println!("Plain serial: {}", plain_serial);

    let mc = new_magic_crypt!("magickey", 256); // AES256 알고리즘을 사용하는 MagicCrypt256타입의 객체 생성
    let serial = mc.encrypt_str_to_base64(&plain_serial); // 암호화 후 BASE64로 인코딩
    println!("Encrypted serial: {}", serial);

    let dec = mc.decrypt_base64_to_string(serial).unwrap(); // BASE64로 인코딩된 데이터를 디코딩 후 암호 해제
    println!("Decrypted serial: {}", dec);

    let mut offset = 0;
    for item in items.iter() {
        let len = item.get_length();
        let rawdata = &dec[offset..offset + len];
        println!("Verify {}: {}", item.get_name(), rawdata);
        println!("Verify result: {}", item.verify(rawdata));
        offset += len;
    }
}
