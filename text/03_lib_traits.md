# 표준 라이브러리와 트레이트

러스트의 std 패키지에 있는 표준 라이브러리와 트레이트 중 몇가지 자주 사용되는 것들을 소개하겠습니다. 

## 벡터 타입 std::vec::Vec

러스트로 프로그래밍을 할 때 가장 많이 사용하는 라이브러리가 벡터 타입이라고 생각합니다. 말 그대로 같은 타입의 데이터들을 배열같이 저장하는 것인데, 크기에 제한이 없고 아주 많은 메소드들을 지원하고 있어서 데이터를 저장할 때 가장 많이 사용하는 타입입니다.

문자열을 나타내는 String도 사실은 u8타입 데이터를 벡터에 저장한 것입니다. (https://doc.rust-lang.org/src/alloc/string.rs.html#365) 

---

어떤 타입이나 트레이트가 실제로 어떻게 정의되어있는지 알면 좀더 사용하기 편리합니다. 온라인 매뉴얼을 통해 쉽게 확인해볼 수 있는 방법이 있습니다.

String타입의 매뉴얼 https://doc.rust-lang.org/std/vec/struct.Vec.html 을 열어보시면 페이지 오른쪽 위에 소스를 볼 수 있는 링크 source가 있습니다.

<img src="lib_trait_vec1.png">

source를 클릭하면 해당 타입의 소스 코드로 넘어갑니다.

특정 메소드의 코드를 보고 싶다면 마찬가지로 매뉴얼에서 해당 메소드에 대한 항목을 보면 source 버튼이 있습니다.

<img src="lib_trait_vec2.png">

---

벡터는 사실 거의 모든 언어마다 다 있는 것이니 굳이 길게 설명하지않고 러스트에서 자주 사용하는 패턴을 이용한 예제를 보겠습니다.

```rust
use std::vec::Vec;

#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    published: u32,
}

fn find_rust<'a>(books: &'a Vec<Book>) -> Vec<&'a Book> {
    let mut found: Vec<&Book> = Vec::new();
    for b in books.iter() {
        if b.title.contains("Rust") {
            found.push(b);
        }
    }
    found
}

fn main() {
    let rust_book = Book {
        title: String::from("The Rust Programming Language"),
        author: String::from("Steve Klabnik and Carol Nichols"),
        published: 20230228,
    };

    let rust_in_action = Book {
        title: String::from("Rust in Action"),
        author: String::from("Tim McNamara"),
        published: 20210810,
    };

    let another = Book {
        title: String::from("The another book"),
        author: String::from("Unknown"),
        published: 20111111,
    };

    let mut library: Vec<Book> = Vec::new();
    library.push(rust_book);
    library.push(rust_in_action);
    library.push(another);

    let rust_books = find_rust(&library);
    let mut only_titles: Vec<String> = Vec::new();

    if rust_books.is_empty() {
        println!("Cannot find any Rust book");
    } else {
        for b in rust_books.into_iter() {
            let mut title = b.title.clone();
            title.push('\n');
            only_titles.push(title);
        }
    }

    let collect = only_titles.into_iter().collect::<String>();
    println!("{}", collect);
}
```

벡터에 데이터를 추가하는 메소드는 push()입니다. 다음과 같이 vec이라는 매크로를 사용해서 변수 선언과 데이터 추가를 동시에 할 수도 있습니다.

```rust
let mut library = vec![rust_book, rust_in_action, another];
```

여기에 러스트 언어의 특성상 주의할 것이 있습니다. push()메소드이 인자를 보면 벡터에 저장할 객체의 참조를 사용하는게 아니라 값을 전달합니다. 바로 여기에서 러스트 언어의 소유권 이동이 발생합니다. 아래와같이 벡터에 저장한 객체를 다시 사용하려고 해보면 “value borrowed here after move” 에러가 발생하는 것을 알 수 있습니다.

```rust
let mut library: Vec<Book> = Vec::new();
library.push(rust_book);
library.push(rust_in_action);
library.push(another);
println!("{:?}", rust_book);
```

library에 rust_book 객체의 소유권이 옮겨진 이후로 rust_book 객체에 접근할 수 없습니다.

그리고 find_rust 함수를 호출합니다. 함수 인자로는 벡터의 참조를 전달합니다. books.iter()는 books 벡터에 저장된 각 Book타입 객체의 참조 포인터를 반환합니다. 따라서 b는 &Book타입입니다. 결국 find_rust 함수가 하는 일은 books에 저장된 각 책의 제목중에 Rust라는 단어가있는 책들의 참조 포인터만 골라서 found 벡터에 저장하는 것입니다.

이제 rust_books라는 벡터에는 책 제목에 Rust가 들어간 책들의 참조 포인터가 저장되었습니다. 그리고 그 다음 for 루프에서는 그렇게 찾은 책들의 제목만 only_titles라는 벡터로 저장합니다. 여기에 책 제목의 String타입의 참조 포인터를 저장할 수도 있습니다만 나중에 책 제목을 출력할 때 한 줄에 하나씩 출력하기위해 ‘\n’을 추가해야하므로 참조가 아니라 String 객체를 저장했습니다.

마지막으로 only_titles벡터의 이터레이터에 collect() 메소드를 호출합니다. 그러면 각 String 타입 객체들이 합쳐져서 하나의 String 타입 객체가 됩니다. 프로그램의 출력은 하나의 String 객체가 됩니다.

```rust
% cargo run
The Rust Programming Language
Rust in Action

%
```

제가 이 예제에서 보여드리고자하는 벡터의 사용패턴은

1. iter()/into_iter()를 사용하여 각 데이터에 접근하기
2. 원본 벡터안의 객체중 일부를 다른 벡터로 저장할 때 참조 포인터와 라이프타임을 사용하기
3. 벡터안의 객체들을 합치기위해 이터레이터와 collect() 메소드 사용하기

그 외에 자주 사용하는 메소드들을 보자면

- len: 현재 벡터에 몇개의 데이터가 있는지 알려줍니다.
- &[index]: []를 사용해서 index 위치의 객체에 접근할 수 있습니다. index는 usize 타입만 허용됩니다. 그냥 library[index]같은 형태로 객체에 접근하려고하면 소유권 이동이 발생하므로 허용되지 않습니다. 그래서 보통 &library[index]같이 특정 위치의 객체에 대한 참조 포인터를 얻을 때 사용합니다.
- push/pop: 벡터의 마지막에 데이터를 추가하거나 빼는 메소드입니다. 스택과 같다고 생각하면됩니다.
- insert/remove: 특정 위치에 데이터를 넣거나 빼는 메소드입니다. 벡터 내부의 데이터를 이동해야되므로 상황에 따라 실행속도가 느려질 수 있습니다.
- as_ptr/as_mut_ptr: 데이터 배열의 포인터를 얻습니다. 만약 C/C++ 코드와 같이 사용하게된다면 이 메소드를 자주 사용하게 될 것입니다

그리고 벡터를 생성할때 Vec::from() 메소드를 사용하기도하지만 vec! 매크로를 사용하는 경우가 더 많습니다.

```rust
fn main() {
    let library = vec![
        Book {
            title: String::from("The Rust Programming Language"),
            author: String::from("Steve Klabnik and Carol Nichols"),
            published: 20230228,
        },
        Book {
            title: String::from("Rust in Action"),
            author: String::from("Tim McNamara"),
            published: 20210810,
        },
        Book {
            title: String::from("The another book"),
            author: String::from("Unknown"),
            published: 20111111,
        },
    ];
......
```

마지막으로 한가지 더 팁을 드리자면 벡터의 참조 포인터를 함수로 전달하면 슬라이스가 됩니다. find_rust함수를 다음과 같이 슬라이스를 전달받도록 만들 수도 있습니다. 이렇게 만들면 벡터뿐 아니라 배열을 처리하는 함수로도 만들 수 있기 때문에 좀더 유연한 코드를 만들 수 있습니다.

```rust
fn find_rust<'a>(books: &'a [Book]) -> Vec<&'a Book> {
    let mut found: Vec<&Book> = Vec::new();
    for b in books.iter() {
        if b.title.contains("Rust") {
            found.push(b);
        }
    }
    found
}
```

### 연습문제

- Vec타입의 소스 코드를 확인해보세요. 매뉴얼에 소스 코드를 볼 수 있는 링크가 있습니다. 생각보다 단순하게 구현된 타입인데 어떻게 그렇게나 많은 메소드를 지원할 수 있을까요? 데이터 크기를 늘릴 수 있는 배열이라는 컨셉의 장단점을 조사해보세요. 데이터구조를 공부하는데 중요한 배경지식이 될 것입니다.
- https://doc.rust-lang.org/std/vec/struct.Vec.html#guarantees 벡터의 내부 구조에 대해서 더 자세히 이해하고싶다면 이 문서를 참고하세요.

## 객체의 내부 데이터를 출력하는 트레이트 std::fmt::{Display, Debug}

객체를 터미널에 출력하는 방법을 이야기해보겠습니다. 특히 디버깅할 때 가장 많이 사용할 것입니다. 그리고 사용자에게 데이터를 보여주는 인터렉티브한 어플리케이션을 만들다보면 객체의 값을 터미널에 출력해야될 일이 자주 생깁니다. 

객체를 출력하기위해서 이미 우리가 여러번 사용해본게 있습니다.  구조체에  derive(Debug)구문을 추가해서 디버그 메세지를 출력하는걸 해봤었습니다. 

```rust
#[derive(Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

fn main() {
    let dot: Point = Point { x: 1.2, y: 3.4 };
    println!("{:?}", dot);
}
```

Debug라는 trait의 전체 경로를 확인해보면 std::fmt::Debug라는 trait입니다. derive라는 구문은 사전적으로는 유래하다, 파생하다라는 뜻을 가지고있는데 Point라는 구조체의 데이터를 출력하기 위한 코드를 자동으로 생성해달라는 의미가 됩니다.

그렇다면 자동으로 생성하지않고 내가 직접 만들어서 쓸 수도 있다는 것이겠지요. 다음과 우리가 이전에 trait의 구현체를 만들어본것 처럼 다음과 같이 직접 구현해볼 수 있습니다. std::fmt::Debug라는 trait가 있다는것 알기때문에 다음과 같이 시작할 수는 있겠는데 어느 메소드를 구현해야하는지를 어떻게 확인할 수 있을까요?

```rust
impl fmt::Debug for Point {

}
```

당연히 해당 trait의 메뉴얼을 확인해야겠지요.

https://doc.rust-lang.org/std/fmt/trait.Debug.html

이게 Debug trait의 메뉴얼입니다. 가장 윗부분을 보면 트레이트의 정의를 확인할 수 있습니다.

```rust
pub trait Debug {
    // Required method
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>;
}
```

“Required method”라고 써놓은 것을 보니 fmt라는 함수를 구현하면 되겠네요.

그런데 사실 구현을 막상 하려고해도 f라는 인자를 어떻게 써야할지 정의만 봐서는 모를 수밖에 없습니다. 매뉴얼을 좀더 읽어보면 답이 나옵니다.

```rust
impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Point [{} {}]", self.x, self.y)
    }
}
```

Examples라는 섹션에 위와 같이 구현할 수 있다는 예제를 보여주고 있습니다. Formatter라는 구조체가 뭔지 뭔지 write라는 매크로가 뭔지 따져보기전에 일단 한번 만들어보겠습니다.

```rust
use std::fmt;

pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "POINT [{} {}]", self.x, self.y)
    }
}

fn main() {
    let dot: Point = Point { x: 1.2, y: 3.4 };
    println!("{:?}", dot);
}
```

이제 코드를 실행해보면 아래와같이 우리가 지정한 형태로 객체의 값이 출력됩니다.

```rust
POINT [1.2 3.4]
```

그런데 derive(Debug)구문을 사용하면 자동으로 만들어주는 것을 굳이 우리가 직접 만들 필요는 없겠지요. 대신에 다음과 같이 디버깅 메세지를 출력하는게 아니라 정식으로 사용자에게 특정 구조체의 값들을 보여줄 필요가 있을때가 있습니다.

```rust

fn main() {
    let dot: Point = Point { x: 1.2, y: 3.4 };
    println!("{}", right_dot);
}
```

단순히 내가 만든 데이터 객체의 값을 출력할 때도 있겠지만, 내가 만든 라이브러리를 다른 개발자가 가져다쓸 때, 객체의 값을 출력해볼 수도 있는 것입니다.

예를 들어 내가 Point라는 라이브러리를 만들어서 좌표상의 점에 대한 처리를 하는 기능들을 구현했다면, 이 라이브러리의 사용자는 다음과 같이 어플리케이션을 만들 것입니다.

```rust
fn main() {
    let first_dot: Point = get_user();
    let second_dot: Point = get_user();
    println!("The distance between {} and {} is {}", 
              first_dot, second_dot, get_distance(first_dot, second_dot));
}
```

사용자가 출력하려는 메세지는 디버깅 메세지가 아닌 정식으로 터미널의 stdout에 출력하려는 메세지입니다. 이런 경우를 위해서 Debug trait가 아닌 std::fmt::Display trait가 존재합니다. 그런데 사실 내부 구현은 Debug 와 완전히 동일합니다. 단지 println!등에서 사용하는 출력 형태가 {:?}가 아니라 {}라는 차이 뿐입니다.

```rust
use std::fmt;

pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "POINT({} {})", self.x, self.y)
    }
}

fn main() {
    let dot: Point = Point { x: 1.2, y: 3.4 };
    println!("{}", right_dot);
}
```

내부 구현은 동일하다해도 사용자가 사용하는 경우가 다르므로 어느쪽을 구현할 것인지 잘 판단해야합니다. 개발단계에서만 값을 출력해볼 것인지, 최종 사용자에게도 값이 노출되어야할 클래스인지, 둘다인지 잘 판단해서 사용해야합니다.

## 두 객체가 같은지 비교하는 트레이트 PartialEq와 Eq

아주 간단하게 아래와 같이 사용자에게 한 문자로된 명령을 입력받는 프로그램을 만들어보겠습니다.  

```rust
use std::io;

pub enum Command {
    Help,
    Quit,
    Run,
}

fn user_input() -> Result<Command, String> {
    println!("input h/q/r: ");
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => match input.as_str().strip_suffix("\n") {
            Some("h") => return Ok(Command::Help),
            Some("q") => return Ok(Command::Quit),
            Some("r") => return Ok(Command::Run),
            _ => return Err(format!("Wrong input: {input}")),
        },
        Err(error) => return Err(format!("Wrong input: {error}")),
    }
}

fn main() {
    let com: Command = user_input().expect("Wrong input");

    match com {
        Command::Help => println!("show help message"),
        Command::Quit => return,
        Command::Run => println!("do something"),
    }
}
```

프로그램을 실행하면 h/q/r 중에 하나를 입력하라는 메세지를 출력합니다. 만약 사용자가 r을 입력하고 엔터를 치면 “do something”이라는 메세지를 출력합니다.

```rust
% cargo run
input h/q/r: 
r
do something
```

실제 프로젝트라면 “do something”을 출력하게 아니라 실제로 뭔가를 실행하는 코드가 있을 것입니다.

그리고 여기에 한가지 기능을 더 추가하는 과정을 생각해보겠습니다.

user_input함수는 사용자에게 e라는 명령을 입력받으면 Command::Execute(추가 메세지)를 반환합니다. 그리고 main함수는 거기에 맞는 처리를 실행합니다.

```rust
use std::io;

pub enum Command {
    Help,
    Quit,
    Execute(String),
    Run,
}

fn user_input() -> Result<Command, String> {
    println!("input h/q/r/e: ");
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => match input.as_str().strip_suffix("\n") {
            Some("h") => return Ok(Command::Help),
            Some("q") => return Ok(Command::Quit),
            Some("r") => return Ok(Command::Run),
            Some("e") => return Ok(Command::Execute(format!("debug"))),
            _ => return Err(format!("Wrong input: {input}")),
        },
        Err(error) => return Err(format!("Wrong input: {error}")),
    }
}

fn main() {
    let com: Command = user_input().expect("Wrong input");
    let p = String::new();

    assert_ne!(com, Command::Execute(p));
    match com {
        Command::Help => println!("show help message"),
        Command::Quit => return,
        Command::Run => println!("do something"),
        Command::Execute(path) => println!("execute {path}"),
    }
}
```

그런데 아직 모든 기능이 완료된게 아니라서 main함수에서 assert_ne! 매크로를 사용해서 Command::Execute 입력을 받으면 프로그램이 중단되도록 하겠습니다. 추후에 Command::Execute명령에 따라 실행될 코드를 먼저 구현한 후에 main함수에서 실행되도록 만들 계획입니다.

보통 작업중인 코드가 실행되지 않도록 할때 assert나 assert_eq, assert_ne등의 매크로를 많이 사용합니다. 다른 언어에서도 자주 사용하는 기능입니다. 그런데 위 코드를 빌드해보면 이상한 에러가 발생합니다.

```rust
% cargo build 
   Compiling ex v0.1.0 (/Users/user/ex)
error[E0369]: binary operation `==` cannot be applied to type `Command`
  --> src/main.rs:29:5
   |
29 |     assert_ne!(com, Command::Execute(p));
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |     |
   |     Command
   |     Command
   |
note: an implementation of `PartialEq` might be missing for `Command`
  --> src/main.rs:3:1
   |
3  | pub enum Command {
   | ^^^^^^^^^^^^^^^^ must implement `PartialEq`
   = note: this error originates in the macro `assert_ne` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider annotating `Command` with `#[derive(PartialEq)]`
   |
3  + #[derive(PartialEq)]
4  | pub enum Command {
   |

error[E0277]: `Command` doesn't implement `Debug`
  --> src/main.rs:29:5
   |
29 |     assert_ne!(com, Command::Execute(p));
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `Command` cannot be formatted using `{:?}`
   |
   = help: the trait `Debug` is not implemented for `Command`
   = note: add `#[derive(Debug)]` to `Command` or manually `impl Debug for Command`
   = note: this error originates in the macro `assert_ne` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider annotating `Command` with `#[derive(Debug)]`
   |
3  + #[derive(Debug)]
4  | pub enum Command {
   |

Some errors have detailed explanations: E0277, E0369.
For more information about an error, try `rustc --explain E0277`.
error: could not compile `ex` (bin "ex") due to 3 previous errors
```

가장 먼저 발생한 에러 메세지를 보면 assert_ne! 매크로가 실행될 때 ‘==’ 연산자를 이용해서 두개의 값이 같은지를 확인하는데, Command 타입은 ‘==’ 연산자를 실행할 수 없다는 말을 하고 있습니다. 그리고 친절하게도 Command타입에 PartialEq 트레이트를 구현하는게 필요하다는 설명을 해주면서 #[derive(PartialEq)]를 타입에 추가하는 것을 추천해주고 있습니다.

그 다음에는 마찬가지로 assert_ne! 매크로가 Debug 트레이트를 사용하고 있으니 PartialEq와 같이 #[derive(Debug)]를 추가해주라는 안내 메세지가 출력되었습니다.

일단 컴파일러가 알려주는대로 Debug와 PartialEq의 구현을 추가해보겠습니다.

```rust
use std::io;

#[derive(Debug, PartialEq)]
pub enum Command {
    Help,
    Quit,
    Execute(String),
    Run,
}

fn user_input() -> Result<Command, String> {
    println!("input h/q/r/e: ");
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => match input.as_str().strip_suffix("\n") {
            Some("h") => return Ok(Command::Help),
            Some("q") => return Ok(Command::Quit),
            Some("r") => return Ok(Command::Run),
            Some("e") => return Ok(Command::Execute(format!("asdf"))),
            _ => return Err(format!("Wrong input: {input}")),
        },
        Err(error) => return Err(format!("Wrong input: {error}")),
    }
}

fn main() {
    let com: Command = user_input().expect("Wrong input");
    let p = String::new();

    assert_ne!(com, Command::Execute(p));
    match com {
        Command::Help => println!("show help message"),
        Command::Quit => return,
        Command::Run => println!("do something"),
        Command::Execute(path) => println!("execute {path}"),
    }
}
```

Debug와 PartialEq 트레이트를 구현해주었더니 문제없이 빌드되고 있습니다.

Debug트레이트는 이전에 사용해봤는데 PartialEq 트레이트를 무었일까요? 그리고 두 객체를 비교한다고하면 Eq나 Equal같은 이름이 아니라 왜 Partial 일부분만 비교한다는 이름이 붙었을까요?

가장 먼저 해야할 일은 PartialEq의 매뉴얼을 보는 것입니다.

https://doc.rust-lang.org/std/cmp/trait.PartialEq.html

매뉴얼에 있는 예제 코드를 보면 Debug 트레이트와 마찬가지로 직접 구현해줄 수도 있고, derive 구문을 사용해서 자동으로 생성할 수도 있다는 것을 알 수 있습니다. 그리고 두 객체를 비교하는 트레이트는 PartialEq와 Eq 두가지가 있다는 것을 알 수 있습니다.

그럼 왜 PartialEq와 Eq 두가지 트레이트가 필요할까요? 단순히 매뉴얼에 있는 정의만 읽어보면 PartialEq는 partial-equivalence 관계(https://en.wikipedia.org/wiki/Partial_equivalence_relation)를 확인하는 것이고 Eq는 equivalence 관계(https://en.wikipedia.org/wiki/Equivalence_relation)를 확인하는 것이라고 말하고 있습니다. 그런데 둘다 수학적인 내용들이라 사실 의미를 알기가 어렵습니다.

사실 일반적으로 값을 비교할 때는 PartialEq를 쓰는 것이 맞습니다. 값을 비교할 수 없는 예외적인 케이스들이 있기 때문에 PartialEq라는 이름을 사용한다고 합니다. 그런 예외적인 케이스들 중에 매뉴얼에서 소개하는 것이 부동소수점 연산에서 뭔가 연산이 잘못되었음을 나타내는 std::f32::NAN이라는 값을 비교할때 PartialEq를 사용할 수 없다는 것입니다.

assert_eq매크로를 이용해서 std::f32::NAN을 비교해보면 보기에는 서로 같으므로 문제가 없을것 같지만 패닉을 발생시킵니다.

```rust
fn main() {
    assert_eq!(std::f32::NAN, std::f32::NAN);
}
```

수학적으로 두 숫자 아님을 비교한다는 것이 정의되지 않았기 때문에 두 값이 같지 않다고 판단하도록 러스트 언어를 만든 것입니다. 파이썬등 대부분의 언어들도 마찬가지입니다.

그럼 일반적으로 값을 비교할 때 PartialEq를 사용하는 것으로 생각한다면 굳이 Eq 트레이트를 사용해야만 하는 경우는 무엇을까요? 바로 구조체를 HashMap의 키 값으로 사용할 때 입니다.

```rust
#[derive(PartialEq, Eq, Hash)]
struct MyKey {
    x: i32,
    y: i32,
}

struct MyVal {
    distance_from_co_origin: f32,
}

fn main() {
    let mut map = HashMap::new();
    map.insert(
        MyKey { x: 3, y: 4 },
        MyVal {
            distance_from_co_origin: 5.0,
        },
    );
}
```

MyKey는 점의 좌표를 나타냅니다. MyVal은 원점으로부터 점의 거리를 나타냅니다. 그래서 여러 점들의 원점으로부터 거리를 관리하는 해시맵을 만들어봤습니다. 만약 MyKey 구조체에 PartialEq와 Eq 트레이트의 구현이 없다면 빌드가 안됩니다.

Eq 트레이트를 사용하기 위해서는 PartialEq도 같이 구현되어야합니다. 그리고 HashMap에서 키 값으로 사용된다는 표시를 하기위해 Hash 트레이트도 구현합니다.

## 두 객체를 비교하는 std::cmp::Ordering타입과 std::cmp::Ord트레이트

두 객체가 동일한지를 판단하는 std::cmp::PartialEq 트레이트를 알아봤으니 이번에는 크기를 비교하는 std::cmp::PartialOrd 트레이트에 대해서 알아보겠습니다.

그리고 PartialOrd 트레이트를 사용하기 위해 한가지 더 새롭게 알아야할 것이 있습니다. std::cmp::PartialOrd라는 트레이트의 partial_cmp 메소드의 반환값이 std::cmp::Ordering이라는 타입입니다. Ordering은 enum 타입으로 Greater, Less, Equal이라는 3가지 타입을 가지고 있습니다.

그럼 간단하게 두 객체를 비교하는 예제를 보겠습니다.

```rust
use std::cmp::Ordering;

#[derive(PartialEq, Eq)]
struct Person {
    name: String,
    age: i32,
    height: i32,
}

impl PartialOrd for Person {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.height <= 0 || other.height <= 0 {
            return None;
        }

        if self.height > other.height {
            Some(Ordering::Greater)
        } else if self.height < other.height {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Equal)
        }
    }
}

