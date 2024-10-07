# 트레이트(Trait)

# 트레이트 소개

트레이트는 어떤 타입이든 상관없이 공통된 동작을 하도록 지정하는 방법입니다. 예를 들어 서로 다른 타입의 객체들이라고 해도 같은 트레이트을 구현하도록 만들면 결국 공통된 특징을 갖도록 만들 수 있습니다. 함수 인자로 객체를 넘길 때, 타입을 지정하는게 보통이지만, 특정한 트레이트을 구현한 타입이면 무엇이든지 전달할 수도 있게 됩니다. 함수 인자에 구조체 이름이 아니라, 어떤 트레이트의 이름을 쓸 수도 있습니다.

다른 객체지향 언어를 경험해봤다면 추상 클래스나 인터페이스와  비슷한 것이라고 이해해도 좋습니다. 다양한 타입의 객체들을 묶는 추상화를 할 수 있기도 하고, 코드를 재사용할 수도 있는 등 러스트로 규모있는 프로그램을 만들기 위해서는 필수적으로 잘 활용할 수 있어야되는 문법입니다.

아주 간단한 예제를 가지고 시작해보겠습니다. 다음 예제는 서로 다른 두 구조체 Person과 Book에 공통의 트레이트 Printable을 구현하는 예제입니다.

```rust
// src/trait/main.rs
trait Printable {
    type Age;
    fn print(&self);
    fn get_age(&self) -> Self::Age;
}

struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn new(name: &str, age: u32) -> Self {
        Person {
            name: name.to_string(),
            age: age,
        }
    }
}

impl Printable for Person {
    type Age = u32;
    fn print(&self) {
        println!("Name: {}, {} years old", self.name, self.get_age());
    }
    fn get_age(&self) -> Self::Age {
        self.age
    }
}

struct Book {
    title: String,
    author: String,
    published: u32,
}

impl Printable for Book {
    type Age = u32;
    fn print(&self) {
        println!(
            "Title: {}\nAuthor: {}\nPublished: {}",
            self.title,
            self.author,
            self.get_age()
        );
    }
    fn get_age(&self) -> Self::Age {
        self.published
    }
}

fn print_info(item: &dyn Printable<Age = u32>) {
    item.print();
}

fn main() {
    let person = Person::new("Alice", 22);
    let book = Book {
        title: String::from("The Rust Programming Language"),
        author: String::from("Steve Klabnik and Carol Nichols"),
        published: 20230228,
    };

    print_info(&person);
    print_info(&book);
}
```
```bash
$ cargo run --bin trait
    Finished dev [unoptimized + debuginfo] target(s) in 0.75s
     Running `target/debug/trait`
Name: Alice, 22 years old
Title: The Rust Programming Language
Author: Steve Klabnik and Carol Nichols
Published: 20230228
```

첫번째로 Printable이라는 트레이트를 선언하는 코드가 나옵니다.

```rust
trait Printable {
    type Age;
    fn print(&self);
    fn get_age(&self) -> Self::Age;
}
```

Printable이라는 트레이트에는 2개의 함수와 1개의 타입이 포함됩니다. 이제 이 트레이트를 구현할 구조체들은 2개의 함수를 정의해야합니다. 그리고 Age라는 타입을 무슨 데이터 타입으로 사용할 것인지를 정해야합니다. i32같은 기본 타입을 사용할 수도있고, 새로운 구조체를 만들어서 사용할 수도있습니다. 그러니 트레이트라는 것이 반드시 특정 함수를 구현하도록 하는 것만이 아니라, 어떤 데이터 타입을 쓸지도 정할 수 있다는 것을 주의하시기 바랍니다.

그리고 Person 구조체를 정의합니다.

```rust
struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn new(name: &str, age: u32) -> Self {
        Person {
            name: name.to_string(),
            age: age,
        }
    }
}
```

우선 Person타입이 트레이트와 상관없이 가지는 Person타입만의 고유한 함수 new를 구현합니다. 그 다음으로 Person을 위해 Printable 트레이트를 구현합니다.

```rust
impl Printable for Person {
    type Age = u32;
    fn print(&self) {
        println!("Name: {}, {} years old", self.name, self.get_age());
    }
    fn get_age(&self) -> Self::Age {
        self.age
    }
}
```

사람의 나이를 의미하는 Age라는 타입을 정의합니다. 타입 이름은 Age이지만, 사실은 u32타입의 별칭(alias)이라고 생각하면 됩니다. 사람 나이는 음수가 없으니까 u32를 사용했습니다. 그리고 print 함수와 get_age함수의 구현이 있습니다. print함수는 사람의 이름과 나이를 터미널에 출력합니다. get_age함수는 Age타입을 반환해야하니까 결과적으로는 u32타입을 반환하는 것이 됩니다.

그리고 Book 구조체의 정의와, Book 구조체를 위한 Printable 트레이트의 구현이 나옵니다.

```rust
struct Book {
    title: String,
    author: String,
    published: u32,
}

impl Printable for Book {
    type Age = u32;
    fn print(&self) {
        println!(
            "Title: {}\nAuthor: {}\nPublished: {}",
            self.title, self.author, self.get_age()
        );
    }
    fn get_age(&self) -> Self::Age {
        self.published
    }
}
```

Age타입을 u32로 정의했습니다. 그리고 print함수와 get_age함수의 구현이 나옵니다. print는 책의 제목과 저자, 출판날짜를 출력합니다. 사람을 위한 Printable 트레이트와 책을 위한 트레이트는 동일한 인터페이스를 가지고 내부 동작만 다릅니다.

이제 정말 중요한게 나옵니다. 바로 dyn이라는 키워드를 쓰는 트레이트 객체입니다.

```rust
fn print_info(item: &dyn Printable<Age = u32>) {
    item.print();
}
```

print_info라는 함수인데, 인자의 타입이 특이합니다. dyn이라는 키워드가 있는 참조 타입입니다. “dyn Printable”는 Printable 트레이트를 구현한 모든 타입을 인자로 받는다는 의미입니다. 참조 키워드 &가 있으므로 Printable 트레이트를 구현한 타입의 참조를 인자로 받는다는 뜻이 됩니다. 그런데 주의할 것이 하나있는데 <Age=u32>라는 구문도 있습니다. 이것의 의미는 Age타입을 u32로 정의한 객체만 참조하겠다는 것이 됩니다.

정리하자면 Printable 트레이트를 구현하되 Age타입을 u32로 정의한 타입들의 참조를 인자로 받는 함수입니다. print_info함수를 호출할 때 서로 다른 타입 Person과 Book의 참조를 전달할 수 있게되는 이유가 바로, Person과 Book이 Printable 트레이트를 구현했고, Age타입을 u32로 정의했기 때문입니다. 만약 Printable 트레이트를 구현했다해도, Age의 타입이 u32이 아닌 다른 타입이었다면 print_info함수에 전달할 수 없습니다.

