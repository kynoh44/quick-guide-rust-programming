# 시리얼번호 만들기 프로젝트

이전에 스마트 포인터 Box를 소개하면서 잠깐 시리얼 키를 생성하는 예제를 만들었습니다. 프로그램 사용자는 사용자 ID와 제품 ID를 입력해서 단순히 두 문자열을 합친 시리얼 키를 생성하는 예제였습니다.

```rust
// src/smart_pointer_application/main.rs
use std::io::{stdin, stdout, Write};

fn get_user_input() -> String {
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
    fn get_input(&mut self);
    fn generate(&self) -> Option<&str>;
}

struct UserID {
    digit: u32,
    id: Option<String>,
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

struct ProductID {
    digit: u32,
    id: Option<String>,
}

impl GenSerialData for ProductID {
    fn get_input(&mut self) {
        println!("Please input {}-digits Product ID: ", self.digit);
        self.id = Some(get_user_input());
    }

    fn generate(&self) -> Option<&str> {
        self.id.as_ref().map(|x| x.as_str())
    }
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

fn main() {
    println!("hello");

    let userid = UserID { digit: 4, id: None };
    let productid = ProductID { digit: 8, id: None };
    let mut items: Vec<Box<dyn GenSerialData>> = vec![Box::new(userid), Box::new(productid)];

    collect_data(&mut items);
    let serial = generate_serial(&items);
    println!("Serial generated: {}", serial);
}
```

```bash
$ cargo run --bin smart_pointer_application
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/smart_pointer_application`
hello
Please input 4-digits User ID: 
1234
Please input 8-digits Product ID: 
qwerasdf
Serial generated: 1234qwerasdf
```

4개의 숫자로 된 사용자번호와 8개의 숫자로 된 제품 번호를 입력하면 그냥 두개의 번호를 붙여서 시리얼 번호로 알려줍니다. 이 예제는 사실상 시리얼 키를 생성하는 기능은 전혀 없고 단순히 2가지 입력 데이터를 받기 위한 Box와 트레이트 객체를 사용하는 방법만 소개하는 예제였습니다. 이번 장에서는 이 예제를 좀 더 개선해서 조금은 더 진짜 시리얼 키를 생성하는 실제 제품처럼 만들어보려고합니다.

시리얼 키라는건 특정 사용자가 특정 제품을 구매했다는 인증을 해주는 것이라고 생각합니다. 시리얼 키는 암호화되어있고, 암호를 풀어서 사용자 정보와 제품 정보가 옳바르면 제품 설치를 완료하면 될것 같습니다. 생각해보면 크게 2개의 프로그램이 필요하겠네요. 시리얼 키를 생성하는 프로그램과 키가 옳바른 키인지 인증하는 프로그램이 필요합니다. 시리얼 키의 암호를 풀기 위한 키는 프로그램에 내장하면 되겠습니다.

그리고 더 고민해보면 시리얼 키를 생성하는데 들어가는 데이터도 더 많아지거나 달라질 수 있을것 같습니다. 회사에 제품이 1개만 있는 것도 아니고, 제품마다 판매 방식이 달라질 것이니까요. 단순히 제품의 ID만 입력할 게 아니라, 계약 만기 날짜나 고객의 국가 번호 등등 모든 시리얼 키게 동일한 정보가 들어가는게 아닐 수도 있을것 같습니다. 결국 영업부서가 편리하게 사용하려면 여러가지 입력 데이터에 대한 처리 코드를 미리 만들어넣고, 필요에 따라 입력 데이터를 고를 수 있도록 옵션을 처리할 수 있게 만들어야합니다.

큰 규모의 프로젝트가 되지는 않겠지만, 이 책에서 소개한 러스트의 기능들을 활용하는 예제 정도는 될것 같습니다.