fn main() {
    let alpha = Person {
        name: "alpha".to_owned(),
        age: 10,
        height: 110,
    };
    let beta = Person {
        name: "beta".to_owned(),
        age: 10,
        height: 100,
    };

    if alpha > beta {
        println!("{} is taller.", alpha.name);
    }
    if alpha.partial_cmp(&beta).unwrap() == Ordering::Greater {
        println!("{} is taller.", alpha.name);
    }
}
```

우리가 구현해야할 것은 PartialOrd 트레이트의 partial_cmp 메소드입니다. 이 메소드는 Option<Ordering>을 반환합니다. 왜 Option을 사용하는지는 예제 코드를 보면 알 수 있습니다. height 값이 0이거나 음수인 경우, 아직 객체가 제대로 초기화되지 않았거나 버그로 인해 객체의 데이터가 깨졌을 수 있습니다. 그렇게 단순히 비교만 하는게 아니라 비교할 수 없는 상황이 있을 수도 있기 때문에 그런 에러 상황을 처리하기위해 Option을 반환하도록 했습니다. Rust언어가 얼마나 에러 처리에 철저한지 알 수 있습니다.

에러 처리 이후에는 실제로 비교해야할 데이터를 비교합니다. 메소드를 호출하는 객체가 더 크다면 Ordering::Greater를 반환합니다. 작으면 Ordering::Less, 같으면 Ordering::Equal을 반환합니다.

main함수에서는 객체를 비교할 수 있는 2가지 방법을 보여주고 있습니다. 첫번째는 직관적으로 ‘>’ 연산자를 사용하는 것입니다. 당연히 PartialOrd 트레이트가 구현되었기 때문에 ‘>’와 ‘<’ 연산자를 사용하는 것이 가능합니다. 그리고 ‘==’ 연산자는 PartialEq 트레이트가 구현되었기 때문에 사용할 수 있습니다.

그 다음에는 메소드를 직접 호출하는 것을 보여줍니다. 직관적이지는 않지만 연산의 결과값을 변수에 저장하거나, 다른 함수에 전달해야할 때 ‘<’같은 연산자를 사용할 수 없으므로 이렇게 메소드를 직접 호출해서 결과값을 저장해야합니다.

```rust
let compare = alpha.partial_cmp(&beta);
who_is_taller(alpha, beta, compare);
```

두 객체를 비교할 수 있게 되었습니다. 비교하면 생각나는게 정렬 알고리즘이지요. 한반에 있는 학생들을 키에 따라 정렬하는 예제를 만들어보았습니다.

```rust
use std::cmp::Ordering;