만약에 여러개의 트레이트를 모두 다 구현하는 타입들을 지정하고 싶다면 아래와같이 사용하면 됩니다.

```rust
fn some_function(param: &(dyn Trait1 + Trait2)) {
    // Function body
}
```

Trait1과 Trait2를 모두 구현한 타입의 레퍼런스를 인자로 받는 함수가 됩니다.

이와같이 dyn 키워드를 사용하고 트레이트 이름으로 나타내는 것을 트레이트 객체라고 부릅니다. 특정한 트레이트를 구현한 객체들을 지정한다고 이해하면 될듯합니다.

## 연관 타입(Associated Type)과 연관 함수 (Associated Function)

이전 예제에서 Printable 트레이트에 있는 Age 타입을 연관 타입이라고 부릅니다. 트레이트 구현에 공통적으로 필요한 변수가 있는데 어떤 타입이 될지는 구현에 따라 달라질 수 있습니다. print_info 함수에서와 연관 타입을 동일하게 구현한 객체들을 공통적으로 사용할 수 있습니다.

그리고 Person구조체에 있는 new 함수를 연관 함수라고 부릅니다. 자세히보면 함수의 인자에 self가 없습니다. 그러므로 특정 구조체 객체에 종속되서 동작하는게 아니라 객체가 없는 상태에서 Person::new와 같이 Person이라는 타입 이름으로만 호출할 수 있습니다. 보통 new라는 이름으로 객체를 생성하는 연관 함수를 만드는게 관례입니다.

## 트레이트 객체(Trait object)

dyn키워드는 트레이트 객체를 나타내는 키워드라고 설명했습니다. 이 트레이트 객체를 좀더 자세히 이해할 필요가 있습니다.

```rust
dyn TraitName
```

위에서본 print_info함수를 다시 보겠습니다. print_info함수는 item이라는 트레이트 객체의 print함수를 호출합니다. 타입 이름이 없는데 어떻게 구현하는 함수를 호출할 수 있을까요?

```rust
fn print_info(item: &dyn Printable<Age = u32>) {
    item.print();
}
```

Person타입의 print함수는 0x1000_0000에 있고, Book타입의 print함수는 0x3000_0000에 있다고 생각해봅시다. 컴피일러는 vtable이라는 것을 만들어서 트레이트에서 구현된 함수들의 포인터를 관리합니다. (이것은 자바나 C++의 인터페이스와 동일합니다.)

```rust
book.vtable = {
    print = 0x1000_0000;
    ... 다른 함수들의 시작 주소들도 들어갈 수 있음
}
person.vtable = {
    print = 0x3000_0000;
    ... 다른 함수들의 시작 주소들도 들어갈 수 있음
}

fn print_info(item: &dyn Printable<Age = u32>) {
    item.vtable.print()
}
```

위와같이 트레이트를 구현하는 객체마다 vtable을 추가해서 트레이트 함수들의 포인터를 저장합니다. 이렇게 추가적인 테이블을 만들어서 함수 포인터를 저정하고, 요청받은 함수를 호출하는 방식을 Dynamic dispatch라고 부릅니다만 용어보다는 공통된 특성 혹은 함수들의 구현을 위해 추가적인 데이터가 들어간다는 것을 알고있는게 중요할 것입니다. 물론 메모리를 한번 더 읽게되는 단점도 있습니다만 특별하게 CPU를 많이 사용하는 연산을 수행하는 부분에서 호출되지 않는한 성능에 차이가 나지 않습니다.

## 표준 라이브러리에 포함된 트레이트

개발자들이 직접 필요한 트레이트를 만들 수 있지만, 러스트의 표준 라이브러리(The Rust Standard Library: https://doc.rust-lang.org/std/index.html)에서 미리 만들놓은 편리한 트레이트들이 있습니다. 표준 라이브러리라고 부르는 만큼 어떤 상황에서도 사용할 수 있을만큼 성능이나 범용적이 좋은 트레이트들입니다. 그중에서 초보 단계에서도 자주 사용하게되는 몇가지만 예제를 만들어보겠습니다.

### Display와 Debug 트레이트

Display라는 트레이트가 있습니다. 구조체 타입을 println! 등의 출력 함수에서 곧바로 출력할 수 있도록 만드는 트레이트입니다. 다음은 Display 트레이트의 정의를 매뉴얼에서 가져온 것입니다.

```rust
use std::fmt;

pub trait Display {
    // Required method
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>;
}
```

위에서 Printable이라는 트레이트를 만들어봤는데 사실상 동일한 일을 하는 트레이트입니다. Book타입의 Display트레이트를 구현해보겠습니다.

```rust
// src/std_trait_display_debug/main.rs
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

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "POINT [{} {}]", self.x, self.y)
    }
}

fn main() {
    let dot: Point = Point { x: 1.2, y: 3.4 };
    println!("{:?}", dot);
    println!("{}", dot);
}
```
```bash
$ cargo run --bin std_trait_display_debug
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.22s
     Running `target/debug/std_trait_display_debug`
POINT [1.2 3.4]
POINT(1.2 3.4)
```

Display라는 트레이트는 fmt라는 하나의 메소드로 이루어져있습니다. fmt메소드의 구현도 간단합니다.

```rust
write!(f, "내가 쓰고싶은 메세지{}", 출력할 데이터)
```

이런 형태로 얼마든지 자유롭게 구현만하면 됩니다. fmt::Formatter라는 객체에 내가 지정한 메세지를 써주기만 하면 됩니다. 단지 주의할 것은 write!매크로 호출에서 “;” 세미콜론을 쓰지 않는 다는 점입니다. fmt함수의 반환값이 fmt::Result입니다. 그러므로 해당 타입의 값을 반환해야합니다. write! 매크로 함수가 fmt::Result 값을 반환하므로 “;” 세미콜론을 쓰면 안됩니다. 만약 ";"를 쓰게되면 아무런 값도 반환하지 않게되서 컴파일 에러가 발생할 것입니다.

사실상 같은 일을 하는 Debug트레이트라는 것도 있습니다. 구현된 코드를 보면 Display와 다를게 없어보입니다. 그럼 Display 트레이트와 뭐가 다른 걸까요? println에서 사용하는 포맷 문자열이 다릅니다. 보통 println에서 “{}”을 사용하면 Display 트레이트의 fmt메소드가 호출되고, “{:?}”를 사용하면 Debug의 fmt메소드가 호출됩니다

트레이트를 처음 설명하면서 만든 Book 구조체 구현 예제에서 Printable이라는 트레이트의 print함수를 만들었었지요. 사실 그렇게 특정 구조체의 값을 출력하는 트레이트나 함수를 직접 구현할 필요가 없습니다. 다음과 같이 #[derive(Debug)]라는 속성을 구조체에 추가해주면, 기본적인 Debug 트레이트 구현을 자동으로 만들어줍니다.

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

    println!("{:?}", book); // USE {:?} for Debug trait
}
```
```bash
Book { title: "The Rust Programming Language", author: "Steve Klabnik and Carol Nichols", published: 20230228 }
```

위와같이 구조체의 이름과 각 필드의 이름이 자동으로 출력됩니다. 따라서 Debug트레이트는 직접 구현하는 것보다 derive로 Debug속성을 추가하는 것을 추천합니다.

Debug와 Display는 사실상 동일한 일을 합니다만, 사용하는 의도가 다른 것입니다. 일반적으로 특정 타입을 문자열로 변환해서 출력하고자할 때는 Display를 사용합니다. 서로 다른 모듈이나 객체들간의 통신 인터페이스로도 사용할 수 있습니다. 하지만 개발자가 임시로 디버깅 용도로 타입의 데이터를 출력하고자할 때나 에러 메세지 등에서는 Debug를 사용하는게 의도에 맞게 사용하는 것입니다. 의도에 맞게 사용하면 코드를 읽는 다른 개발자나 미래의 나 자신에게 더 읽기 쉬운 코드가 될 것입니다.

>
> 파이썬을 경험해보신 분들은 __repr__이나 __str__메소드과 유사한 것들이라고 생각하시면 이해하기 좋습니다.
>

### Clone

Clone 트레이트는 clone이라는 메소드를 구현하는 것인데, 간단하게 설명하면 바로 deep copy를 수행하는 것입니다. 다음은 Clone 트레이트의 정의를 매뉴얼에서 가져온 것입니다.

```rust
pub trait Clone: Sized {
    // Required method
    fn clone(&self) -> Self;

