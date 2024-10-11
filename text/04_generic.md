# 제네릭(Generic) 프로그래밍 소개

제네릭 프로그래밍은 다른 프로그래밍 언어와 마찬가지로 같은 코드를 다른 타입에 사용할 수 있도록 하는 것입니다. 제네릭을 이용한 타입(구조체와 열거형)과 제네릭을 이용한 함수(일반 함수, 메소드, 트레이트 등)을 만들 수 있는데 있는데 다음 예제로 제네릭 타입, 함수, 트레이트에 대해서 알아보겠습니다.

```rust
// src/generic_struct/main.rs
#[derive(Debug)]
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
```bash
$ cargo run --bin generic_struct
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.23s
     Running `target/debug/generic_struct`
Sum: Pair { first: 15, second: 10 }
Sum: Pair { first: 15, second: 10 }
```

코드를 하나씩 뜯어보겠습니다. 가장 먼저 Pair라는 구조체를 만들었습니다. 

```rust
#[derive(Debug)]
struct Pair<T> {
    first: T,
    second: T,
}
```

타입 파라미터는 T입니다. T에는 다양한 타입들이 올 수 있습니다. u32같은 기본 타입도 사용될 수 있고, 다른 구조체 타입도 올 수 있습니다.

>
> C++ 언어의 제네릭과 완전히 동일한 문법을 가지고있고, 파이선등의 다른 언어들에서 제네릭을 사용하는 것과도 거의 동일하므로, 다른 언어에서 제네릭을 접해보신 분들은 쉽게 사용하실 수 있습니다.
>

그 다음은 일반 함수 add입니다. 

```rust
fn add<T>(a: T, b: T) -> T
where
    T: std::ops::Add<Output = T>,
{
    a + b
}
```

타입 파라미터는 T입니다. 마찬가지로 T에는 다양한 타입들이 올 수 있습니다. 인자가 두개 있는데 두 인자가 반드시 같은 타입이어야만 합니다. 같은 타입의 인자를 두개 받아서 두 인자를 합한 값을 반환합니다. 러스트에서 제공하는 추가적인 기능이 있는데, 러스트의 제네릭 함수에는 타입 파라미터가 어떤 트레이트를 구현해야하는지를 선언할 수 있습니다.

```rust
where
    T: std::ops::Add<Output = T>,
```

이렇게 where절을 추가하면 T타입은 std::ops::Add트레이트를 반드시 구현하고 있어야하며 Add 트레이트에서 사용하는 Output타입은 T가 되어야한다는 제약조건을 걸게됩니다. 아래와같이 정수나 실수를 add함수에 전달할 수 있다는 것은 정수나 실수 타입이 내부적으로 이미 std::ops::Add 트레이트를 구현하고 있기 때문입니다.

```rust
    let result = add(3.14, 1.618);
    println!("Sum: {}", result); // Prints "Sum: 4.758"
```

정수나 실수가 std::ops::Add 트레이트를 이미 구현하고 있기 때문에 add 함수에 사용될 수 있다면, 그럼 우리가 만든 Pair라는 타입도 마찬가지로 std::ops::Add 트레이트를 구현한다면 add함수를 이용할 수 있다는 의미가 됩니다. Pair타입을 위한 std::ops::Add 트레이트 구현을 해볼까요.

```rust
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
```

이전에 트레이트를 소개할 때 구현해본 것과 조금 더 복잡해졌습니다. 각 차이점들을 하나씩 확인해보겠습니다.

가장 먼저 Pair 타입이 제네릭 타입이기 때문에 Pair타입을 사용하기 위해서 트레이트 구현에도 똑같이 <T>라는 타입 지정이 추가(이렇게 제네릭에서 타입이 지정되는 것을 바인딩이라고 부릅니다)되었습니다. impl<T>와 Pair<T> 이렇게 2군데에 타입 파라미터를 추가합니다.

```rust
impl<T> std::ops::Add for Pair<T>
```

add함수에서 T타입이 std::ops::Add 트레이트 구현이 되어있어야한다는 제한을 했습니다. 그래서 Pair를 위한 트레이트 구현에서도 아래와같이 T타입이 std::ops::Add 트레이트를 구현해야한다는 제약을 추가합니다.

```rust
where
    T: std::ops::Add<Output = T> + Copy,