#[derive(PartialEq, Eq)]
struct Person {
    name: String,
    age: i32,
    height: i32,
}

impl PartialOrd for Person {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.height <= 0 || other.height <= 0 {
            return None;
        }

        if self.height > other.height {
            Some(Ordering::Greater)
        } else if self.height < other.height {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Equal)
        }
    }
}

fn class_sorting(people: &mut Vec<Person>) {
    let len = people.len();

    for i in 0..(len - 1) {
        for j in (i + 1)..len {
            if people[i] > people[j] {
                people.swap(i, j);
            }
        }
    }
}

fn main() {
    let mut class: Vec<Person> = vec![
        Person {
            name: "aaa".to_owned(),
            age: 10,
            height: 110,
        },
        Person {
            name: "bbb".to_owned(),
            age: 10,
            height: 100,
        },
        Person {
            name: "ccc".to_owned(),
            age: 10,
            height: 120,
        },
        Person {
            name: "ddd".to_owned(),
            age: 10,
            height: 90,
        },
    ];

    class_sorting(&mut class);

    for p in class.iter() {
        println!("{} is {}", p.name, p.height);
    }
}
```

그리고 모든 프로그래밍 언어의 라이브러리가 그렇듯이 Rust의 벡터 타입에도 정렬을 위한 메소드가 있습니다. 이때 벡터의 각 데이터들을 비교하는 클로저를 전달해야하므로 이때는 ‘>’같은 산술 연산자가 아니라 partial_cmp 메소드를 호출하는 클로저를 사용하게됩니다.

```rust
use std::cmp::Ordering;