    // Provided method
    fn clone_from(&mut self, source: &Self) { ... }
}
```

한가지 먼저 알아야되는게 있습니다. self와 Self의 차이입니다. self는 객체 자신을 가르키는 변수입니다. 그리고 Self는 객체의 타입입니다. Self는 아무런 객체와 상관없은 단지 현재 트레이트를 구현하는 타입을 가르킵니다. 트레이트를 구현하는 것이 구조체이면 구조체의 이름이라고 생각할 수 있습니다.

그래서 clone함수는 자기 자신을 참조하면서, 같은 타입을 반환하는 함수입니다. 그럼 예제를 한번 만들어보겠습니다.

```rust
// src/trait_clone/main.rs
#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    published: u32,
}

impl Clone for Book {
    fn clone(&self) -> Self {
        Book {
            title: self.title.clone(),
            author: self.author.clone(),
            published: self.published,
        }
    }
}

//fn print_info(item: &dyn Clone) {
//    println!("item implements Clone trait");
//}

fn main() {
    let book = Book {
        title: String::from("The Rust Programming Language"),
        author: String::from("Steve Klabnik and Carol Nichols"),
        published: 20230228,
    };

    let another = Book {
        title: String::from("The another book"),
        author: String::from("Unknown"),
        published: 20111111,
    };

    let mut book_clone = book.clone();
    println!("{:?}", book_clone);
    book_clone.clone_from(&another);
    println!("{:?}", book_clone);

    //print_info(&book);
}
```
```bash
$ cargo run --bin trait_clone
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.25s
     Running `target/debug/trait_clone`
Book { title: "The Rust Programming Language", author: "Steve Klabnik and Carol Nichols", published: 20230228 }
Book { title: "The another book", author: "Unknown", published: 20111111 }
```

clone함수의 내용을 보면 새로운 Book 객체를 만듭니다. title필드는 self.title을 그대로 복사해야합니다. self.title은 String타입의 객체이므로 String타입의 clone메소드를 호출하면 똑같은 객체를 만들 수 있습니다. String 타입의 clone 메소드 또한 Clone 트레이트를 구현한 것이겠지요. 표준 라이브러리에 정의된 타입들은 대부분 clone 메소드를 가지고 있습니다. 만약 clone메소드를 호출하지않고 다음과 같이 만든다면 어떻게될까요?

```rust
impl Clone for Book {
    fn clone(&self) -> Self {
        Book {
            title: self.title,
            author: self.author,
            published: self.published,
        }
    }
}
```

소유권에 대해서 설명할 때 이야기한 바와 같이 힙 영역 메모리에 저장된 객체들은 이와같이 대입이 될때 소유권의 이동이 일어납니다. 그래서 다음과 같은 컴파일 에러가 발생합니다.

```rust
error[E0507]: cannot move out of `self.title` which is behind a shared reference
  --> src/main.rs:43:20
   |
43 |             title: self.title,
   |                    ^^^^^^^^^^ move occurs because `self.title` has type `String`, which does not implement the `Copy` trait
```

만약 String타입에 Copy 트레이트가 구현되어있다면 자동으로 copy 메소드를 호출해서 자동으로 객체 복사를 해주게되지만, String 타입은 Copy 트레이트를 구현하지않았으므로 위와같은 에러가 발생합니다.

그리고 published는 u32타입의 변수이므로 clone이 필요없이 값이 복사됩니다.

이전 예제에서 트레이트를 구현 후 트레이트 객체를 인자로 전달하는 함수를 만들어봤습니다. 그럼 Clone도 트레이트니까 비슷하게 Clone을 구현한 객체들을 인자로 받는 함수를 만들 수 있을까요? 예제에서 주석으로된 print_info 함수를 코드로 만들어서 다시 빌드해보겠습니다.

```rust
...
fn print_info(item: &dyn Clone) {
    println!("item implements Clone trait");
}
...
```
```rust
error[E0038]: the trait `Clone` cannot be made into an object
  --> src/main.rs:50:22
   |
