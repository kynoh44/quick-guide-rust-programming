## 시리얼 번호 암호화

가장 먼저 시리얼 키를 암호화해서 좀 더 제대로된 시리얼 키같이 보이도록 바꿔보겠습니다.

이 예제에서 보여드리려는 것은 Cargo에 어떻게 새로운 크레이트(다른 언어에서 라이브러리라고 부르는)을 추가하고 사용하는지 입니다. 가장 쉬운 방법은 아무래도 구글에서 검색해보는 것입니다. "rust encryption"이라고 검색해보면 많은 자료들이 나올 것입니다. 지금 제가 이 글을 쓰고 있는 2025년 1월 8일에 검색해본 결과 첫번째 글은 러스트 언어 홈페이지에 있는 토론 글입니다. 왜 러스트에서 문자열을 암호화하는게 어려운지에 대한 글입니다. 그리고 2번째가 MagicCrypt 메뉴얼 사이트 https://docs.rs/magic-crypt/latest/magic_crypt/ 입니다. 3번째는 Openssl의 encrypt 모듈에 대한 메뉴얼 사이트 https://docs.rs/openssl/latest/openssl/encrypt/index.html 가 나오고 있습니다. 그 외에도 여러가지 크레이트들이 검색됩니다.

일단 가장 먼저 검색되는 2개의 크레이트를 조사해서 어느 것을 사용할지 결정해보겠습니다. 가장 먼저 https://crates.io/ 사이트에서 해당 크레이트가 얼마나 오래동안 개발이 되고 현재도 개발이 활발하게 진행되고 있는지, 안정적인 버전을 출시했는지, 사용자가 얼마나 많은지 등을 확인해보는게 좋습니다. 우선 magic_crypt를 검색해서 https://crates.io/crates/magic-crypt 페이지를 열어보겠습니다. 페이지의 왼쪽에는 크레이트의 소개와 사용법등을 보여주고, 오른쪽에는 가장 최신 버전이 언제 출시되었는지, 라이선스는 무엇인지, 그리고 Cargo를 이용해서 어떻게 설치할 수 있는지 등을 보여줍니다. Github에 레포 주소도 있습니다. 그리고 페이지 아래쪽을 보면 "Stats Overview"라는 섹션에 있고 현재 391,263번 다운로드가 되었다고 알려줍니다. 총 31개의 릴리즈가 있었다고도 보여주네요. 제가 crates.io 사이트에서 crypt나 encrypt로 검색해보면 대부분의 크레이트들의 다운로드 횟수가 몇만번이 안되는 것을 보면 MagicCrypt가 많이 사용되는 크레이트라는 것을 알 수 있었습니다.crates.io에서 openssl도 검색해보면 당연하게도 다운로드 횟수가 훨씬 높습니다. 1억번이 넘네요. 당연한 결과겠지요.

저는 다운로드 횟수 다음으로 예제 코드를 확인해봅니다. Openssl의 encryp 모듈의 예제 코드와 MagicCrypt의 예제 코드를 보면 확연하게 MagicCrypt의 예제 코드가 단순하다는 것을 알 수 있습니다. 기능이 풍부한 크레이트는 사용법이 복잡한 경우가 많고, 기능이 단순한 크레이트가 사용법이 쉬운 경우가 많습니다. 저는 보통 시작할 때는 나한테 꼭 필요한 기능만 있다면 사용법이 쉬운 크레이트를 먼저 선택합니다. 개발을 하면서 내가 어떤 기능이 필요한가를 더 잘 이해하게되고, 사용중인 크레이트에 부족한 기능일 때 다른 크레이트로 변경하는 것을 선호합니다. 현재 저는 암호화에 대해서 별로 아는 것도 없으니 선택할 옵션이 없습니다. 그냥 어떻게든 암호화만 되면 충분합니다. 나중에 개발을 하면서 암호화에 대해 제가 더 잘 이해하게되고, 그러면서 더 세세한 옵션 선택을 할 수 있는 상황이 되면 좀 더 기능이 많은 크레이트를 사용할 수 있겠지요. 결국 지금은 시작 단계이니 Openssl보다 더 단순하고 직관적인 MagicCrypt를 사용하겠습니다.

첫번째로 할 일은 Cargo를 써서 MagicCrypt 크레이트를 설치하는 것입니다. https://crates.io/crates/magic-crypt 에서 2가지 설치 방법을 알려주고 있습니다. 첫번째는 `cargo add magic-crypt` 명령을 실행하는 것입니다. 이 명령은 cargo가 알아서 가장 최신 버전을 확인해서 Cargo.toml에 추가해줍니다. 두번째 방법은 개발자가 직접 Cargo.toml 파일에 `magic-crypt = "4.0.1"`를 입력하는 방법입니다. 최신 버전이 아니라 이전 버전을 사용해야될 때는 직접 Cargo.toml을 수정하는게 편리합니다. 우리는 특정 버전이 필요한 상황이 아니므로 cargo 툴을 사용해서 설치하겠습니다.

```bash
$ cargo add magic-crypt
    Updating crates.io index
      Adding magic-crypt v4.0.1 to dependencies
             Features:
             + std
    Updating crates.io index
    Blocking waiting for file lock on package cache
     Locking 22 packages to latest compatible versions
      Adding aes v0.8.4
      Adding base64 v0.22.1
      Adding block-buffer v0.10.4
      Adding block-padding v0.3.3
      Adding cbc v0.1.2
      Adding cfg-if v1.0.0
      Adding cipher v0.4.4
      Adding cpufeatures v0.2.16
      Adding crc-any v2.5.0
      Adding crypto-common v0.1.6
      Adding debug-helper v0.3.13
      Adding des v0.8.1
      Adding digest v0.10.7
      Adding generic-array v0.14.7 (latest: v1.1.1)
      Adding inout v0.1.3
      Adding libc v0.2.169
      Adding magic-crypt v4.0.1
      Adding md-5 v0.10.6
      Adding sha2 v0.10.8
      Adding tiger v0.2.1
      Adding typenum v1.17.0
      Adding version_check v0.9.5
```

그럼 Cargo.toml가 어떻게 바꼈는지를 보겠습니다.

```bash
$ git diff Cargo.toml
diff --git a/Cargo.toml b/Cargo.toml
index 5272213..2e4f18d 100644
--- a/Cargo.toml
+++ b/Cargo.toml
@@ -6,6 +6,7 @@ edition = "2021"
 # See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

 [dependencies]
+magic-crypt = "4.0.1"
```

dependencies 섹션에 magic-crypt가 추가된 것을 볼 수 있습니다. 크레이트의 설치가 되었으니 이제 소스 코드에서 크레이트를 사용할 수 있습니다.

============================================ 2025 01 08 ==================


```rust
// src/project_step1/main.rs
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