#[derive(PartialEq, Eq)]
struct Person {
    name: String,
    age: i32,
    height: i32,
}

impl PartialOrd for Person {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.height <= 0 || other.height <= 0 {
            return None;
        }

        if self.height > other.height {
            Some(Ordering::Greater)
        } else if self.height < other.height {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Equal)
        }
    }
}

fn main() {
    let mut class: Vec<Person> = vec![
        Person {
            name: "aaa".to_owned(),
            age: 10,
            height: 110,
        },
        Person {
            name: "bbb".to_owned(),
            age: 10,
            height: 100,
        },
        Person {
            name: "ccc".to_owned(),
            age: 10,
            height: 120,
        },
        Person {
            name: "ddd".to_owned(),
            age: 10,
            height: 90,
        },
    ];

    class.sort_by(|a, b| a.partial_cmp(b).unwrap());

    for p in class.iter() {
        println!("{} is {}", p.name, p.height);
    }
}
```

## Iterators 반복자

지금까지 이터레이터를 여러번 사용해봤는데 사실 이터레이트는 트레이트로 구현됩니다. 러스트의 기본 데이터 타입에는 이터레이터가 미리 구현되어있어서 우리가 직접 구현할 필요가 없었습니다. 하지만 우리가 직접 만든 타입에 이터레이터를 사용하기 위해서는 우리가 직접 구현해야겠지요.

이번 장에서는 내가 만든 구조체 타입에 이터레이터를 사용할 수 있도록 만들어보겠습니다.  

아래 소스는 이전에 fmt::Display 트레이트를 구현해본 예제입니다.

```rust
#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    published: u32,
}

fn main() {
    let book = Book {
        title: String::from("The Rust Programming Language"),
        author: String::from("Steve Klabnik and Carol Nichols"),
        published: 20230228,
    };

    println!("{}", book);
}
```

이 예제에서 우리가 만든 Book 타입에 이터레이트를 적용해보겠습니다.

우선 이터레이터가 어떤 트레이트인지 러스트 메뉴얼을 찾아보겠습니다.

```rust
pub trait Iterator {
    type Item;

    // Required method
    fn next(&mut self) -> Option<Self::Item>;

    // Provided methods
....skip...
}
```

https://doc.rust-lang.org/std/iter/trait.Iterator.html

메뉴얼에 따르면 필수로 구현해야할 next라는 메소드가 1개있고, next메소드를 구현하면 자동으로 제공되는 메소드가 74개있다고 합니다. 그렇게 총 75개의 메소드를 가지는 트레이트입니다. next메소드는 자기 자신을 mutable 참조로 받아서 Option에 감싼 결과 값 하나를 반환하게 됩니다. 우리가 해야할 일은 Item이라는 타입에 무엇을 쓸지 정해야합니다. 보통은 자신이 구현한 구조체나 구조체의 참조가 되겠지요. 그리고 주의해야할 것이 이터레이터가 현재 몇개의 값을 반환했는지, 그리고 더 이상 반환할 값이 있는지 없는지 등의 상태 관리를 해야된다는 것입니다. 그렇게 이터레이터가 가진 상태가 변하기때문에 &mut self라는 타입을 사용하는 것입니다.

그럼 Book 타입의 객체들을 처리하기위한 BookSelf라는 이터레이터 타입을 구현해보겠습니다.

```rust
#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    published: u32,
}

#[derive(Debug)]
struct BookShelf<'a> {
    books: &'a [Book],
}