```

여기서 Copy트레이트가 추가되었는데요 add함수에서 self인자가 참조가 아니라 값이 넘어가는 것이기때문에 소유권의 이동이 일어나기 때문입니다. add함수의 self인자가 &self와같은 참조가 아님을 잘 봐주세요. Copy 트레이트를 추가하지않고 빌드하면 다음과 같은 에러가 발생합니다.

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

고맙게도 Copy 트레이트를 추가하라고 알려주고있습니다. 지금 단계에서 잘 이해가 안된다고해도 컴파일러가 알려주는대로 Copy를 추가해서 일단 빌드가되도록 만들고 천천히 이해해도 좋습니다.

>
> Copy 트레이트는 Marker 트레이트로 소유권이 이동할 때 이 타입이 비트단위로 복사하면 값이 이동될 수 있다는 표시를 한 것입니다. 실제로 구현해야하는 함수가 있는 것은 아닙니다. 보통 Clone 트레이트는 객체도 같이 복사하는 Deep copy를 수행하는데 Copy는 비트단위 복사만 실행합니다. 즉 포인터 값만 복사되고 객체는 복사되지 않습니다. 포인터외에도 i32같은 기본 타입에는 Copy 트레이트가 구현되어있습니다. 우리 코드에서 self가 인자로 넘어갈 때 비트단위로 복사되어서 전달됩니다.
>

std::ops::Add 트레이트의 구현을 보겠습니다.

```rust
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            first: self.first + rhs.first,
            second: self.first + rhs.second,
        }
    }
}
```

두 타입을 더한 결과물은 같은 타입이 되는게 보통이겠지요. 그래서 Output을 Self 타입으로 지정합니다. 그리고 add메소드를 구현합니다. add 메소드의 반환값은 함수 내부에서 새롭게 만들어진 Self타입의 객체입니다. first와 second 필드를 각각 계산해서 새로운 Self 타입의 객체를 만들어서 반환합니다.

그래서 최종적으로 main함수에서 Pair타입의 두 변수, left_pair와 right_pair를 add함수에 전달할 수 있게됩니다.
```rust
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
...
```

여기에서 result의 타입이 생략되었지만 Pair<i32>타입이라는 것을 알아낼 수 있어야합니다.

이와같이 러스트 표준 라이브러리나 기타 라이브러리 등에 있는 함수나 트레이트등은 대부분 제네릭을 사용하고 있습니다. 이것들을 사용하기 위해서는 먼저 해당 제네릭 타입 파라미터가 어떤 트레이트를 구현해야하는지를 확인해서, 트레이트를 구현하고 그 다음 제네릭 함수나 트레이트를 이용하면 됩니다. 이렇게 제네릭과 트레이트를 같이 사용되는 경우가 거의 대부분입니다. 처음에는 많이 낯설고 이해하기 쉽지 않지만, 몇번 만들다보면 익숙해집니다.

## 트레이트에 제네릭 사용하기

구조체 타입에 적용된 제네릭을 사용해봤으니까 이번에는 트레이트에 적용된 제네릭을 사용해보겠습니다. 아래 예제는 이전에 트레이트를 설명하면서 사용했던 Printable 트레이트 구현 예제를 제네릭을 사용하도록 바꾼 예제입니다.

```rust
// src/generic_trait/main.rs
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
```bash
$ cargo run --bin generic_trait
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.17s
     Running `target/debug/generic_trait`
Name: Alice, 22 years old
```

트레이트를 설명하면서 Printable 트레이트에 아래와 같이 연관 타입을 사용할 수 있다고 설명했었습니다.

```rust
trait Printable {
    type Age;
    fn print(&self);
    fn get_age(&self) -> Self::Age;
}
```

제네릭을 안다면 이것을 아래와같이 제네릭으로 바꿀 수 있습니다.

```rust
trait Printable<AGE> {
    fn print(&self);
    fn get_age(&self) -> AGE;
}
```

그리고 Printable을 구현하거나 사용할 때도 “Age=u32”라는 연관 타입 지정을 쓸필요없이 제네릭으로 다음과 같이 구현할 수 있습니다.

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
print_info에 전달할 트레이트 객체도 <u32>이라는 타입 바인딩을 가지고있어야합니다. 트레이트 객체와 제네릭이 결합된게 한번에 눈에 들어오지 않을 수 있습니다. 이렇게 3단계로 생각하면 좋습니다.

1. print_info에 전달되는 인자는 하나의 타입이 아니라, 다양한 타입의 레퍼런스를 받을 수 있다.
2. print_info에 전달되는 타입은 Printable이라는 트레이트를 구현한 타입의 레퍼런스이다.
3. Printable이라는 트레이트가 반드시 u32 타입을 위해 구현되어야한다.

제너릭과 연관 타입 중에서 어느게 더 간단하다고 단정할 수는 없습니다만, 제네릭으로 지정하는 타입이 u32, i8같은 기본 타입이 아니라 또다른 구조체 타입이라면 제네릭을 사용하는게 더 간단할 때가 많습니다.

## 트레이트와 구조체 모두 제네릭 사용하기

다음과 같이 트레이트에도 제네릭이 사용되고, 구조체에도 제네릭이 사용되는 경우가 실무에서 더 일반적일 것입니다.

```rust
// src/generic_trait_struct/main.rs
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
```bash
$ cargo run --bin generic_trait_struct
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.14s
     Running `target/debug/generic_trait_struct`
