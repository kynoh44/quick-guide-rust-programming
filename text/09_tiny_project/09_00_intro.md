# 시리얼번호 만들기 프로젝트

## 프로젝트 시작하기

이전에 스마트 포인터 Box를 소개하면서 잠깐 시리얼 키를 생성하는 예제를 만들었습니다. 프로그램 사용자는 사용자 ID와 제품 ID를 입력해서 단순히 두 문자열을 합친 시리얼 키를 생성하는 예제였습니다. 트레이트를 만들어서 사용자 ID와 제품 ID를 위한 공통 인터페이스를 만들어서 사용했었습니다. 왜 그런 공통 인터페이스가 필요한지, 그리고 트레이트 객체를 만들어서 각 입력을 관리하는 이유가 뭔지 등을 설명하겠습니다.

가장 단순하게 시작을 해보겠습니다. 만약에 팀장님이나 다른 동료에게 시리얼 키를 만드는 프로그램을 만들어달라고 부탁을 받으면 어떻게 시작하셨겠나요? 저라면 어떤 입력 데이터를 가지고 시리얼 키를 만들면 되는지 물어보겠습니다. 사용자ID와 제품ID가 있다는 말을 들으면 아마 다음과 같이 만들었을것 같습니다.

```rust
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

fn main() {
    println!("Please input 4-digits User ID: ");
    let userid = Some(get_user_input());

    println!("Please input 8-digits Product ID: ");
    let productid = Some(get_user_input());

    let plain_serial = format!("{}{}", userid.unwrap(), productid.unwrap());
    println!("Plain serial: {}", plain_serial); // 암호화 전 시리얼 출력

    let verify_userid = &plain_serial[0..4];
    let verify_productid = &plain_serial[4..12];
    println!("Verify User ID: {}", verify_userid);
    println!("Verify Product ID: {}", verify_productid);
}
```

```bash
$ cargo run --bin serial_project_step1
   Compiling my-rust-book v0.1.0 (/home/gkim/study/quick-guide-rust-programming)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.33s
     Running `target/debug/serial_project_step1`
Please input 4-digits User ID: 
1234
Please input 8-digits Product ID: 
qwerasdf
Plain serial: 1234qwerasdf
Verify User ID: 1234
Verify Product ID: qwerasdf
```

사용자 ID가 무엇인지, 제품 ID가 무엇인지를 조사했을 것입니다. 그래서 각각 4글자, 8글자의 문자로된 데이터라는 것을 알았으면 위와 같이 프로그램 사용자에게 4개의 숫자로 된 사용자번호와 8개의 숫자로 된 제품 번호를 입력을 받습니다. 그리고 두개의 번호를 붙여서 시리얼 번호를 만들어서 알려줍니다. 이 예제는 사실상 시리얼 키를 생성하는 기능은 전혀 없고 단순히 2가지 입력 데이터를 받기 위한 예제였습니다만 어쨌든 대략 이렇게 시작했을 것입니다. 이정도로 만들어서 잘 동작하는 것을 확인하고, 이 프로그램을 사용할 사용자에게 이정도로 동작하는 프로그램이면 어떠냐고 물어보고, 나름 요구사항등을 수집하려는 시도를 했을 것입니다.

아마도 프로그램 사용자는 이정도면 쓸 수 있겠다 암호화만 해달라고 리뷰를 해줄 것입니다. 그런데요 우리가 일을 하다보면 항상 겪는 일이지만, 동료에게 리뷰를 받고 계속 개발을 하다보면 어떤 일이 벌어지나요? 항상, 예 정말 말 그대로 항상 요구사항은 변합니다. 예전에 이정도면 충분하다고 리뷰를 해주지 않았냐고 하소연을 해봐도 거의 항상 요구사항은 변하고, 변하는 것만이 아니라 요구사항이 늘어나고 프로그램은 복잡해집니다. 아마 조금이라도 그런 경험을 해보신 분이라면 지금 저 프로그램은 전혀 요구사항 변화를 수용할 수 없다는 것을 이해하실 것입니다.