impl<'a> Iterator for BookShelf<'a> {
    type Item = &'a Book;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some((first, remains)) = self.books.split_first() {
            self.books = remains;
            Some(first)
        } else {
            None
        }
    }
}

fn main() {
    let mut book_array = [
        Book {
            title: String::from("The Fellowship of the Ring"),
            author: String::from("J. R. R. Tolkien"),
            published: 19540729,
        },
        Book {
            title: String::from("The Two Towers"),
            author: String::from("J. R. R. Tolkien"),
            published: 19541111,
        },
        Book {
            title: String::from("The Return of the King"),
            author: String::from("J. R. R. Tolkien"),
            published: 19551020,
        },
    ];

    let mut mybooks_iter = BookShelf {
        books: &mut book_array,
    };
    println!("{:?}", mybooks_iter.next());
    println!("{:?}", mybooks_iter.next());
    println!("{:?}", mybooks_iter.next());
    println!("{:?}", mybooks_iter.next());

    let mybooks_for = BookShelf {
        books: &mut book_array,
    };
    for b in mybooks_for {
        println!("{:?}", b);
    }
}
```

일단 구현이 어떻게할지는 놔두고 main함수에서 어떻게 사용하는지부터 보겠습니다.

book_array라는 이름의 배열을 만들어서 여러개의 Book 객체를 저장합니다. 그리고 mybooks_iter라는 변수를 만드는데 이 변수는 BookSelf타입이고 books라는 필드에 book_array의 참조 포인터를 저장합니다. mybooks_iter라는 변수가 이터레이터가 되는 것입니다. 이제부터 mybooks_iter의 next()라는 메소드를 호출할 때마다 배열에 저장된 Book 타입 객체들이 하나씩 반환되는 것입니다.

또한 이터레이터를 for 루프에도 사용할 수 있습니다. mybooks_for라는 인터레이터를 만들어서 for 루프에 사용하면 next() 메소드를 자동으로 호출해줍니다.

그럼 BookSelf라는 구조체는 어떻게 구현되었는지 볼까요. Book타입 배열의 참조 포인터를 가지고 있습니다. 그냥 배열의 시작 주소를 가지고 있는것 뿐입니다. 이것이 어떻게 이터레이터 역할을 할 수 있는지를 Iterator 트레이트 구현을 확인해야 알 수 있습니다.

Iterator 구현을 보면 가장 먼저 볼 수 있는게 Item 타입은 Book객체의 참조 포인터라는 것입니다. 이것은 이터레이터가 next() 메소드를 호출하거나 for 루프에서 사용될때 반환하는 값의 타입이 Book 객체의 참조 포인터라는 것입니다. 그 다음을 보면 next() 메소드의 구현이 나옵니다. 메소드 인자는 &mut self로 우리가 만든 BookSelf 구조체 mutable 참조가 됩니다. 그리고 반환값인 Option<&Book>이 되겠지요. 구현은 생각보다 간단합니다. 우선 현재 self.books라는 배열을 첫번째 객체와 나머지로 쪼갭니다. 만약 잘 쪼개진다면 어쨌든 배열안에 데이터(Book객체에 대한 포인터)가 1개 이상 들어있다는 뜻이므로 첫번째 객체는 반환하고 나머지는 self.books에 저장합니다. 그런데 배열을 쪼개려고했는데 None이 반환되었다는 것은 self.books 배열에 아무런 데이터가 없다는 뜻이므로 None을 반환해주면 됩니다.

아마 다른 언어로 이터레이터를 만들어본 경험이 있으시다면 쉽게 익숙해질 수 있을거라 생각합니다. 단지 주의해야할 것은 반환값은 Option이므로 더이상 데이터가 없을때 None을 호출해야한다는 것입니다. 그리고 라이프타임을 지정해야되는데 이것은 일단 라이프타임을 지정하지않고 코딩한 후에 컴파일러의 에러 메세지를 참고해서 라이프타임 지정을 추가해주면 됩니다.

### 벡터의 각 요소를 수정할 수 있는 반복자

지금까지는 배열이나 벡터의 각 데이터를 읽기만 하는 예제를 사용했었습니다. 하지만 보통은 아래와 같이 데이터를 수정해야되는 상황이 더 많을 것입니다.

```rust
#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    published: u32,
}

fn main() {
    let book_array = [
        Book {
            title: String::from("The Fellowship of the Ring"),
            author: String::from("J. R. R. Tolkien"),
            published: 19540729,
        },
        Book {
            title: String::from("The Two Towers"),
            author: String::from("J. R. R. Tolkien"),
            published: 19541111,
        },
        Book {
            title: String::from("The Return of the King"),
            author: String::from("J. R. R. Tolkien"),
            published: 19551020,
        },
    ];

    for b in book_array.iter() {
        b.author = String::from("John Ronald Reuel Tolkien");
    }

    println!("{:?}", book_array);
}

```

위 예제를 빌드해보면 아래와 같은 에러가 발생합니다.

```rust
$ cargo build
   Compiling pyalgo v0.1.0 (/home/gurugio/pyalgo)
error[E0594]: cannot assign to `b.author`, which is behind a `&` reference
  --> src/main.rs:28:9
   |
27 |     for b in book_array.iter() {
   |              -----------------
   |              |          |
   |              |          help: use mutable method: `iter_mut()`
   |              this iterator yields `&` references
28 |         b.author = String::from("John Ronald Reuel Tolkien");
   |         ^^^^^^^^ `b` is a `&` reference, so the data it refers to cannot be written

For more information about this error, try `rustc --explain E0594`.
error: could not compile `pyalgo` (bin "pyalgo") due to 1 previous error
```

book_array.iter()는 immutable reference를 반환합니다. 따라서 book_array.iter()에서 반환된 불변 참조가 b에 저장되고, b를 통해 데이터에 접근하면 데이터를 수정할 수 없습니다.

컴파일러가 친절하게 iter() 대신에 iter_mut() 메소드를 사용하라고 알려주고 있습니다. 그리고 한가지 더 잊지 말아야하는게있는데 book_array 선언에 mut를 추가하는 것입니다.


## 해쉬맵 std::collections::HashMap

아마도 Vector만큼이나 많이 사용되는 자료구조를 들자면 해쉬맵HashMap을 꼽을 수 있을 것입니다.

우선 간단하게 사용하는 예제를 보겠습니다.

```rust
use std::collections::HashMap;

#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    published: u32,
}

fn main() {
    let mut library: HashMap<String, Book> = HashMap::new();
    let the_book = Book {
        title: String::from("The Rust Programming Language"),
        author: String::from("Steve Klabnik and Carol Nichols"),
        published: 20230228,
    };
    library.insert("718503105".to_owned(), the_book);
    library.insert(
        "1617294551".to_owned(),
        Book {
            title: String::from("Rust in Action"),
            author: String::from("Tim McNamara"),
            published: 20210810,
        },
    );
    library.insert(
        "0000000000".to_owned(),
        Book {
            title: String::from("The another book"),
            author: String::from("Unknown"),
            published: 20111111,
        },
    );

    let found = library.get("0000000000");
    println!("{:?}", found);
    let not_found = library.get("xxxxxxxxxx");
    println!("{:?}", not_found);
}
```

library변수에 해쉬맵으로 여러권의 책을 저장하고 있습니다. 키는 ISBN값이고 저장값은 Book 타입의 객체입니다. 해쉬맵에 데이터를 저장하기위해서 insert 메소드를 사용하고 키에 해당하는 데이터를 얻기 위해서 get 메소드를 사용합니다.

주의해야 할 것은 insert 메소드에 값을 저장할 때 객체의 값을 전달한다는 것입니다. 만약 the_book 객체를 library에 저장한 후에는 the_book 객체에 대한 소유권이 library로 이동합니다. 따라서 the_book이라는 변수를 다시는 사용할 수 없습니다.

그리고 하나 더 주의할 것이 있는데 insert 메소드는 이미 키가 존재할 경우 데이터를 덮어쓴다는 것입니다. 그러니 실제 제품 개발에서는 항상 데이터가 있는지를 확인하는게 필요합니다. 데이터가 있는지 확인하는 방법은 연습문제로 남겨놓았습니다.

그리고 한가지 예제를 더 보겠습니다. 이전 예제는 키 값이 간단한 스트링 타입이었습니다. 이번 예제는 직접 만든 구조체 타입을 키로 사용하는 예제입니다.

```rust
use std::collections::HashMap;