Name: Alice, 22 years old
Alice is at : Address { post: 0, city: "Unknown" }
Bruce is at : "NewYork"
```

예제가 복잡해보이지만 총 2가지의 타입 파라미터를 쓰고있고, 각각이 무엇을 위한 타입인지를 기억하면서 시작해보겠습니다.

1. 트레이트 Printable의 AGE 타입 파라이터: 나이를 표현하는 타입입니다.
2. 구조체 Person의 ADDR 타입 파라미터: 주소를 표현하는 타입입니다.

약간의 요령이 있는데 Person 구조체의 타입을 구현할 때는 항상 Person<ADDR>로 사용한다고 기억하면 편합니다. Printable 트레이트의 구현 코드에서도 Person<ADDR>과 같이 제네릭 타입을 항상 써줘야한다는걸 주의해야합니다.


```rust
impl<ADDR> Person<ADDR> {
...
impl<ADDR> Printable<u32> for Person<ADDR> {
...
```

그럼 예제 코드를 쪼개서 하나씩 읽어보겠습니다. 가장 먼저는 Printable이라는 트레이트를 선언합니다.

```rust
trait Printable<AGE> {
    fn print(&self);
    fn get_age(&self) -> AGE;
}
```

Printable이라는 트레이트가 있는데 아직 구현은 안되어있습니다. 단지 그런 트레이트가 있고, print와 get_age라는 메소드들이 있다는 것을 선언한 것입니다. get_age는 AGE라는 제네릭 타입을 반환하게됩니다. 나이는 보통 정수값이 되겠지만, 그렇지 않은 경우도 있나봅니다. 어쨌든 제네릭 타입이니 나중에 Printable 트레이트를 구현하는 코드에서 AGE 타입을 무엇으로할지 정하게될거라는 예상을 할 수 있습니다.

그 다음은 Address라는 구조체 타입을 만들고있습니다.

```rust
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
```

별다를게 없는 구조체타입입니다. Default 트레이트를 구현해줬습니다. city라는 필드가 디폴트값이라면 보통 빈 문자열이 되겠지만, 굳이 "Unknown"으로 초기화될 수 있도록, Default 트레이트를 직접 구현한 것입니다.

그 다음은 Person이라는 구조체 타입입니다.

```rust
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
```

사람의 이름, 나이, 위치를 저장하는 구조체입니다. 위치가 꼭 집 주소같은 문자열이 될 필요는 없으므로 ADDR이라는 제네릭 타입을 사용해줬습니다. 우편번호도 될 수 있고, 좌표도 될 수 있으니 Post나 Coordinates같이 새로운 타입을 사용할 수도 있겠네요. 그래서 제네릭 타입으로 만들었나봅니다. impl<ADDR> Person<ADDR>과 같이 ADDR이라는 제네릭 타입을 어디에 써줘야되는지를 잘 봐주세요.

이제 우리가 보고자했던 코드가 나옵니다. 제네릭 타입을 사용한 구조체에 제네릭 타입을 사용한 트레이트를 구현하는 코드입니다.

```rust
impl<ADDR> Printable<u32> for Person<ADDR> {
    fn print(&self) {
        println!("Name: {}, {} years old", self.name, self.age);
    }
    fn get_age(&self) -> u32 {
        self.age
    }
}
```

첫줄은 트레이트 구현을 선언하는 코드인데 2개의 제네릭 타입이 동시에 나오므로 눈에 잘 안들어옵니다. 한글로 다시 써보겠습니다.
```rust
impl<구조체의 제네릭 타입> 트레이트<트레이트의 제네릭 타입> for 구조체이름<구조체의 제네릭 타입> {
```

Printable 트레이트가 u32타입으로 구현됩니다. 따라서 제네릭 타입을 사용하는 get_age라는 함수가 u32 타입을 반환하게됩니다.

마지막으로 print_info에 전달되는 트레이트 객체가 Printable<u32> 타입이 됩니다.

```rust
fn print_info(item: &dyn Printable<u32>) {
```

Printable<i32>나 Printable<String>등등 제네릭 타입이 다른 Printable 트레이트를 구현한 타입들은 print_info에 전달 될 수 없습니다. 오직 Printable<u32>만이 사용될 수 있습니다.

트레이트와 구조체, 열거형에 모두 제네릭 타입이 사용되면 여러개의 제네릭 타입이 사용되고, 어느게 어디에 바인딩된 타입인지 헷갈릴 수 있습니다. 조금 시간이 필요한 부분이므로 조급하게 생각하지말고 천천히 적응해나가도록 합시다. 조금씩 읽고 쓰다보면 그 편리함과 유연함을 즐길 수 있는 순간이 올 것입니다.


















======================== 2024 10 11 ==============================















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

# 트레이트와 제네릭을 이용한 에러 처리 실습

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