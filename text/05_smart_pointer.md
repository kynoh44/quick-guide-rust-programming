# 데이터를 자동으로 관리하는 스마트 포인터

일반 포인터는 단순히 데이터의 주소를 가지는 타입이지만, 스마트 포인터는 가리키는 데이터를 관리하기 위한 메타데이터나 메모리 관리 기능을 가지고 있기 때문에 중요합니다. 스마트 포인터는 트레이트와 제네릭을 활용해서 구현되기 때문에 스마트 포인터를 잘 활용하기 위해서는 트레이트와 제네릭을 잘 이해할 필요가 있습니다.

스마트 포인터는 사실은 일반적인 구조체입니다. 하지만 다음 2개의 트레이트를 구현하고있기 때문에 자동으로 메모리 관리를 할 수 있게되는 것입니다.

1. Deref 트레이트: 메모리 주소나 레퍼런스를 읽어서 원본 데이터에 접근하는 것이 역참조(Dereference)입니다. 이런 스마트 포인터는 사실상 구조체라고 말씀드렸습니다. 구조체는 포인터가 아니니 구조체를 통해서 데이터에 접근하려면 뭔가 추가적인 구현이 필요하겠지요. 이게 바로 Deref 트레이트입니다. *와 같은 역참조 연산자를 스마트 포인터와 사용할 수 있게 해줍니다. 잠시 후에 직접 구현해보면서 다시 이야기하겠습니다.
2. Drop 트레이트: 스마트 포인터는 자신의 스코프를 벗어나게되면 자동으로 해지되어야합니다. 러스트는 스마트 포인터가 스코프를 벗어나면 Drop 트레이트의 drop 메소드를 호출해서 스마트 포인터가 가리키는 데이터를 해지합니다. 그 외에도 파일을 닫거나하는 자원 정리 기능들을 실행할 수도 있습니다. 마찬가지로 잠시 후에 직접 구현해보겠습니다.

## 데이터를 힙 영역에 저장하는 Box<T>

스마트 포인터 타입이 여러가지가 있습니다만 가장 핵심적인 스마트 포인터는 Box<T>타입입니다. <T>라는 표시는 이 데이터 타입 내부에 저장되는 타입이 제네릭 타입이라는 것입니다. 자신이 원하는 어떤 데이터 타입도 저장할 수 있다는 의미입니다.

Box<T>의 가장 핵심적인 역할은 메모리를 힙 영역에 저장한다는 것입니다. 여기에서 러스트 언어에서 아주 중요한 개념이 나옵니다. 러스트 언어로 개발하기위해서는 데이터가 힙 영역에 저장되느냐 스택 영역에 저장되느냐를 항상 생각해야합니다. 스택과 힙 영역은 똑같은 메모리이지만 아주 중요한 차이점을 가지고 있습니다.

1. 스택 영역: 데이터의 크기를 컴파일 시점에 알 수 있어야합니다. 컴파일러가 함수 호출에 관한 스택 프레임을 만드는 컴파일 시점에 데이터를 저장할 메모리 크기를 계산합니다. 그리고 데이터의 크기는 변할 수 없습니다. 만약 스택에 저장된 데이터의 크기가 늘어난다면 스택 프레임은 망가져버릴 것입니다. 대표적으로 u32같은 기본 타입들은 항상 크기가 고정되어있습니다. 따라서 따로 스택에 저장될 수 있습니다.
2. 힙 영역: 데이터의 크기를 컴파일러가 알 수 없고, 프로그램이 동작하던 중에 결정됩니다. 데이터의 크기가 바뀔 수 있습니다.

이제 Box<T>의 동작 방식을 이해할 수 있습니다.

1. 소유권 관리: Box<T>를 사용하면 데이터의 타입이 무엇이든 상관없이 데이터를 힙 영역에 저장합니다. 그리고 이 데이터를 가리키는 포인터와 메타데이터를 스택에 저장합니다. 순수하게 Box라는 구조체 타입이 있을 것입니다. 이 Box라는 구조체타입의 각 필드는 고정된 크기를 가지는 필드들입니다. 따라서 Box라는 구조체의 객체는 스택에 저장될 수 있습니다. 그리고 T라는 데이터는 힙 영역에 저장합니다. Box는 T의 포인터와 메타데이터를 가지는 것입니다. 그리고 Box타입의 객체의 소유권이 이동하게되고, 최종적으로 Box 객체의 소유권이 사라져서 해지될 때 Drop 트레이트가 사용되어서 T타입의 객체도 같이 해지되는 것입니다. 
2. 메모리 역참조: Box 객체 안에 포인터가 들어있을 것입니다. *라는 역참조 연산자는 사실 포인터 타입에 사용하는 것입니다. Box라는 구조체에 사용하면 아무것도 안됩니다. 하지만 Deref 트레이트를 구현하기 때문에 역참조 연산자를 사용했을 때 Deref 트레이트를 통해서 힙 영역에 있는 데이터를 반환하는 것입니다.

## 기본 사용법

```rust
fn main() {
    let my_data = Box::new(10);
    println("What is in the heap?: {}", my_data);
}
```

&my_data는 스택 주소 반환, 그럼 힙에 있는 데이터의 주소는? my_data.as_ref()
예제 추가

## Box<T>를 직접 구현해보기

Box<T>를 잘 이해하기 위해서 아주 단순한 Box<T>를 직접 만들어보겠습니다. 사실 Box<T>에는 다양한 메소드들이 있지만, 우리는 스마트 포인터의 가장 핵심 기능인 역참조와 자동 메모리 해지만을 구현해보겠습니다.