50 | fn print_info(item: &dyn Clone) {
   |                      ^^^^^^^^^ `Clone` cannot be made into an object
   |
   = note: the trait cannot be made into an object because it requires `Self: Sized`
   = note: for a trait to be "object safe" it needs to allow building a vtable to allow the call to be resolvable dynamically; for more information visit <https://doc.rust-lang.org/reference/items/traits.html#object-safety>
```

컴파일러 메세지가 약간 이해하기 어렵지만 이렇게 생각해보면 힌트가 될 수 있습니다. 트레이트 객체로 전달한다는 것은 원래의 타입이 뭔지를 숨기고 여러 타입이 공통으로 구현한 함수를 호출하는 것입니다. 만약 구현하는 함수가 clone 메소드처럼 Self를 반환한다면, 트레이트 객체의 원래 타입이 뭔지 알아야됩니다. 따라서 clone 메소드를 구현해야하는 Clone 트레이트는 트레이트 객체로 사용할 수 없는 것입니다.

에러 메세지에서 알려주는 Object safety(https://doc.rust-lang.org/reference/items/traits.html#object-safety) 라는 규칙들이 있는데 트레이트 오브젝트를 만들어서 쓸 수 있는 트레이트가 가져야하는 규칙들입니다. 여기에 Sized 트레이트를 구현하면 안된다고 써있는데 Clone 트레이트는 상위 트레이트로 Sized 트레이트를 구현하고 있습니다. 위에 Clone트레이트의 정의를 보면 Clone:Sized라고 써있는데 Sized 트레이트를 상위 트레이트로 가진다는 표시입니다. Sized 트레이느는 바로 트레이트에서 사용하는 모든 타입들이 컴파일 시점에 어느 크기를 갖는지 알 수 있다는 표시입니다 컴파일 타임에 Self 타입의 크기를 알아야 메모리 복사에서 안정성을 보장할 수 있기 때문입니다. 참고로 이와같이 어떤 메소드를 구현하고자하는 트레이트가 아니라, 속성만 부여하고자하는 트레이트도 있습니다. 그리고 그런 트레이트들을 Marker trait라고 부릅니다. 그래서 Sized트레이트가 정의된 위치가 std::marker입니다.

러스트는 이와같이 메모리 안정성을 위해서 복잡해보이는 규칙들을 가지고 있습니다. 과하다싶을 때도 있지만, 이런 규칙들을 개발자가 생각하면서 개발해야하는 언어들의 문제점을 해결하는게 이토록 쉽지않은 일이라는 것을 알 수 있습니다. 처음에는 복잡해보이고 의미를 알 수 없는 규칙들이지만, 메모리 안정성을 염두해두고 연습을 해나간다면 이해할 수 있습니다. 시작단계에서는 모든 트레이트가 다 트레이트 객체를 만들 수 있지 않다는 것만 기억해두고 점차 익숙해지면서 더 이해하면 됩니다.

>
> Sized나 Copy 트레이트 등은 메모리를 세밀하게 제어할때 사용됩니다. 그 외 다양한 마커 트레이트가 있지만, 이 책의 수준을 넘어서기때문에 자세히 설명하지 않겠습니다.
>

지금까지 길게 Clone 트레이트의 구현을 설명했지만, 사실은 이렇게 직접 구현할 필요가 없습니다. derive[Clone]속성을 추가해주면 자동으로 Clone트레이트의 구현이 생성됩니다.

```rust
#[derive(Debug, Clone)]
struct Book {
    title: String,
    author: String,
    published: u32,
}
...
```

### Default

Default 트레이트는 구조체의 각 필드를 디폴트값으로 초기화해서 객체를 생성해줍니다. 다음 예제는 Default 트레이트를 직접 구현하지않고 derive(Default) 속성을 추가해서 자동으로 생성된 코드를 사용한 예제입니다.

```rust
// src/trait_default/main.rs
#[derive(Debug, Clone, Default)]
struct Book {
    title: String,
    author: String,
    published: u32,
}

fn main() {
    let book = Book::default();
    let book_clone = book.clone();
    println!("{:?}", book_clone);
}
```
```bash
gkim@gkim-laptop:~/study/my-rust-book$ cargo run --bin trait_default
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
     Running `target/debug/trait_default`
Book { title: "", author: "", published: 0 }
```

Default트레이트는 default라는 메소드를 가지고 있습니다. default 메소드는 새로운 객체를 만들 때 사용하므로 정적 메소드입니다. 따라서 “타입이름::default()” 형태로 호출합니다. 간혹 자동으로 생성되는 값들이 개발자의 의도와 다를 때만 직접 구현해주면 됩니다.

### PartialEq

PartialEq는 두 객체가 같은 값을 가지고 있는지를 확인하는 트레이트입니다.

```rust
// src/trait_partialeq_first/main.rs
#[derive(Debug, Clone, Default)]
struct Book {
    title: String,
    author: String,
    published: u32,
}

impl PartialEq for Book {
    fn eq(&self, other: &Self) -> bool {
        self.title == other.title && self.author == other.author
    }
}

fn main() {
    let second = Book {
        title: String::from("The Rust Programming Language"),
        author: String::from("Steve Klabnik and Carol Nichols"),
        published: 20230228,
    };

    let first = Book {
        title: String::from("The Rust Programming Language"),
        author: String::from("Steve Klabnik and Carol Nichols"),
        published: 20190812,
    };

    if second == first {
        println!(
            "Yes, they are same book but different release {} != {}.",
            first.published, second.published
        );
    }
}
```
```bash
$ cargo run --bin trait_partialeq_first
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/trait_partialeq_first`
Yes, this book is writtend by Person { name: "Steve Klabnik", age: 30 }
```

위와 같이 두 책이 같은 책인지 확인하려면 책 제목과 저자 이름을 확인하게 됩니다.

트레이트 이름이 PartialEq라고해서 왜 Partial이라는 이름이 들어갔는지 의아하게 생각할 수도 있습니다. 하지만 완전하게 동일한 객체를 비교하는게 아니라 위와같이 좀더 넓은 의미에서 일부 같은 값을 가진 객체도 비교할 수 있다는 유연성을 갖는다고 이해하면 쉬울듯합니다. 바로 아래 예제를 생각해보면 됩니다.

```rust
// src/trait_partialeq_second/main.rs
#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

#[derive(Debug, Clone, Default)]
struct Book {
    title: String,
    author: String,
    published: u32,
}

impl PartialEq for Book {
    fn eq(&self, other: &Self) -> bool {
        self.title == other.title && self.author == other.author
    }
}

impl PartialEq<Person> for Book {
    fn eq(&self, other: &Person) -> bool {
        self.author.contains(&other.name)
    }
}

impl PartialEq<Book> for Person {
    fn eq(&self, other: &Book) -> bool {
        other.author.contains(&self.name)
    }
}

fn main() {
    let second = Book {
        title: String::from("The Rust Programming Language"),
        author: String::from("Steve Klabnik and Carol Nichols"),
        published: 20230228,
    };
    let steve = Person {
        name: "Steve Klabnik".to_string(),
        age: 30,
    };
    if second == steve {
        println!("Yes, this book is writtend by {:?}", steve);
    }
}
```
```bash
$ cargo run --bin trait_partialeq_second
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.17s
     Running `target/debug/trait_partialeq_second`
Yes, this book is writtend by Person { name: "Steve Klabnik", age: 30 }
```

Book 타입과 Person 타입은 동일한 객체가 될 수는 없습니다. 하지만 일부 같은 값을 갖는지를 비교할 수는 있겠지요. 아직 제너릭에 대한 이야기를 하지 않았지만, PartialEq에 어떤 타입 바인딩을 쓰는지에 따라서 비교할 대상의 타입을 지정할 수 있다는 것만 생각해보시면 되겠습니다.

PartialEq 외에도 Eq 트레이트가 있습니다. 하지만 보통 우리가 == 연산자로 비교할 때는 PartialEq를 사용한다는 것을 기억하시기 바랍니다.

#### PartialEq와 Eq 트레이트의 차이

러스트에서 일반적으로 두 객체가 같은지 비교하는 트레이트가 PartialEq라는 것이 이상하게 느껴질 것입니다. 왜 이름에 일부분(Partial)이라는 의미가 들어가는지, 왜 Eq가 기본이 아닌지 이상합니다. 사실 정의만 놓고 본다면 PartialEq는 

1. a=b일때 b=a를 만족하고
2. a=b이고 b=c일때 a=c를 만족하는지를

확인하는 것입니다. 그리고 Eq 트레이트는 a==a를 만족하는지를 확인하는 것입니다. 사실 이게 무슨 의미인지 쉽게 이해하기 어렵습니다. 그래서 위에 책과 저자를 비교하는 예제를 조금 더 확장해보면서 생각해보겠습니다.

```rust
//src/trait_partialeq_eq/main.rs
#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

#[derive(Debug, Clone, Default)]
struct Book {
    title: String,
    author: String,
    published: u32,
}

impl PartialEq for Book {
    fn eq(&self, other: &Self) -> bool {
        self.title == other.title && self.author == other.author
    }
}

impl PartialEq<Person> for Book {
    fn eq(&self, other: &Person) -> bool {
        if self.author.contains("Unknown") {
            return false;
        }
        self.author.contains(&other.name)
    }
}

impl PartialEq<Book> for Person {
    fn eq(&self, other: &Book) -> bool {
        if self.name.contains("Unknown") {
            return false;
        }
        other.author.contains(&self.name)
    }
}

fn main() {
    let second = Book {
        title: String::from("Necronomicon"),
        author: String::from("Unknown"),
        published: 20230228,
    };
    let unknown = Person {
        name: "Unknown".to_string(),
        age: 30,
    };
    if second == unknown {
        println!("Yes, this book is writtend by {:?}.", unknown);
    } else {
        println!("No, we don't know who wrote it.")
    }
}
```
```bash
$ cargo run --bin trait_partialeq_eq
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.18s
     Running `/home/gkim/study/my-rust-book/target/debug/trait_partialeq_eq`
No, we don't know who wrote it.
```

책의 저자도 Unknown이고, 사람의 이름도 Unknown입니다. 그럼 이 책 저자와 사람의 이름이 같은 것일까요? 이 책을 이 사람이 썼다고 생각할 수 없는 것입니다. 표기는 같지만 비교할 수 없는 값들이기 때문입니다. 예, 이런 경우가 있기 때문에 PartialEq이라는 이름을 사용하는 것입니다.

그럼 Eq가 필요한 경우는 언제일까요? Eq는 항상 정확하게 비교할 수 있는 경우에 사용해야합니다. 바로 키와 값을 저장하는 HashMap같은 자료구조에서 키로 사용하는 타입은 반드시 Eq 트레이트가 구현되어야합니다. HashMap과 Eq트레이트에 대해서는 뒤에 HashMap의 사용법을 소개할 때 다시 이야기하겠습니다.
















==================================== 2024 10 07 ===================================





# Generic 프로그래밍

제너릭 프로그래밍은 다른 프로그래밍 언어와 마찬가지로 같은 코드를 다른 타입에 사용할 수 있도록 하는 것입니다. 제너릭 타입과 제너릭 함수가 있는데 다음 예제로 알아보겠습니다.

```rust
struct Pair<T> {
    first: T,
    second: T,
}

fn add<T>(a: T, b: T) -> T
where
    T: std::ops::Add<Output = T>,
{
    a + b
}

fn main() {
    let integer_pair: Pair<i32> = Pair {
        first: 5,
        second: 10,
    };

    let float_pair: Pair<f64> = Pair {
        first: 3.14,
        second: 1.618,
    };

    let result = add(4, 6);
    println!("Sum: {}", result); // Prints "Sum: 10"

    let result = add(3.14, 1.618);
    println!("Sum: {}", result); // Prints "Sum: 4.758"
}
```

Pair라는 제너릭 구조체를 만들었습니다. 타입 파라미터는 T입니다. 다른 언어들과 다를게 없어서 쉽게 이해할 수 있습니다.

그 다음은 제너릭 함수 add입니다. 타입 파라미터는 T입니다. 같은 타입의 두 인자를 받아서 두 인자를 합한 값을 반환합니다. 러스트의 제너릭 함수에는 타입 파라미터가 어떤 트레이트를 구현해야하는지를 선언할 수 있습니다.

```rust
where
    T: std::ops::Add<Output = T>,
```

이렇게 where절을 추가하면 T타입은 std::ops::Add트레이트를 구현하고 있어야하며 Output타입은 T가 되어야한다는 제약조건을 걸게됩니다. 아래와같이 정수나 실수를 add함수에 전달할 수 있다는 것은 정수나 실수 타입이 std::ops::Add 트레이트를 구현하고 있다는 의미겠지요.

```rust
    let result = add(3.14, 1.618);
    println!("Sum: {}", result); // Prints "Sum: 4.758"
```

그럼 우리가 만든 타입도 마찬가지로 std::ops::Add 트레이트를 구현한다면 add함수를 이용할 수 있다는 의미가 됩니다. Pair타입을 위한 std::ops::Add 트레이트 구현을 해볼까요.

```rust
#[derive(Debug)]
struct Pair<T> {
    first: T,
    second: T,
}

impl std::ops::Add for Pair<i32> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            first: self.first + rhs.first,
            second: self.first + rhs.second,
        }
    }
}

fn add<T>(a: T, b: T) -> T
where
    T: std::ops::Add<Output = T>,
{
    a + b
}

fn main() {
    let left_pair: Pair<i32> = Pair {
        first: 5,
        second: 10,
    };

    let right_pair: Pair<i32> = Pair {
        first: 10,
        second: 5,
    };

    let result = add(left_pair, right_pair);
    println!("Sum: {:?}", result);
    // Sum: Pair { first: 15, second: 10 }
}
```

이전에 트레이트를 이야기할 때 구현해본 바와 크게 다르지 않습니다. 단지 Pair가 제너릭 타입이기 때문에 i32타입을 바인딩한 Pair타입에 대한 add함수를 구현한다는 표시를 추가했습니다.

```rust
impl 트레이트이름 for Pair<타입> {
...
```

그럼 Pair에 특정한 타입이 바인딩되었을때의 트레이트 구현뿐 아니라 제너릭 타입을 사용한 Pair도 사용할 수 있지 않을까요? 아래와 같이 구현할 수 있습니다.

```rust
#[derive(Debug)]
struct Pair<T> {
    first: T,
    second: T,
}

impl<T> std::ops::Add for Pair<T>
where
    T: std::ops::Add<Output = T> + Copy,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            first: self.first + rhs.first,
            second: self.first + rhs.second,
        }
    }
}

fn add<T>(a: T, b: T) -> T
where
    T: std::ops::Add<Output = T>,
{
    a + b
}

fn main() {
    let left_pair: Pair<i32> = Pair {
        first: 5,
        second: 10,
    };

    let right_pair: Pair<i32> = Pair {
        first: 10,
        second: 5,
    };

    let result = add(left_pair, right_pair);
    println!("Sum: {:?}", result);

    let left_pair_u32: Pair<u32> = Pair {
        first: 5,
        second: 10,
    };

    let right_pair_u32: Pair<u32> = Pair {
        first: 10,
        second: 5,
    };

    let result = add(left_pair_u32, right_pair_u32);
    println!("Sum: {:?}", result);
}
```

사실 이 부분이 러스트에서 이해하기 어렵다고 알려진 부분중에 하나입니다. 바뀐 코드는 단 2줄이지만 조금 복잡합니다. 한줄씩 따로 이야기해보겠습니다.

우선 Pair<i32>와같이 특정 타입만 사용하던 코드를 제너릭 타입 파라미터로 바꿔야합니다. 이럴때는 impl<T>와 Pair<T> 이렇게 2군데에 타입 파라미터를 추가하도록 되어있습니다.

```rust
impl<T> std::ops::Add for Pair<T>
```

그럼 Pair<T> 구조체에 정의된 first와 second간에 + 연산이 실행됩니다. 그럴때 위에서 이야기했듯이 T타입이 + 연산이 가능하다는 제한이 필요합니다. 그래서 아래와같이 T타입이 std::ops::Add 트레이트를 구현해야한다는 제약을 추가합니다.

```rust
where
    T: std::ops::Add<Output = T> + Copy,
```

여기서 Copy트레이트가 추가되었는데요 add함수에서 self인자가 참조가 아니라 값이 넘어가는 것이기때문에 소유권의 이동이 일어나기 때문입니다. Copy 트레이트를 추가하지않고 빌드하면 다음과 같은 에러가 발생합니다.

```rust
error[E0382]: use of moved value: `self.first`
  --> src/main.rs:15:21
   |
14 |             first: self.first + rhs.first,
   |                    ---------------------- `self.first` moved due to usage in operator
15 |             second: self.first + rhs.second,
   |                     ^^^^^^^^^^ value used here after move
   |
note: calling this operator moves the left-hand side
  --> /Users/user/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/ops/arith.rs:92:12
   |
92 |     fn add(self, rhs: Rhs) -> Self::Output;
   |            ^^^^
   = note: move occurs because `self.first` has type `T`, which does not implement the `Copy` trait

For more information about this error, try `rustc --explain E0382`.
```

고맙게도 Copy 트레이트를 추가하라고 알려주고있습니다.

Copy 트레이트는 Marker 트레이트로 소유권이 이동할 때 이 타입이 비트단위로 복사하면 값이 이동될 수 있다는 표시를 한 것입니다. 실제로 구현해야하는 함수가 있는 것은 아닙니다. 보통 Clone 트레이트와 같이 사용되는데 clone을 실행할 때 비트단위로 복사하라고 알려주는 것입니다. 만약 포인터 등이 포함된 타입이라면 Deep copy를 수행하는데 비트단위 복사만으로는 부족하겠지요. 포인터 값만 복사되고 객체는 복사되지 않을 것이니까요. 그런 타입에는 Copy 트레이트를 연결할 수 없습니다. 우리 코드에서 실제로 러스트가 비트단위 복사를 실행하는 것은 아닙니다. 물론 지금 예제와 달리 복사가 필요한 코드를 구현할 수도 있겠지요. 그럴때 필요한 것입니다.

이와같이 러스트 표준 라이브러리나 기타 라이브러리 등에 있는 제너릭 함수를 사용하기 위해서는 보통 해당 제너릭 함수의 타입 파라미터가 어떤 트레이트를 구현해야하는지를 확인해서, 트레이트를 구현하고 제너릭 함수를 이용하면 됩니다. 이렇게 제너릭과 트레이트를 같이 사용되는 경우가 많습니다.

## 트레이트에 제너릭 사용하기

구조체 타입에 제너릭을 사용해봤으니까 이번에는 트레이트에 제너릭을 사용해보겠습니다. 아래 예제는 이전에 트레이트를 설명하면서 사용했던 예제를 제너릭을 사용하도록 바꾼 예제입니다.

```rust
trait Printable<AGE> {
    fn print(&self);
    fn get_age(&self) -> AGE;
}

struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn new(name: &str, age: u32) -> Self {
        Person {
            name: name.to_string(),
            age: age,
        }
    }
}

impl Printable<u32> for Person {
    fn print(&self) {
        println!("Name: {}, {} years old", self.name, self.age);
    }
    fn get_age(&self) -> u32 {
        self.age
    }
}

fn print_info(item: &dyn Printable<u32>) {
    item.print();
}

fn main() {
    let person: Person = Person::new("Alice", 22);
    print_info(&person);
}
```

트레이트를 설명하면서 Printable 트레이트에 아래와 같이 타입 정의도 사용할 수 있다고 설명했었습니다.

```rust
trait Printable {
    type Age;
    fn print(&self);
    fn get_age(&self) -> Self::Age;
}
```

제너릭을 안다면 이것을 아래와같이 제너릭으로 표현할 수 있습니다.

```rust
trait Printable<AGE> {
    fn print(&self);
    fn get_age(&self) -> AGE;
}
```

그리고 Printable을 구현하거나 사용할 때도 “Age=u32”라는 타입 지정을 쓸필요없이 제너릭으로 다음과 같이 구현할 수 있습니다.

```rust
impl Printable<u32> for Person {
    fn print(&self) {
        println!("Name: {}, {} years old", self.name, self.age);
    }
    fn get_age(&self) -> u32 {
        self.age
    }
}

fn print_info(item: &dyn Printable<u32>) {
    item.print();
}
```

물론 어느게 더 간단하다고 할 수는 없습니다만, 제너릭으로 지정하는 타입이 u32, i8같은 기본 타입이 아니라 또다른 구조체 타입이라면 제너릭을 사용하는게 더 간단할 때가 있습니다.

## 트레이트와 구조체 모두 제너릭 사용하기

다음과 같이 트레이트에도 제너릭이 사용되고, 구조체에도 제너릭이 사용되는 경우가 실무에서 더 일반적일 것입니다.

```rust
trait Printable<AGE> {
    fn print(&self);
    fn get_age(&self) -> AGE;
}

#[derive(Debug)]
struct Address {
    post: u32,
    city: String,
}

impl Default for Address {
    fn default() -> Self {
        Address {
            post: 0,
            city: "Unknown".to_string(),
        }
    }
}

struct Person<ADDR> {
    name: String,
    age: u32,
    location: ADDR,
}

impl<ADDR> Person<ADDR> {
    fn new(name: &str, age: u32, addr: ADDR) -> Self {
        Person {
            name: name.to_string(),
            age: age,
            location: addr,
        }
    }
}

impl<ADDR> Printable<u32> for Person<ADDR> {
    fn print(&self) {
        println!("Name: {}, {} years old", self.name, self.age);
    }
    fn get_age(&self) -> u32 {
        self.age
    }
}

fn print_info(item: &dyn Printable<u32>) {
    item.print();
}

fn main() {
    let alice: Person<Address> = Person::new("Alice", 22, Address::default());
    print_info(&alice);
    println!("Alice is at : {:?}", alice.location);

    let bruce: Person<String> = Person::new("Bruce", 33, "NewYork".to_string());
    println!("Bruce is at : {:?}", bruce.location);
}
```

약간 복잡해보이지만 예제에서 2가지의 타입 파라미터를 쓰고있고, 각각이 무엇을 위한 타입인지를 기억한다면 어려울게 없습니다.

1. 트레이트 Printable의 AGE 타입 파라이터
2. 구조체 Person의 ADDR 타입 파라미터

약간의 요령이 있는데 Person 구조체의 타입을 구현할 때는 항상 Person<ADDR>로 사용한다고 기억하면 편합니다.

```rust
impl<ADDR> Person<ADDR> {
...
impl<ADDR> Printable<u32> for Person<ADDR> {
...
```

그리고 트레이트의 구현 코드에서도 ADDR이라는 타입을 사용하지 않아도 써줘야한다는걸 주의해야합니다. 그리고 구조체의 메소드를 구현할때나, 트레이트를 구현할 때나 늘 impl<ADDR>과 같이 impl키워드에도 타입 파라미터를 써줘야합니다

# Life-time 라이프타임

라이프타임은 Dangling reference(유효하지 않은 참조, 이미 해지된 객체에 대한 포인터에 접근하는 경우)를 방지하기 위한 기법입니다. 자세한 설명은 러스트 언어의 표준 문서를 참조하시고 이 장에서는 핵심만 이야기하겠습니다. 사실 라이프타임의 개념을 잘 이해한다고해도 실제로 개발을 하다보면 막막할 때가 자주 생길 것입니다. 그럴때는 기본 개념들을 다시 한번 읽어보고, 컴파일러 에러 메세지를 최대한 활용하는게 필요합니다.

라이프타임은 대부분 아래와같이 어떤 구조체를 만들고, 그 구조체의 일부나 전체를 다른 구조체에서 참조하는 경우에 사용합니다.

```rust
use std::collections::HashMap;

struct Item {
    name: String,
    price: f32,
    quantity: u32,
}

struct Storage {
    items: HashMap<String, Item>,
}

impl Storage {
    fn new() -> Self {
        Storage {
            items: HashMap::new(),
        }
    }

    fn storage_store(&mut self, item: Item) {
        self.items.insert(item.name.clone(), item);
    }
}

struct Statistics<'a> {
    items: &'a HashMap<String, Item>,
}

impl<'a> Statistics<'a> {
    fn new(items: &'a HashMap<String, Item>) -> Self {
        Statistics { items }
    }

    fn get_average(&self) -> f32 {
        let total = self.items.values().fold(0.0, |acc, i| acc + i.price);
        let count = self.items.values().fold(0, |acc, i| acc + i.quantity);
        total / count as f32
    }
}

fn main() {
    let mut mystorage = Storage::new();

    let apple = Item {
        name: "apple".to_string(),
        price: 1.0,
        quantity: 10,
    };
    mystorage.storage_store(apple);

    let banana = Item {
        name: "banana".to_string(),
        price: 2.0,
        quantity: 20,
    };
    mystorage.storage_store(banana);

    let stats = Statistics::new(&mystorage.items);
    println!("{}", stats.get_average());
}
```

아주 간단한 예제를 만들어봤습니다.

Storage는 창고에 저장된 과일의 갯수와 가격을 관리하고, Statistics는 여러 창고중 하나의 창고에 있는 과일 가격에 대한 통계를 계산하는 일을 합니다. Statistics는 창고 정보가 필요하므로 창고에 대한 레퍼런스를 가지게됩니다. Statistics가 레퍼런스를 가지고있는 Storage 객체가 Statistic보다 먼저 해지되면 안됩니다.

이렇게 서로 다른 객체에 대한 참조가 발생할 때 참조된 객체가 먼저 해지되지 않도록 하기 위해서 라이프타임 지정이 필요합니다. 러스트 컴파일러는 컴파일 시점에 모든 객체의 소유권 이동 시점을 체크하므로, 객체가 해지되는 것도 체크할 수 있습니다. 따라서 라이프타임을 지정하게되면 참조된 객체가 먼저 해지되지 않는 것을 보장할 수 있습니다.

# 트레이트와 제너릭을 이용한 에러 처리 실습

러스트에서 처음 Result, Option을 배우면 보통 다음과 같이 에러처리를 하게됩니다.

```bash
fn check_command(cmd: &str) -> Result<usize, String> {
  match cmd {
    "good" => Ok(0),
    "unsupported" = Err("Unsupported command".to_owned()),
    "bad" => Err("Bad command".to_owned()),
    _ => Err("Wierd command".to_owned()),
  }
}

fn handle_command(cmd: &str) -> Result<usize, String> {
  // use unwrap
  let passed = check_command(cmd).unwrap();
  println!("first check passed: status={}", passed);

  // check Result
  let failed = check_command(cmd);
  if failed.is_ok() {
    println!("Second check passed: status={}", failed.unwrap());
  }
  if failed.is_err() {
    println!("Second check failed: status={:?}", failed);
  }

  // use pattern
  match check_command(cmd) {
    Ok(s) => println!("Third check passed:{}", s),
    Err(s) => println!("Error={}", s),
  }

  Ok(0)
}

fn main() {
  let status = handle_command("good");
  if status.is_ok() {
    println!("Everything is fine");
  } else {
    println!("I don't feel good");
  }
}
```

Result타입으로 얻은 반환값을 처리할 때 주로 3가지 방법을 사용하게 되는데

1. unwrap 메소드
2. is_ok, is_err등 unwrap이외의 메소드
3. match를 이용한 패턴 매칭

내가 원하는 값은 사실 Result타입으로 감싸여진 값이기 때문에 그 값을 항상 꺼내야 한다는게 불편합니다. 그래서 보통 unwrap을 많이 사용합니다. 그런데 unwrap은 치명적인 단점이 있어서 아주 초기단계 프로토타입을 작성할 때나 사용하지 실제 서비스에 들어가는 코드에는 사용할 수 없습니다. 바로 프로그램에 패닉을 일으키고 죽는다는 것입니다. 에러를 제대로 처리하지않고 죽기만 하는 코드는 제품에 사용할 수가 없습니다.

그리고 어쨌든 반환값이 에러인지 아닌지를 확인해야하기 때문에 is_ok, is_err 등의 메소드를 사용하거나, 매턴 매칭을 사용해서 Result안에 있는 값을 꺼내서 사용합니다.

사실 이러면 check_command에서 이미 한번 체크한 에러를 handle_command 함수에서 또 다시 체크하는 꼴입니다. 중복되는 에러 체크가 너무 많아집니다. handle_command 함수는 에러를 체크하려는 함수가 아니라, 정상적인 상황에서 처리를 하는게 주 목표인 함수이고, 만약 에러가 났으면 바로 에러 값을 상위 함수로 반환하기만 하면 되는 함수입니다. 그러니 이렇게 에러 체크를 일일이 다시 할 필요가 없지요.

일반적으로 러스트 언어 답게 만든 에러 처리 코드를 한번 보겠습니다. 물론 일반적인 것이고 모든 프로젝트가 이렇게 한다는 것은 아닙니다. 하지만 한번 이런 에러 처리 방법을 보고나면 러스트가 추구하는 에러 처리 방식과 Result/Option 등의 동작 방식을 더 잘 이해하실 수 있을 것입니다.

첫번째로 내 프로젝트 전반에서 공통적으로 사용할 에러 타입을 하나 만들고 Result타입을 재정의합니다.

```rust
#[derive(Debug, PartialEq)]
pub enum MyError {
    UnsupportedCommand,
    WrongInput(String),
    UnknownValue {
        name: String,
        expected: String,
        found: String,
    },
    // Add more error cases
}

pub type Result<T, E = MyError> = std::result::Result<T, E>;
```

먼저 내 프로젝트에서 발생할 수 있는 에러들을 생각해서 각 에러마다 타입을 만들어주고, 각 타입마다 어떤 정보를 넣어줄 것인지를 생각해서 MyError를 만들어줍니다. 위의 예제에서는 command처리에 있어서 3가지 경우의 에러가 발생할 것을 가정해서 각각의 경우에 따른 에러 값을 만들어줬습니다.

- UnsupportedCommand: 명령어가 지원되지 않는 경우는 명확하니까 추가적인 데이터는 없는 에러 값을 만들어줍니다.
- WrongInput(String): 잘못된 입력의 경우 어떤 입력이 잘못되었는지를 에러 메세지로 반환하기 위해 String 데이터를 포함하는 에러 값을 만들어줍니다.
- UnknownValue{…}: 이 에러는 사용자에게 좀 더 많은 데이터를 전달하기 위해 구조체를 포함하는 에러 타입입니다.

이렇게 에러 타입에 따라 다양한 정보를 넣어줄 수 있으니 너무나 편리해집니다. 경우에 따라 서로 다른 타입의 데이터를 처리하기위해 if-else를 넣을 필요도 없고 MyError라는 타입 하나만으로 모든 에러를 처리할 수 있으면서도, 각 에러 경우에 따라 다른 정보를 넣어서 관리할 수 있으니 에러 처리 코드가 간결해집니다. 또한 러스트에서 항상 모든 에러를 다 처리해줬는지를 체크해주니 에러 처리를 빼놓는 일을 막을 수 있습니다.

프로젝트가 커지거나 개발을 해나가면서 좀 더 다양한 에러가 발생할 수 있습니다. 당연히 MyError에 새로 추가된 데이터 타입을 추가하면 됩니다.

내가 만든 함수에서는 MyError를 반환할 수 있지만, 다른 라이브러리를 사용할 때 얻게되는 다른 타입의 에러는 어떻게 에러처리를 해야할까요?

```rust
    let mut file = match File::open(&path) {
        Err(why) => MyError::WrongInput(format!("File::open error={}", why),
        Ok(file) => file,
    };
```

그럴때는 이와같이 에러의 타입을 바꿔주면 됩니다. 이 함수를 호출하는 상위 함수는 MyError 타입만을 사용하게 됩니다.

그럼 새로운 에러 타입은 어떻게 사용할까요. check_command 함수에 새로운 에러 타입을 적용해봤습니다.

```rust
// fn check_command(cmd: &str) -> Result<usize, String> {
fn check_command(cmd: &str) -> Result<usize> {
    match cmd {
        "good" => Ok(0),
        "unsupported" => Err(MyError::UnsupportedCommand),
        "bad" => Err(MyError::WrongInput(format!(
            "Cannot handle the command:{}",
            cmd
        ))),
        _ => Err(MyError::UnknownValue {
            name: "Wierd Command Error".to_owned(),
            expected: "good".to_owned(),
            found: cmd.to_owned(),
        }),
    }
}
```

내 프로젝트에서 사용할 에러 타입을 만들고, 이 에러 타입을 사용할 Result를 재정의하고 나면 check_command함수와 같이 함수의 반환값이 단순해집니다. 원래는 Result<usize, String>과 같이 정상 상황일때의 반환값 타입과 에러 상황에서의 반환값 타입을 같이 써줘야하지만, Result를 재정의하고 나면 정상 상황에서의 반환값 타입만 기록해주면 됩니다. 에러 상황에서의 반환값 타입은 암묵적으로 내가 정의한 에러 타입이 되는 것입니다.

```rust
pub fn handle_command(cmd: &str) -> Result<usize> {
    // use ? operator if it doesn't need to check the error value in this function
    let passed = check_command(cmd)?;
    println!("Good command passed: status={}", passed);

    // use map_err to handle the error
    let _ = check_command(cmd)
        .map_err(|e| println!("Command failed: error={:?}", e))
        .map(|s| println!("Command passed: status={}", s));

    // use pattern
    if let Ok(status) = check_command(cmd) {
        println!("wierd but ok: status={:?}", status);
    } else {
        println!("Command failed: command={}", cmd);
    }
    Ok(0)
}
```

만약 내 프로젝트의 규모가 커져서 여러 하위 모듈로 나눠지고 각 모듈별로 에러 타입을 나눠야할 필요가 생기면 하위 모듈마다 에러 타입을 가지고 상위 모듈을 하위 모듈의 에러를 사용하도록 만들면 됩니다. 에러 타입들도 계층 구조를 가지게되므로 프로젝트를 모듈로 나누는게 더 편리해집니다.

아래는 handle_command 함수를 mycommand.rs로 옮기고 handle_command를 호출하는 super_handle_command을 main.rs에 만들어준 것입니다.

```rust
mod mycommand;
use crate::mycommand::*;

#[derive(Debug, PartialEq)]
pub enum SuperError {
    CommandError(MyError),
}

pub type Result<T, E = SuperError> = std::result::Result<T, E>;

impl From<MyError> for SuperError {
    fn from(err: MyError) -> Self {
        Self::CommandError(err)
    }
}

fn super_handle_command(cmd: &str) -> Result<usize> {
    Ok(mycommand::handle_command(cmd)?)
}

fn main() {
    let status = super_handle_command("bad");
    if status.is_ok() {
        println!("Everything is fine");
    } else {
        println!("I don't feel good:{:?}", status);
    }
}
```

main 모듈은 mycommand.rs 모듈의 에러 타입을 포함하는 새로운 에러 타입을 만들고 사용합니다. 한가지 추가된 것은 From 트레이트를 구현하는 것입니다. 그래야 handle_command에서 반환된 MyError타입이 자동으로 SuperError로 변환되서 main함수로 전달될 수 있습니다. 새로운 하위 모듈이 생길때마다 From<새로운 하위 모듈의 에러 타입> 트레이트를 구현하면 됩니다.

그 외에 에러값을 처리하는 것은 하위 모듈에서와 동일합니다. 하위 모듈의 함수를 호출할 때마다 일일이 에러를 체크할 필요가 없어지니 편리합니다.

에러 값을 출력해보면 아래와 같이 상위 에러 타입안에 하위 에러 타입이 저장되어있는 것을 확인할 수 있습니다.