#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    published: u32,
}

fn main() {
    let mut library: HashMap<Book, String> = HashMap::new();
    library.insert(
        Book {
            title: String::from("The Rust Programming Language"),
            author: String::from("Steve Klabnik and Carol Nichols"),
            published: 20230228,
        },
        "1718503105".to_owned(),
    );
    library.insert(
        Book {
            title: String::from("Rust in Action"),
            author: String::from("Tim McNamara"),
            published: 20210810,
        },
        "1617294551".to_owned(),
    );
    library.insert(
        Book {
            title: String::from("The another book"),
            author: String::from("Unknown"),
            published: 20111111,
        },
        "0000000000".to_owned(),
    );

    let found = library.get(&Book {
        title: String::from("The another book"),
        author: String::from("Unknown"),
        published: 20111111,
    });
    println!("{:?}", found);
}
```

이전 예제와 반대로 책의 정보를 가지고 ISBN값을 찾는 예제입니다. 이 예제를 빌드하면 아래와 같은 에러 메세지를 볼 수 있습니다. 

```rust
error[E0599]: the method `insert` exists for struct `HashMap<Book, String>`, but its trait bounds were not satisfied
  --> src/main.rs:12:13
   |
4  | struct Book {
   | -----------
   | |
   | doesn't satisfy `Book: Eq`
   | doesn't satisfy `Book: Hash`
   | doesn't satisfy `Book: PartialEq`
...
12 |     library.insert(
   |     --------^^^^^^
   |
   = note: the following trait bounds were not satisfied:
           `Book: Eq`
           `Book: PartialEq`
           which is required by `Book: Eq`
           `Book: Hash`
help: consider annotating `Book` with `#[derive(Eq, Hash, PartialEq)]`
   |
4  + #[derive(Eq, Hash, PartialEq)]
5  | struct Book {
   |

error[E0599]: the method `insert` exists for struct `HashMap<Book, String>`, but its trait bounds were not satisfied
  --> src/main.rs:20:13
   |
4  | struct Book {
   | -----------
   | |
   | doesn't satisfy `Book: Eq`
   | doesn't satisfy `Book: Hash`
   | doesn't satisfy `Book: PartialEq`
...
20 |     library.insert(
   |     --------^^^^^^
   |
   = note: the following trait bounds were not satisfied:
           `Book: Eq`
           `Book: PartialEq`
           which is required by `Book: Eq`
           `Book: Hash`
help: consider annotating `Book` with `#[derive(Eq, Hash, PartialEq)]`
   |
4  + #[derive(Eq, Hash, PartialEq)]
5  | struct Book {
   |

error[E0599]: the method `insert` exists for struct `HashMap<Book, String>`, but its trait bounds were not satisfied
  --> src/main.rs:28:13
   |
4  | struct Book {
   | -----------
   | |
   | doesn't satisfy `Book: Eq`
   | doesn't satisfy `Book: Hash`
   | doesn't satisfy `Book: PartialEq`
...
28 |     library.insert(
   |     --------^^^^^^
   |
   = note: the following trait bounds were not satisfied:
           `Book: Eq`
           `Book: PartialEq`
           which is required by `Book: Eq`
           `Book: Hash`
help: consider annotating `Book` with `#[derive(Eq, Hash, PartialEq)]`
   |
4  + #[derive(Eq, Hash, PartialEq)]
5  | struct Book {
   |

error[E0599]: the method `get` exists for struct `HashMap<Book, String>`, but its trait bounds were not satisfied
  --> src/main.rs:37:25
   |
4  | struct Book {
   | -----------
   | |
   | doesn't satisfy `Book: Eq`
   | doesn't satisfy `Book: Hash`
   | doesn't satisfy `Book: PartialEq`
...
37 |     let found = library.get(&Book {
   |                 --------^^^ method cannot be called on `HashMap<Book, String>` due to unsatisfied trait bounds
   |
   = note: the following trait bounds were not satisfied:
           `Book: Eq`
           `Book: PartialEq`
           which is required by `Book: Eq`
           `Book: Hash`
help: consider annotating `Book` with `#[derive(Eq, Hash, PartialEq)]`
   |
4  + #[derive(Eq, Hash, PartialEq)]
5  | struct Book {
   |
```

insert와 get 메소드를 사용하기 위해서 Book 구조체에 Eq, Hash, PartialEq 트레이트의 구현이 있어야한다는 에러 메세지입니다. 에러를 해결하는 것은 컴파일러가 안내하는대로 아래와 같이 각 트레이트의 구현을 추가해주면 됩니다.

```rust
use std::collections::HashMap;

#[derive(Debug, Eq, Hash, PartialEq)]
struct Book {
    title: String,
    author: String,
    published: u32,
}

fn main() {
    let mut library: HashMap<Book, String> = HashMap::new();
    library.insert(
        Book {
            title: String::from("The Rust Programming Language"),
            author: String::from("Steve Klabnik and Carol Nichols"),
            published: 20230228,
        },
        "1718503105".to_owned(),
    );
    library.insert(
        Book {
            title: String::from("Rust in Action"),
            author: String::from("Tim McNamara"),
            published: 20210810,
        },
        "1617294551".to_owned(),
    );
    library.insert(
        Book {
            title: String::from("The another book"),
            author: String::from("Unknown"),
            published: 20111111,
        },
        "0000000000".to_owned(),
    );

    let found = library.get(&Book {
        title: String::from("The another book"),
        author: String::from("Unknown"),
        published: 20111111,
    });
    println!("{:?}", found);
}
```

그럼 왜 이런 에러가 발생하는 것일까요? 키 값으로 우리가 직접 만든 타입을 사용할 때 키 값을 비교할 수가 없기 때문입니다. insert나 get에서 키 값을 비교할 때 두개의 객체를 비교해야하는데, 구조체 타입의 경우 어떻게 두 객체의 값을 비교해야하는지 알지 못합니다. 그래서 PartialEq와 Eq가 모두 구현되어야합니다.

그리고 Hash라는 트레이트의 구현도 필요합니다. hash 트레이트는 해당 타입의 해쉬 Hash 값을 계산하기 위한 트레이트입니다. Book 구조체에는 String 타입 2개의 u32 타입 1개의 데이터가 들어있습니다. Hash 트레이트의 구현을 추가해주면 각 데이터들의 해쉬값을 조합해서 최종 Book 타입 객체의 해쉬값을 계산합니다. 러스트의 해쉬맵 라이브러리가 내부적으로 객체의 해쉬값을 이용해서 데이터를 검색하고 저장하기 때문에, 우리가 직접 만든 타입을 사용해서 키를 지정하고 싶다면 해시값을 생성하는 방법도 해쉬맵 라이브러리에 알려주어야합니다.

물론 각 트레이트의 구현을 직접 구현할 수도 있습니다. 책을 비교할 때 제목, 저자, 출판일을 모두 비교하는게 아니라 제목만 비교할 수도 있으니까요. 저는 해쉬맵의 소개를 위해서 최대한 간단하게 소스를 구현해봤지만, 직접 각 트레이트를 구현해보는 것도 좋은 연습이 될듯합니다.

### 연습문제

- ISBN이 “0000000000”인 책의 제목은 “The another book”입니다. library에서 entry 메소드를 이용해서 이 책의 제목을 “Not released book book”으로 바꾸는 코드를 만들어보세요. entry메소드는 library에 있는 데이터중에 특정한 키의 값이 있는지를 확인하는 메소드이고, Entry라는 Enum 형의 값을 반환합니다. Entry가 무엇인지 확인해보고, 값이 이미 있을때의 반환값과, 값이 없을 때의 반환값이 어떻게 다른지 확인해보세요. 그리고 Entry의 메소드 중에 어떤 메소드를 사용하면 찾는 값이 없을때만 데이터를 넣을 수 있는지 매뉴얼을 검색해보세요.

## 파일 읽고 쓰기위한 std::{fs::File, io::Read, path::PathBuf}

파일을 읽거나 쓰는 것은 거의 모든 프로그램에서 처리하는 일이므로 새로운 언어를 배울 때 반드시 알아봐야하는 것들 중 하나입니다.

러스트에서는 파일을 읽고 쓰기 위해 3가지 module을 알아야합니다. 일단 각각이 무엇인지 간단하게 설명하고 어떻게 사용하는지를 알아보겠습니다.

1. std::fs: 로컬 파일시스템에 있는 파일을 처리하기위한 모듈입니다. 일반적으로 운영체제에 상관없이 사용할 수 있는 기능들을 모아놓은 것입니다. 그 중에서 File 구조체가 일반 파일에 접근할 때 사용됩니다.
2. std::io: 입출력을 위한 타입, 라이브러리, 에러 타입 등을 모아놓은 모듈입니다. Read, Write라는 trait가 있습니다. 주의할 것은 std::io::Read라는 trait가 있다는 것의 의미를 알아야합니다. 트레이트가 있다는 것은 다른 어딘가에 구현체가 있어야 한다는 것입니다. 코드 파일에 use std::io::Read라고 선언을 해서 사용은 하지만 사실 구현체는 다른 곳에 있습니다. 예를 들면 std::fs::File 구조체가 std::io::Read트레이트를 구현하고 있습니다. 아래 예제에서 사용 방법을 보겠습니다. 
3. std::path: 파일을 처리하기 위해서는 파일의 경로를 알아야합니다. 보통 문자열로 사용할 수도 있겠지만, 문자열로 경로를 표현하면 운영체제에 종속적으로 동작할 수 밖에 없습니다. 어느 운영체제나 플랫폼에서도 동작하기 위해서는 파일의 경로를 추상화해야합니다. std::path에 있는 PathBuf와 Path 타입이 파일의 경로를 추상화하는 것들입니다.

이제 이 3가지를 가지고 파일을 읽는 예제를 한번 만들어보겠습니다.

```rust
use std::{
    env::current_dir,
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

/// &Path is immutable form of PathBuf (same to &str and String)
fn grep(filename: &Path, word: &str) {
    let mut f: File = File::open(filename).unwrap();
    let mut text_buffer = String::new();

    f.read_to_string(&mut text_buffer).unwrap();
    for line in text_buffer.split('\n') {
        if line.contains(word) {
            println!("{line}");
        }
    }
}

fn main() {
    let mut filename: PathBuf = current_dir().unwrap();

    filename.push("src/main.rs"); // Be careful not to use  "/src/main.rs"
    grep(&filename, "main"); // show lines that include specific word
}
```

main함수에서 첫번째로 하는 일은 current_dir()함수를 호출해서 프로그램이 실행되는 위치를 알아내는 것입니다. 우리는 “cargo run” 명령으로 프로그램을 실행할 것이므로 현재위치에 “src/main.rs” 경로를 추가하면 main.rs 파일의 위치가 될 것입니다.

한가지 주의해야할 것은 filename의 push() 메소드에 “/src/main.rs”와 같이 절대경로를 전달하면 안된다는 것입니다. push() 메소드는 filename에 저장된 경로에 이어서 하위 경로를 추가하는 일을 하지만, 만약에 전달된 경로가 “/”로 시작하는 절대 경로라면 filename에 저장된 경로를 지우고 전달된 경로로 바꾸게 됩니다. 결국 “/src/main.rs” 파일을 읽으려고하고 에러가 발생할 것입니다.

그리고 grep이라는 함수를 호출합니다. grep 함수의 인자에는 filename의 참조를 전달합니다. 그런데 특이한게 있습니다. PathBuf타입의 변수의 참조를 전달하는데 grep 함수의 인자는 &Path가 되는 것입니다. 주석에도 써놓았듯이 PathBuf는 일반적으로 경로를 추가하거나 바꿀 수 있는 타입입니다. 그리고 참조를 해서 포인터만 절달하게되면 &Path 타입이 됩니다. String과 &str의 관계와 동일합니다. 이렇게 하는 이유는 컴파일의 효율성을 높여서 성능을 높이기 위한 것입니다. PathBuf를 처음 생성해서 PathBuf타입으로 가지고 있을때 최대한 모든 처리를 실행해서 경로를 추가하거나 지우거나해서 최종 경로를 만들어내고, 다른 함수에 전달할 때는 &Path 타입으로 전달하도록하면 성능을 높일 수 있습니다. 프로그램이 보통 설정 파일이나 특정 파일을 찾을 때는 경로를 저장하는 변수의 값을 바꾸지만, 파일을 찾고나면 경로를 바꿀 일이 드뭅니다. 이미 있는 파일의 경로를 바꿀 일이 많지는 않으니까요. 파일을 지우더라도 경로는 바뀌지 않습니다. 그래서 읽기 전용으로 경로를 관리할 일이 많으므로 이렇게 별도의 타입을 만들었다고 생각됩니다. 최대한 활용하시기 바랍니다.

grep 함수는 전달된 파일 경로를 이용하여 파일을 열고 파일을 관리하기 위한 std::fs::File 타입의 객체를 반환합니다. File객체는 std::io::Read 트레이트를 구현하고 있어서 read()나 read_to_string() 메소드를 가지고 있습니다. 

---

여기서 잠깐 어떤 구조체 타입이 어느 트레이트를 구현하고 있는지를 확인하는 방법을 알아보겠습니다. 우선 std::fs::File 구조체의 매뉴얼을 열어보겠습니다. https://doc.rust-lang.org/std/fs/struct.File.html 링크를 열면 페이지 왼쪽에 다음과 같이 자체 메소드와 구현된 트레이트의 리스트가 보입니다.

파일 lib_trait_file1.png

create나 open은 File 구조체의 자체 메소드입니다. 그리고 아까 이야기했던 Read 트레이트가 리스트에 있을 것입니다. 클릭해보면 다음과 같이 read나 read_to_string() 메소드가 있다고 나옵니다. 트레이트 리스트에 Read가 2개가 있는데 자세히 보면 하나는 &File을 위한 트레이트 구현이고 다른 하나는 File을 위한 트레이트 구현입니다. 참조를 사용하냐 변수 그대로 사용하냐에 따라서 구현된 트레이트가 달라집니다. 트레이트의 메소드의 인자에 &self가 있을지 Self가 있을지에 따라 다른 것입니다. &self인 경우 객체의 데이터가 바뀔 수 없고, Self인 경우 객체의 데이터가 바뀌거나 객체가 해지될 수도 있습니다. read()나 read_to_string()은 양쪽에 다 존재합니다.

파일 lib_trait_file2.png

예제에서 read_to_string()을 사용하니까 read_to_string() 메소드의 설명 부분에서 Read more를 클릭해봅시다. 페이지가 Trait std::io::Read로 넘어가게됩니다. 따라서 우리는 read_to_string() 메소드를 사용하기 위해서는 use std::io::Read라는 선언을 추가해줘야한다는 것을 알게됩니다.

---

read_to_string()메소드는 설명에서 보는 것과같이 파일 전체의 내용을 String타입 변수에 추가하는 것입니다. 그렇게 파일 전체를 한꺼번에 읽어서 각 라인별로 찾고자하는 문자열을 가지고있는 라인만 출력하게됩니다.

대강 동작은 하는 프로그램입니다. 제가 한가지 고치고 싶은 것은 에러처리를 제대로하는 것이 아니라 unwrap()을 호출하는 것으로 처리하고 있다는 것입니다. 모든 에러 처리를 unwrap() 메소드를 사용한다는 것은 에러가 발생할 때마다 프로그램이 패닉을 발생시키고 종료된다는 것입니다. 에러 처리라는게 사실상 없는 것이나 마찬가지입니다. 좀더 에러를 제대로 처리하기위해 “?” 연산자와 std::io::Result타입(fs::File의 메소드들은 모두 std::io::Result타입으로 에러를 반환합니다)을 이용하여 에러가 발생했을 때 에러를 상위 레벨로 전달하는 방법과 사용자가 이해하기 쉬운 에러 메세지를 출력하는 방법을 알아보겠습니다. 아래 예제는 일부러 존재하지 않는 파일을 읽도록 만들어서 에러를 발생시키는 코드입니다.

```rust
use std::{
    env::current_dir,
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

/// &Path is immutable form of PathBuf (same to &str and String)
fn grep(filename: &Path, word: &str) -> std::io::Result<()> {
    let mut f: File = File::open(filename)?;
    let mut text_buffer = String::new();

    f.read_to_string(&mut text_buffer)?;
    for line in text_buffer.split('\n') {
        if line.contains(word) {
            println!("{line}");
        }
    }
    Ok(())
}

fn main() -> std::io::Result<()> {
    let mut filename: PathBuf = current_dir()?;

    filename.push("src/wrongfilename.rs");
    grep(&filename, "main")?; // show lines that include specific word
    Ok(())
}
```

```rust
% cargo run
   Compiling ex v0.1.0 (/Users/user/ex)
    Finished dev [unoptimized + debuginfo] target(s) in 0.23s
     Running `target/debug/ex`
Error: Os { code: 2, kind: NotFound, message: "No such file or directory" }
```

프로그램을 실행하면 어떤 에러가 발생했는지 에러의 번호와 에러 타입, 에러를 좀 더 설명하는 메세지를 출력합니다. 에러의 번호는 운영체제가 해당 에러를 어떤 번호로 처리하는지에 따라 달라지는데 리눅스의 경우 “errno -l” 명령으로도 확인할 수 있고, “man errno” 명령으로도 확인할 수 있습니다. kind는 에러의 타입인데 숫자로된 번호를 일종의 타입으로 바꾼것이므로 에러 번호나 마찬가지입니다.

에러를 처리할 때 기본 std::io::Result와 같이 러스트의 기본 모듈이 제공하는 에러 타입을 사용할 수도 있지만 그렇게되면 여러가지 에러 타입들을 처리하기 위해 에러 처리 코드가 복잡해질 수밖에 없습니다. 다음과 같이 std::io::Result를 반환하는 함수와 std::core::Result를 반환하는 함수를 호출하는 함수는 어떤 에러를 반환해야할까요?

```rust
fn multiple_errors(filename: &str, number: &str) -> std::io::Result<()> {
    let mut f = File::open(filename)?; // std::io::Result
    let num: u32 = number.parse()?; // https://doc.rust-lang.org/std/str/trait.FromStr.html#associatedtype.Err
    Ok(())
}

fn multiple_errors(
    filename: &str,
    number: &str,
) -> core::result::Result<(), core::num::ParseIntError> {
    let _ = File::open(filename)?; // std::io::Result
    let _ = number.parse::<u32>()?; // https://doc.rust-lang.org/std/str/trait.FromStr.html#associatedtype.Err
    Ok(())
}
```

std::io::Result<()>를 반환하도록하면 parse()의 에러를 처리하지못하고, parse()의 에러를 반환하도록하면 open()의 에러를 처리하지 못합니다.

결국 프로그램의 규모가 조금이라도 커지게되면 다음과같이 프로그램 고유의 표준 에러 타입을 만들고, 여러가지 에러 타입들을 표준 에러 타입으로 변환해서 사용하게됩니다.

## 타입을 바꿔주는 From/Into와 TryFrom/TryInto 트레이트

### From과 Into

특정 타입을 다른 타입으로 바꿀때 사용하는 트레이트입니다. 

이 트레이트가 저처럼 C/C++만 사용하시던 분들께는 필요성이 잘 이해가 안될 수 있습니다. 저도 처음에는 이게 굳이 왜 필요한가 생각했었습니다. 그런데 한번 사용해보니 왜 필요한지 이해가되서 점점 더 자주 사용하게되었습니다. 제가 C/C++로 프로그래밍하는 경우, 한 모듈에서 다른 모듈로 데이터를 전달할때 구조체의 각 필드를 쪼개서 전달하는게 보통이었습니다. 그리고 다른 모듈은 여러개의 데이터를 하나의 새로운 데이터 구조로 만들어서 사용했습니다. 그런데 From/Into같은 표준 트레이트가 있으면 모듈간 데이터를 전달할 때 표준 인터페이스가 있다는 의미가 되므로 데이터 전달 과정에서 오류를 방지할 수 있게됩니다.

다음은 Book타입의 객체를 u32타입의 ISBN 숫자로 바꾸는 예제입니다.

```rust
#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    published: u32,
    isbn: String,
}

impl From<Book> for u32 {
    fn from(book: Book) -> u32 {
        book.isbn.parse().unwrap_or(0)
    }
}

fn main() {
    let the_book = Book {
        title: String::from("The Rust Programming Language"),
        author: String::from("Steve Klabnik and Carol Nichols"),
        published: 20230228,
        isbn: String::from("718503105"),
    };

    let rust_in_action = Book {
        title: String::from("Rust in Action"),
        author: String::from("Tim McNamara"),
        published: 20210810,
        isbn: String::from("1617294551"),
    };

    let isbn: u32 = the_book.into();
    let isbn2 = u32::from(rust_in_action);
    println!("The book is {isbn} and Rust in Action is {isbn2}");
}
```

Book타입을 u32로 변경하기 위한 것이므로 최종 생성되는 데이터는 u32타입이 됩니다. 그래서 u32타입을 위해 From<Book> 트레이트를 구현하는 것입니다. from이라는 메소드를 구현하게되는데 인자를 Book 타입을 받고, 반환값이 u32가 됩니다.

보통 From을 더 많이 사용하는데, From을 구현하면 Into가 자동으로 구현되기 때문입니다. 예제를 봐도 into 메소드를 구현하지 않았는데 into 메소드를 사용할 수 있는 것을 볼 수 있습니다.

### TryFrom

간단한 From/Into를 알아봤으니, 이번에는 조금 더 유연하게 사용할 수 있는 TryFrom/TryInto의 예제를 보겠습니다. 이름에 Try가 들어가는 것을 보면 알 수 있듯이, 시도를 해보고 안되면 실패를 반환할 수 있는 것이 From/Into와 차이점입니다. 이전 예제에서 From/Into의 반환값을 보면 Result타입이 아니라 곧바로 u32를 반환하는 것을 볼 수 있습니다. 이번 예제에서는 Result를 반환하도록 만들어보겠습니다.

```rust
#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    published: u32,
    isbn: String,
}

impl TryFrom<Book> for u32 {
    type Error = u32;

    fn try_from(book: Book) -> Result<u32, Self::Error> {
        match book.isbn.parse::<u32>() {
            Ok(n) => Ok(n),
            Err(_) => Err(0),
        }
    }
}

fn main() {
    let the_book = Book {
        title: String::from("The Rust Programming Language"),
        author: String::from("Steve Klabnik and Carol Nichols"),
        published: 20230228,
        isbn: String::from("718503105"),
    };

    let rust_in_action = Book {
        title: String::from("Rust in Action"),
        author: String::from("Tim McNamara"),
        published: 20210810,
        isbn: String::from("1617294551"),
    };

    let isbn: Result<u32, u32> = the_book.try_into();
    let isbn2 = u32::try_from(rust_in_action);
    println!("The book is {:?} and Rust in Action is {:?}", isbn, isbn2);
}
```

TryFrom트레이트를 구현해봤습니다. 트레이트 구현을 보면 가장 처음 한 일이 Error 타입을 어느 타입을 지정할 지 결정하는 일입니다. 책의 ISBN은 0이 될 수 없으니 에러 타입을 u32 타입 정수로 지정해봤습니다. 보다 친절하게 에러 메세지를 지정하고 싶으면 Error타입을 &’static str 타입이나 String 타입으로 지정할 수도 있겠습니다. 그리고 try_from 메소드를 구현하는데 반환값이 Result<u32, Self::Error>입니다. from메소드에서 u32를 반환하는 것과 차이가 있습니다. 메소드의 구현은 간단합니다. 책 데이터에 있는 isbn값이 정상적인 값이라면 Ok(u32)가 반환될 것이고, 정상적인 값이 아니라서 파싱중에 에러가 발생하면 Err(0)이 반환될 것입니다.

상황에 따라 From을 쓸 것인지, TryFrom을 쓸 것인지 선택하면되겠지만, 보다 유연한 에러처리를 위해서는 TryFrom을 쓰는게 좋을 것입니다. 위 예제처럼 아주 단순한 경우라면 From도 충분하겠지만요.