```rust
use std::ops::{Deref, DerefMut};

struct MySmartPointer<T>(T);

impl<T> MySmartPointer<T> {
    fn new(x: T) -> MySmartPointer<T> {
        MySmartPointer(x)
    }
}

impl<T> Deref for MySmartPointer<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> DerefMut for MySmartPointer<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T> Drop for MySmartPointer<T> {
    fn drop(&mut self) {
        println!("Dropping MySmartPointer");
    }
}

fn main() {
    let my_pointer = MySmartPointer::new(5);
    println!("Value: {}", *my_pointer);
}

```


## 스마트 포인터를 사용하는 경우와 사용하지 않는 경우

When to Use Smart Pointers
Heap Allocation: Use Box<T> when you need to store data on the heap rather than the stack. This is particularly useful for large data structures or when you need a fixed size for a recursive type.
Dynamic Polymorphism: Utilize Box<T> for dynamic dispatch. If you have a trait and you want to store different types that implement this trait, Box<dyn Trait> is your go-to choice.
Shared Ownership: Opt for Rc<T> or Arc<T> (the thread-safe variant of Rc<T>) when data needs to be accessed by multiple owners. These are useful in graph-like data structures or when you need to share data between different parts of a program without a clear single owner.
Interior Mutability: Choose RefCell<T> or Mutex<T>/RwLock<T> (for multithreading scenarios) when you need to modify data even when it's borrowed immutably. This is particularly useful for implementing patterns like the Observer pattern or for working around borrowing rules when you know the borrowing constraints can be safely relaxed.
Custom Smart Pointers: Create your own smart pointers when the standard library’s smart pointers don’t meet your specific requirements, such as specialized memory management strategies, non-standard resource management (like file handles or network connections), or custom reference-counting logic.
When Not to Use Smart Pointers
Stack Allocation Suffices: Avoid using smart pointers for small or short-lived data that can efficiently live on the stack. The overhead of heap allocation and pointer indirection is unnecessary in these cases.
Performance Critical Sections: In performance-sensitive code, the overhead of reference counting in Rc<T>/Arc<T> and the runtime borrow checking of RefCell<T> might be detrimental. In such scenarios, using standard references or other Rust features like lifetimes might be more appropriate.
Exclusive Ownership: If your data has a clear, single owner, and there’s no need for heap allocation, stick to regular references or ownership. Using Box<T> in such cases adds unnecessary overhead.
Concurrency: Avoid Rc<T> and RefCell<T> in concurrent contexts, as they are not thread-safe. Prefer Arc<T>, Mutex<T>, or RwLock<T> in multithreaded environments.
Simple Borrowing Cases: For simple borrowing scenarios where the borrowing rules are easily adhered to, regular references are more suitable. Overusing RefCell<T> or other smart pointers can complicate the code and introduce unnecessary runtime checks.
Performance Implications
Heap Allocation: Smart pointers often involve heap allocation (Box<T>, Rc<T>, Arc<T>). Allocating memory on the heap is generally slower than stack allocation due to the overhead of managing heap memory. This can impact performance, particularly in scenarios with frequent allocations and deallocations.
Indirection and Dereferencing: Smart pointers add a level of indirection. Accessing the data requires dereferencing the pointer, which can be less efficient than direct stack access, especially if done frequently in performance-critical sections of code.
Reference Counting: Rc<T> and Arc<T> manage shared ownership through reference counting. Incrementing and decrementing the reference count involves atomic operations, particularly in Arc<T>, which are thread-safe. These operations can add overhead, especially in multi-threaded contexts where atomic operations are more costly.
Runtime Borrow Checking: RefCell<T> and similar types perform borrow checking at runtime. This adds overhead as it requires runtime checks to enforce borrowing rules, unlike compile-time checks with regular references.
Readability Implications
Clarity of Ownership and Lifetimes: Smart pointers can make ownership and lifetimes explicit, which can be beneficial for readability. For instance, seeing a Box<T> or Rc<T> clearly indicates heap allocation and ownership details.
Complexity in Code: On the flip side, overusing smart pointers or using them inappropriately can lead to code that is harder to follow. For instance, nested smart pointers (Rc<RefCell<T>>) or deep chains of method calls on dereferenced smart pointers can reduce readability.
Explicit Lifetime Management: The explicit management of resources (like the explicit dropping of smart pointers or reference counting) can make code more verbose and harder to read, compared to automatic stack allocation and deallocation.
Conciseness vs. Explicitness: While smart pointers can make some patterns more concise (like shared ownership), they can also lead to more verbose code compared to using simple references. Striking the right balance between conciseness and explicitness is key to maintaining readability.

## 스마트 포인터를 활용하는 예제

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

fn collect_data(items: &mut [Box<dyn GenSerialData>]) {
    for item in items.iter_mut() {
        item.get_input();
    }
}

// &[&dyn GenSerialData] is wrong!
fn generate_serial(items: &[Box<dyn GenSerialData>]) -> String {
    let mut data = String::new();
    for item in items.iter() {
        data.push_str(item.generate().unwrap());
    }
    data
}

fn main() {
    println!("hello");

    let userid = UserID { digit: 4, id: None };
    let product = ProductID { digit: 8, id: None };

    // Vec<&dyn GenSerialData> is wrong!
    let mut items: Vec<Box<dyn GenSerialData>> = vec![Box::new(userid), Box::new(product)];

    collect_data(&mut items);
    let serial = generate_serial(&items);
    println!("Serial generated: {}", serial);
}
```

## 다른 스마트 포인터 타입들

https://medium.com/coinmonks/smart-pointers-made-simple-a-practical-guide-to-rust-f0a2dcb69f52
https://blog.devgenius.io/smart-pointers-in-rust-287974ddfd05
