# 기본 문법

Rust 언어의 기본 문법을 소개하겠습니다. 이 책은 C 언어를 이미 사용해보신 분들을 대상으로 합니다. 변수라는게 뭔지, 함수라는게 뭔지 그런 일반적인 프로그래밍에 대한 설명은 생략하고, Rust 언어에 빠르게 적응하기 위해 필요한 사항들만 소개하겠습니다.

## 함수와 for 루프

FizzBuzz라는 함수를 만들어보겠습니다. 1부터 100까지의 숫자중에 3의 배수를 만나면 Fizz라고 출력하고 5의 배수를 만나면 Buzz라고 출력합니다. 만약에 3과 5의 공배수이면 FizzBuzz라고 출력합니다.

만약에 C만 경험해본 분이라면 아래와 같이 for문에 시작값과 종료 조건등을 생각해보시겠지만, 사실 C언어의 for문은 버그에 굉장히 취약한 형태입니다. "1부터 100까지의 숫자"라는 조건을 보고 다음처럼 생각하기 쉽습니다.

```c
for (i = 1; i < 100; i++)
```

이렇게하면 i가 100일때 처리를 하지 못하는데요 이런식으로 실수하기 쉬운 문법을 가지고 있습니다.

러스트에는 for문에 오직 이터레이터(Iterator)만 사용합니다. 파이선 언어와 유사하기도하고 최신 C++ 문법과도 유사합니다. for와 if만들 사용해서 최대한 C/C++이나 파이선 언어와 유사하게 만들어본 예제입니다.

>
> 예제 코드는 https://github.com/gurugio/my-rust-book에서 다운받을 수 있습니다.
>

```rust
// src/function_for/main.rs
fn fizzbuzz_if_else(max: i32) {
    for i in 1..=max {
        let rem_three: i32 = i % 3;
        let rem_five: i32 = i % 5;

        if rem_three == 0 && rem_five == 0 {
            println!("{} - FizzBuzz", i);
        } else if rem_three == 0 {
            println!("{} - Fizz", i);
        } else if rem_five == 0 {
            println!("{} - Buzz", i);
        } else {
            /* do nothing */
        }
    }
}

fn main() {
    println!("Hello, function_for!");
    fizzbuzz_if_else(10);
}
```

아래와 같이 function_for/main.rs 파일을 빌드하고 실행할 수 있습니다. cargo run 명령은 빌드와 실행을 다 하는 명령입니다.

```bash
$ cargo run --bin function_for
   Compiling my-rust-book v0.1.0 (/home/gkim/study/my-rust-book)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/function_for`
Hello, function_for!
3 - Fizz
5 - Buzz
6 - Fizz
9 - Fizz
10 - Buzz
```

예제 자체는 아주 간단하고 다른 많은 언어들에서 사용되는 문법들과 유사합니다. 예제에 사용된 러스트 문법만 간단히 소개하겠습니다.

프로그램은 main함수에서 시작합니다. 프로그램이 시작된 직후 println!매크로를 호출해서 간단한 메세지를 출력합니다. 그리고 fizzbuzz_if_else함수를 시작합니다.

함수의 정의는 fn 키워드로 시작합니다.

```rust
fn 함수이름(인자: 타입, 인자: 타입, ...) -> 반환값 {
	함수 라인1;
	라인2;
	결과값
}
```

코드의 각 라인은 C/C++과 같이 “;”로 끝나야됩니다. 그리고 마지막에 함수 반환값은 “;”없이 쓰게됩니다. Scala등의 함수형 언어와 유사한 점입니다. 반환값이 없으면 안써주거나 반환할게 없지만 구문의 특성상 반드시 반환값을 지정해야되는 경우 (if-else만 있는 함수같은 경우)에는 ()라고 써주기도 합니다.

for문은 파이썬이나 C++의 for문과 유사합니다. 한가지 주의해야할 점이 있다면 이터레이터(반복자)를 사용하는 구문입니다. 러스트에서는 Range라는 타입으로 for문의 범위를 지정합니다. 파이썬과 유사합니다. 아래와 같이 작성하면 10을 제외한 9까지만 처리하는 코드가 됩니다. Bash나 몇몇 언어에서는 “1..10”이 10을 포함하지만 러스트에서는 10은 포함하지 않습니다.

```rust
for i in 1..10 {
......
}
```

1부터 10까지 처리하도록 하려면 다음과 같이 =를 추가해야합니다.

```rust
for i in 1..=10 {
......
}
```

if문에서 마지막 else는 아무것도 하지 않습니다만 else를 꼭 넣어줘야 완벽하게 모든 케이스를 처리하는 코드가 됩니다. else로 따로 할 일이 없다해도 else를 넣고 아무 처리를 하지 않는다는 코멘트라도 넣어줘야 보안에 신경쓴 코드가 됩니다. 러스트는 이런 보안 헛점들을 방지하기 위한 문법들을 가지고 있습니다. if문에 대한 세부적인 설명은 바로 다음장에서 이야기하겠습니다.

변수의 정의는 let 키워드를 사용합니다.

```rust
let 변수이름: 타입 = 초기값;
```

## if 구문은 값을 가짐

먼저 C로 간단한 if-else문을 사용하는 코드를 만들어보겠습니다.

```c
#include <stdio.h>

int main() {
    int num = 5;
    int var;
    if (num % 3 == 0) {
        var = 3;
    } else if (num % 5 == 0) {
        var = 5;
    } else {
        var = 0;
    }
    printf("var=%d\n", var);
}
```

이 코드를 Rust로 변환하면 다음과 같이 만들 수 있습니다. 미리 주의해야할 것은 Rust에서 if-else문은 값을 가진다는 것입니다.

```rust
// src/if/main.rs
fn main() {
    let num = 5;
    let var = if num % 3 == 0 {
        3
    } else {
        if num % 5 == 0 {
            5
        } else {
            0
        }
    };
    println!("var = {}", var);
}
```

```bash
$ cargo run --bin if_return_value
   Compiling my-rust-book v0.1.0 (/home/gkim/study/my-rust-book)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/if_return_value`
var = 5
```

첫번째 if문 블럭안에 3이 써있는데 3옆에 ;가 없습니다. 3이 { ... }로 둘러싸인 블럭의 값이 된다는 의미입니다. 만약에 num변수에 저장된 값이 3의 배수라면 var에 블럭의 값인 3을 저장하게됩니다.

C에 익숙한 분들은 아마 아래와 같은 3항 연산자가 생각나실 겁니다.

```c
#include <stdio.h>

int main() {
    int num = 5;
    int var = num % 3 == 0 ? 3 : num % 5 == 0 ? 5 : 0;
    printf("var=%d\n", var);
}
```

C언어에서 위와 같은 3항 연산자는 구문 자체가 값을 반환하게 됩니다. C언어에서 if문이 러스트와 같이 반환값을 가질 수는 없지만 3항 연산자를 사용하면 러스트와 같이 좀더 짧고 에러 처리가 확실한 코드가 될 수 있습니다.

물론 러스트도 아래와 같이 처음에 만든 C코드와 완전히 동일하게 구현할 수 있지만 추천하는 방식은 아닙니다.

```rust
fn main() {
    let num = 5;
    let var;
    if num % 3 == 0 {
        var = 3;
    } else if num % 5 == 0 {
        var = 5;
    } else {
        var = 0;
    }

    println!("var = {}", var);
}
```

왜 굳이 if문이 값을 가지도록 만들었을 까요? 프로그래밍 언어의 철학에 대해서 본격적으로 이야기하자면 저도 잘 모르는 깊은 이론들이 있겠지만, 제가 개발하면서 느꼈던 장점은 값이 초기화되지 않은 변수를 최소화할 수 있다는 것입니다. 초기화되지 않은 변수를 일부러 만드는 사람은 없을 것입니다. 하지만 코드가 길어지고 다른 사람의 코드를 유지보수하다보면 실수로 변수의 초기화 코드를 제거하기도 합니다. 그리고 아주 심각한 문제는 초기화가 안된 변수를 알아차리지 못하고 계속 사용하는 경우가 생길 수 있다는 것입니다.

```c
#include <stdio.h>

int main() {
    int num = 5;
    int var;
    if (num % 3 == 0) {
        var = 3;
    } else if (num % 5 == 0) {
        printf("This is error, please enter 3!\n");
    } else {
        var = 0;
    }
    printf("var=%d\n", var);
}
```

위 코드는 var변수를 초기화하는 것을 깜빡한 것입니다. if-else가 한두개이거나 처음 코딩할때는 초기화를 깜빡하지않겠지만, 개발이 점점 진행되고 코드가 길어지고 복잡해지면 초기화를 깜빡하는 경우가 자주 생깁니다. 운이 좋으면 그냥 쓰레기값이 출력되는 것으로 끝나겠지만, 대부분 의미없는 값이 이리저리 돌아다기나가 엉뚱한 곳에서 패닉을 발생시키고 디버깅하는데 며칠이 걸리게 만드는 경험을 해보셨을 것입니다.

러스트에서 if-else로 변수를 초기화하면 다음과 같이 값을 반환하지 않는 경우를 방지할 수 있습니다.

```rust
fn main() {
    let num = 5;
    let var = if num % 3 == 0 {
        3
    } else {
        if num % 5 == 0 {
            //5 missing by mistake
            println!("Wrong number");
        } else {
            0
        }
    };
    println!("var = {}", var);
}
```

```bash
error[E0308]: `if` and `else` have incompatible types
  --> src/main.rs:10:13
   |
6  | /         if num % 5 == 0 {
7  | |             //5 missing by mistake
8  | |             println!("Wrong number");
   | |             ------------------------ expected because of this
9  | |         } else {
10 | |             0
   | |             ^ expected `()`, found integer
11 | |         }
   | |_________- `if` and `else` have incompatible types
```

추가로 한가지 팁을 드리자면 러스트는 다양한 함수형 프로그래밍 기법을 사용하고 있어서 if-else를 사용할 일이 많이 줄어듭니다. C언어에서 if-else를 사용하는 처리를 Rust에서는 match나 map, filter등을 사용하는 경우가 훨씬 많습니다. 만약에 내 코드에 if-else가 많이 보인다면 Rust다운 코드를 만들지 못하고있는게 아닌가 자문해봐야합니다.

## Mutable 변수와 타입 추론

지금까지 우리가 만들어본 예제들은 변수가 생성될 때 값을 정하고 그 후에는 값을 바꾸지 않았었습니다.

```rust
fn fizzbuzz_if_else(max: i32) {
    for i in 1..=max {
        let rem_three: i32 = i % 3;
        let rem_five: i32 = i % 5;

        if rem_three == 0 && rem_five == 0 {
            println!("{} - FizzBuzz", i);
        } else if rem_three == 0 {
            println!("{} - Fizz", i);
        } else if rem_five == 0 {
            println!("{} - Buzz", i);
        } else {
						/* do nothing */
				}
    }
}
```

위 예제를 보면 3개의 변수가 있습니다. “let”으로 생성한 변수가 2개, rem_three, rem_five가 있고, 하나는 max라는 함수 인자입니다. 이렇게 총 3개의 변수가 하나의 함수를 이루는 컨텍스트에서 생성되었습니다.

함수의 컨텍스트는 간단하게 이해하자면 함수가 동작하기 위한 모든 메모리, 코드 등을 합쳐서 부르는 말입니다. 로컬 변수는 함수의 스택에 저장되고, 함수 인자는 함수를 호출하는 상위 함수의 메모리에 있겠지요. 그리고 함수 자체를 이루는 코드도 있을 것입니다. 이런 것들은 컨텍스트라고 생각할 수 있습니다.

어쩌면 지금까지의 예제 코드를 보면서 모든 변수들이 값이 고정된 변수들이었다는 것을 알아차리지 못한 분들도 계셨을 것입니다. 그게 바로 러스트가 변수 선언의 기본을 변하지않는 속성(Immutable)으로 정한 이유입니다. 생각외로 많은 변수들이 한번 초기화되면 값이 바뀌지 않습니다. 특히 함수형으로 코드를 작성하면 더욱 그런 경우가 많아지고요 그리고 함수들을 잘 분리해서 여러 함수들이 간결하게 작성되면 함수의 로컬 변수들의 값이 자주 바뀌지 않는 경우가 많습니다.

어쨌든 값이 변하는 변수를 만들고자할 때는 추가로 mut라는 키워드를 넣어주어야 합니다. 이것은 let으로 변수를 선언할 때 뿐 아니라 함수 인자에서도 마찬가지입니다.

mut 키워드를 사용하는 예제를 보겠습니다.

```rust
// src/mutable_var/main.rs
fn fib(mut index: i32) -> i32 {
    let mut a = 1;
    let mut b = 1;
    let mut t;

    loop {
        t = a + b;
        a = b;
        b = t;

        index -= 1;
        if index <= 0 {
            break;
        }
    }
    b
}

fn main() {
    println!("{}", fib(10));
}
```

```bash
$ cargo run --bin mutable_var
   Compiling my-rust-book v0.1.0 (/home/gkim/study/my-rust-book)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/mutable_var`
144
```

이 예제에서 3가지 새로운 컨셉을 소개하고 있습니다.

1. mut키워드를 사용해서 값이 변하는 변수와 함수 인자를 사용
2. loop 사용방법
3. 각 변수마다 일일이 타입을 안써줘도 러스트가 타입을 추론할 수 있음 (덕타이핑 Duck Typing이라고 부릅니다)

mut키워드는 mutable의 약자입니다. 값이 변하는 변수라는 표시인데요 보통의 프로그래밍 언어들이 값이 안변하는 상수를 위한 키워드가 있는 것에 비하면 반대로 러스트에서는 값이 안변하는 변수가 기본이고, 값이 변하는 변수가 mut 키워드를 추가해줘야합니다. 실제로 러스트로 프로그래밍하다보면 의외로 자주 mut 키워드를 빼먹어서 컴파일 에러가 나는 경우를 겪을 것입니다. 그런데 생각보다 더 많은 경우에 mut 키워드를 무의식적으로 빼먹고도 문제없이 동작하는 것을 겪을 것입니다. 저도 처음에는 mut 키워드를 써줘야한다는게 번거로울것 같았지만, 곧 제 자신이 값이 변하지 않는 변수를 생각보다 많이 써왔다는 것을 깨달았습니다. 그리고 좀 더 경험이 쌓일 수록 값이 바뀌지 않는 변수를 많이 사용하게되고 결국 코드가 조금 더 간결해지는 것을 경험하게 되었습니다. 물론 러스트가 가진 함수형 프로그래밍 기능들을 사용해서 그렇기도 합니다만 지금은 값이 변하지 않는 변수가 기본이라는 것이 코드의 안정성을 위해서 괸찬은 선택이었다고 생각힙다.

loop 사용법은 저 개인적으로 타 언어의 while 보다 더 편리하다고 생각합니다. while을 사용하다보면 while(..)의 ()안에 탈출 표현식을 써주고 또 while() {..} 바디안에 또 다른 탈출식을 써주게 되는 경우가 많습니다. 이것보다는 loop처럼 기본 탈출식이 없는게 좀 더 읽기 편하다고 생각합니다.

러스트는 타입 추론 기능을 가지고 있습니다. C++에서는 auto라는 타입을 써주면 컴파일러가 최적의 타입을 찾아주는데 러스트는 아예 타입을 안써줄 수도 있습니다. 러스트는 강타입 언어이므로 타입이 분명 존재합니다만, 타입을 안써주면 컴파일러 코드에서 변수의 사용을 보고 타입을 지정해줍니다. 이게 왜 편리하냐면 러스트의 문법 특성 상 타입이 아주 긴 경우가 자주있습니다.

아래와 같은 타입을 변수 타입에 써야한다면 번거로울 수밖에 없겠지요.

```rust
Box<dyn Fn() + Send + 'static>
Arc<Vec<Box<dyn Collector<dyn CollectorModel> + 'static>>>,
```
출처 link: https://doc.rust-lang.org/book/ch19-04-advanced-types.html

참고로 함수를 정의할 때는 타입을 생략할 수 없습니다. 함수를 호출하기위해서는 입출력의 타입을 정확하게 알아야 컴파일러가 기계 코드를 생성할 수 있기 때문입니다.

## as를 이용한 타입 바꾸기

사실 C 언어가 가진 큰 단점중에 하나가 타입이 암묵적으로 바뀐다는 것입니다. C++은 C보다는 좀 더 엄격하게 타입이 바뀌는 것을 방지했지만, 그래도 대부분의 현대 언어들에 비하면 많이 느슨한 편입니다.

하지만 러스트는 타입 변환을 항상 명확하게 선언해주어야 합니다.

다음 예제 코드는 문자열로 된 숫자를 정수로 반환하는 함수를 C언어와 유사하게 만들어본 것입니다.

```rust
fn string_to_digit(input: String) -> i32 {
    let mut ret = 0;
    for c in input.chars() {
        ret = ret * 10;
        ret += c - '0'; // 컴파일 에러 발생
    }
    ret
}
```
```bash
error[E0369]: cannot subtract `char` from `char`
  --> src/main.rs:10:19
   |
10 |         ret += c - '0'; // 컴파일 에러 발생
   |                - ^ --- char
   |                |
   |                char
```

빌드해보면 char타입 변수에서 char타입의 값을 뺄 수 없다는 에러가 발생합니다. 아래는 임시방편으로 char타입의 변수를 i32타입의 정수로 변환한 후 빼기 연산을 수행한 코드입니다.

```rust
fn string_to_digit(input: String) -> i32 {
    let mut ret = 0;
    for c in input.chars() {
        ret = ret * 10;
        ret += c as i32 - '0' as i32;
    }
    ret
}
```
이 코드는 컴파일 에러도 없고 잘 동작합니다.

위 예제를 통해 우리는 for문에서 생성되는 c라는 변수가 char라는 타입인지 알 수 있습니다. 러스트에서 for문에는 항상 이터레이터만 사용할 수 있다고 설명했는데, String이라는 구조체의 chars라는 메소드가 문자열의 각 문자들을 반환하는 이터레이터를 생성해주는 메소드라고 생각하면 됩니다.

러스트에서는 char 타입의 변수 c에서 char 타입의 문자 ‘0’를 뺄 수는 없습니다. 현대 언어들에 익숙한 분들에게는 당연할 수도 있는 사항인데요, C 계열 언어에 익숙한 분들에게는 당황스러운 것일 수도 있습니다.

C 언어는 사실 어셈블리로 개발하던 프로젝트의 생산성을 높이기 위해 나온 언어입니다. 따라서 어셈블리가 익숙한 개발자들의 관점에서 디자인된 언어이다보니 모든게 숫자입니다. 참과 거짓도 숫자이고, 오류를 나타내는 NULL이나 에러 값도 숫자이고, 포인터도 숫자, char라는 타입도 숫자입니다. 그러니 타입이 다른 변수들간에도 더하기 빼기가 가능합니다. 하지만 이게 논리적으로 좋은 디자인이라는건 의문이 남습니다. 숫자가 아닌 타입을 더하거나 뺀다는건 논리적으로는 잘못된 연산이 되는게 더 옳지 않을까요.

잡설이 좀 길었지만 어쨌든 러스트에서는 타입의 변환을 as라는 키워드로 합니다. 추후에 몇가지 타입 변환과 관련된 키워드를 보겠지만 가장 기본적인 것은 바로 as 입니다. 러스트도 사실은 C 계열 개발자들의 생산성을 높이기 위해 나온 언어입니다. 이렇게 언어 자체에 키워드가 있어서 타입이 변환된다는 것도 논리적으로 옳은건지는 모르겠습니다만 Syntax sugar라고 생각해도 될듯합니다.

참고로 C언어에 atoi 라이브러리 함수가 있는 것처럼 러스트에도 이미 같은 일을 하는 parse 메소드가 있습니다.

```rust
// src/as/main.rs
use std::num::ParseIntError;

fn string_to_digit(input: String) -> i32 {
    let mut ret = 0;
    for c in input.chars() {
        ret = ret * 10;
        ret += c as i32 - '0' as i32;
    }
    ret
}

fn parse_example(input: &str) -> Result<i32, ParseIntError> {
    input.parse()
}

fn main() {
    println!("{}", string_to_digit("1234".to_string()));
    let ret = parse_example("1234");
    match ret {
        Ok(value) => {
            println!("Parsed integer: {}", value);
        }
        Err(_) => {
            println!("Failed to parse the string as an integer");
        }
    }
}
```

```bash
$ cargo run --bin as
   Compiling my-rust-book v0.1.0 (/home/gkim/study/my-rust-book)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/as`
1234
Parsed integer: 1234
```

string_to_digit 함수의 문제점은 문자열이 숫자 외의 문자를 가지고있는 등의 에러 상황에서 에러 값을 반환할 수가 없다는 것입니다. 에러 값을 반환하기 위해서 함수 인자에 결과값에 대한 포인터를 전달하거나 하면 코드가 복잡해지게됩니다. C 언어의 디자인 단계에서부터 실수한 것이 바로 에러 값에 대한 처리가 없다는 것입니다.

러스트는 에러 값으로 Result나 Option같은 타입을 사용하게 됩니다. 추후에 좀 더 자세히 알아볼 것인데 미리 이런 상황에서 필요하다는 것을 알고 넘어가면 나중에 좀 더 이해하기 쉬울것 같아서 예제를 만들어보았습니다.

## match를 이용한 패턴 매칭

저는 for, while, if, 함수호출 등 러스트의 기본 문법들에 대해서는 C나 최신 함수형 언어들에서 봐왔던 것들이라 금방 익숙해질 수 있습니다. 그러다가 처음으로 흥미를 느낀 문법이 바로 패턴 매칭 부분이었습니다. if-else가 여러개 있는 케이스는 C에서도 switch문으로 작성하기 때문에 별다른게 있을까 생각했었는데 패턴 매칭이라는 것의 의미를 이해하게되면서 감탄했던 기억이 있습니다.

패턴 매칭은 사실 정확한 정의가 무었이기를 따지기 보다는 쓰다보면서 적응해나가는게 더 효율적인 접근방법이라고 생각합니다. 일단 가장 쉬운 예제를 하나 보겠습니다.
```rust
// src/match/main.rs
fn fizzbuzz_2(max: i32) {
    for i in 1..=max {
        match (i % 3, i % 5) {
            (0, 0) => println!("{} - FizzBuzz", i),
            (0, _) => println!("{} - Fizz", i),
            (_, 0) => println!("{} - Buzz", i),
            (_, _) => (),
        }
    }
}

fn main() {
    fizzbuzz_2(15);

    let age = 44;
    let gen = match age {
        0..=20 => "MZ",
        21..=50 => "X",
        51..=100 => "A",
        _ => "?",
    };
    println!("generation={}", gen);

    for i in 1..=30 {
        let msg = match i {
            n if n % 15 == 0 => format!("{} - FizzBizz", n),
            n if n % 3 == 0 => format!("{} - Fizz", n),
            n if n % 5 == 0 => format!("{} - Buzz", n),
            _ => format!("{}", i),
        };
        println!("{}", msg);
    }
}
```

```bash
gkim@gkim-laptop:~/study/my-rust-book$ cargo run --bin match
   Compiling my-rust-book v0.1.0 (/home/gkim/study/my-rust-book)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.17s
     Running `target/debug/match`
3 - Fizz
5 - Buzz
6 - Fizz
9 - Fizz
10 - Buzz
12 - Fizz
15 - FizzBuzz
generation=X
1
2
3 - Fizz
4
5 - Buzz
6 - Fizz
7
8
9 - Fizz
10 - Buzz
11
12 - Fizz
13
14
15 - FizzBizz
16
17
18 - Fizz
19
20 - Buzz
21 - Fizz
22
23
24 - Fizz
25 - Buzz
26
27 - Fizz
28
29
30 - FizzBizz
```

fizzbuzz_2함수는 이전에 만든 fizzbuzz 예제를 패턴 매칭으로 바꾼 함수입니다. match라는 키워드 다음에 있는 (i % 3, i % 5) 는 하나의 튜플 Tuples를 만들었습니다. 이 튜플의 값이 match다음에 나오는 각 패턴에 해당될 때 각기 다른 메세지를 출력하도록 만든 것입니다. 패턴에서 언더바 _ 는 모든 값을 의미합니다. 튜플의 값을 비교해야하는데 두개의 값이 있어야 하므로 하나는 0이고 다른 값은 무엇이든 상관없을 때 언더바를 사용한 것입니다. 패턴 매칭이므로 2개의 값이 있는 튜플의 패턴과 매칭이 되려면 각각의 매칭 케이스마다 2개의 값이 있게됩니다.

if-else를 쓸 때와 마찬가지로 패턴의 비교 순서가 바뀌면 전혀 다른 결과를 만들어냅니다. 만약에 아래처럼 패턴 순서를 바꿨다면 어떤 일이 벌어질까요?

```rust
fn fizzbuzz_2(max: i32) {
    for i in 1..=max {
        match (i % 3, i % 5) {
            (0, _) => println!("{} - Fizz", i),
            (_, 0) => println!("{} - Buzz", i),
            (0, 0) => println!("{} - FizzBuzz", i),
            (_, _) => (),
        }
    }
}
```

만약 튜플 값이 (0, 0)이면 FizzBuzz를 출력하는게 아니라 Fizz를 출력할 것입니다.

흔하게 사용하는 패턴을 하나 더 보겠습니다. main함수안에 다음과 같이 어떤 변수의 값이 어느 범위에 속하는지 판단하는 코드가 있습니다.

```rust
let gen = match age {
    0..=20 => "MZ",
    21..=50 => "X",
    51..=100 => "A",
    _ => "?",
};
```

age라는 변수를 0..20, 21..=50, 51=100이라는 3가지의 Range타입과 비교하는 것인데, 세밀하게따지면 age가 Range타입이 아니므로 패턴으로 매칭이 되는게 조금 이상할 수도 있습니다. 문법적으로 세세하게 따지면 복잡할 수도 있는 코드입니다만 지금은 이런 것도 가능하다 정도로 생각하고 일단 자주 활용하면서 익숙해지면 될듯 합니다. 단순히 age라는 변수의 범위만 따져서 어떤 동작을 수행하는게 아니라 각 경우에 따른 반환 값을 gen이라는 변수를 생성하는데 사용한 것을 주의해서 보시기 바랍니다. match라는 구문도 결국 if와 같이 반환값을 가지게됩니다.

단순히 패턴에 일치하는 것만 확인하는게 아니라 아래와 같이 if와 결합해서 조건식을 써줄 수도 있습니다. 

```rust
for i in 1..=30 {
    let msg = match i {
        n if n % 15 == 0 => format!("{} - FizzBizz", n),
        n if n % 3 == 0 => format!("{} - Fizz", n),
        n if n % 5 == 0 => format!("{} - Buzz", n),
        _ => format!("{}", i),
    };
    println!("{}", msg);
};
```

format!()은 문자열 객체를 반환해주는 매크로 함수입니다. 조건식에 따라 다른 문자열을 생성해서 msg라는 변수에 저장하게됩니다.

러스트 관련 소개 자료나 공식 문서등을 보면 러스트의 패턴 매칭은 강력하다라는 설명이 있습니다. 보통 강력하다라는 말은 다양한 방식으로 사용된다는 의미입니다. 이 책에서 모든 케이스를 일일이 보여드릴 수는 없지만 러스트로 개발하면서 조금씩 시도를 하다보면 이런것도 되네 하면서 감탄하는 경험을 하게되실 것입니다.

## 값을 가지는 표현식

지금까지 if-else나 패턴 매칭의 예제를 보면 변수를 선언하는 부분에 복잡한 코드를 넣은 것을 볼 수 있었습니다. 이런 표현식에 대해서 좀 더 자세히 이야기해보겠습니다. 다음 예제는 지금까지 나온 값을 반환하는 표현식들을 모아놓은 것입니다.

참고로 아래 예제를 실행해보면 ret_zero라는 함수를 호출하고있지 않다는 경고 메세지가 나옵니다. 에러는 아니므로 코드에 문제는 없습니다. _var같은 변수도 마찬가지로 생성만하고 사용하지않습니다만 경고 메세지가 없습니다. 변수 이름을 _로 시작했기 때문입니다. 이와같이 임시로 만들어놓고 나중에 사용하게될 변수는 이름을 _로 시작하면 컴파일을 깔끔하게 할 수 있습니다. ret_zero 함수의 이름에도 앞에 _를 붙여보면 경고 메세지가 사라지는 것을 볼 수 있습니다.

```rust
// src/expr/main.rs
fn ret_zero() -> i32 {
    0
}

fn main() {
    let age = 44;
    let gen = match age {
        0..=20 => "MZ",
        21..=50 => "X",
        51..=100 => "A",
        _ => "?",
    };
    println!("generation={}", gen);

    let num = 45;
    let _var = if num % 3 == 0 {
        3
    } else {
        if num % 5 == 0 {
            5
        } else {
            0
        }
    };

    let x = 9;
    let _y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // This expression will be assigned to `y`
        x_cube + x_squared + x
    };
}
```

```bash
$ cargo run --bin expr
   Compiling my-rust-book v0.1.0 (/home/gkim/study/my-rust-book)
warning: function `ret_zero` is never used
 --> src/expr/main.rs:1:4
  |
1 | fn ret_zero() -> i32 {
  |    ^^^^^^^^
  |
  = note: `#[warn(dead_code)]` on by default

warning: `my-rust-book` (bin "expr") generated 1 warning
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.16s
     Running `target/debug/expr`
generation=X
```

위 예제의 match를 이용한 패턴 매칭 구문에서 { 로 시작하고 } 로 끝나는 하나의 블럭을 표현식Expressions 라고 합니다. 마찬가지로 if-else도 하나의 표현식입니다.

그리고 표현식은 값을 반환합니다.

값을 반환하는건 또 무엇이 있나요? 함수가 있습니다. 함수도 하나의 표현식입니다.

```rust
fn ret_zero() -> i32 {
    0
}
```

함수도 {와 }로 시작과 끝을 나타내고 반환값을 마지막에 써놓은 표현식입니다. 위 예제의 if-else도 각 {} 블럭 안에 반환값이 정해져있습니다. 그리고 match 구문에서도 각 상황에 따른 반환값이 있습니다. 이런 표현식은 ;를 만나면 그 반환값이 무시됩니다. 

반환값을 가지는 표현식을 이용해서 아래와 같은 변수 초기화 코드가 만들어질 수 있게됩니다. 중간중간에 있는 코드들은 ;로 끝나므로 반환값이 없고 마지막에있는 코드에만 ;이 없으므로 마지막줄에 있는 수식의 결과값이 y의 값이 됩니다.

```rust
let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // This expression will be assigned to `y`
        x_cube + x_squared + x
    };
```

y라는 변수의 값을 계산하기 위해 복잡한 코드가 나열되는게 아니라 y의 선언부에 모여있을 수 있습니다. 물론 위에서 본것과 같이 match, if-else등 도 올 수 있습니다. 다른 언어와 마찬가지로 당연히 함수 호출도 올 수 있겠지요.

참고로 영어로 표현식은 Expression이고 표현식에 ;가 붙은 것을 Statement라고 구분해서 부르는 경우도 있습니다.

## 배열과 배열을 참조하는 슬라이스

문자열을 사용하는 예제를 몇개 봤으니 우선 배열과 슬라이스에 대해서 알아본 후 본격적으로 문자열(사실은 문자열이라기 보다는 String 타입의 객체가 정확한 표현이긴 합니다)을 알아보겠습니다.

배열의 정의는 같은 타입의 데이터가 메모리에 연속적으로 나열된 데이터 구조를 말합니다.

```rust
let numbers: [i32; 5] = [1, 2, 3, 4, 5]; // Example array of numbers
```

위와 같이 i32이라는 같은 타입의 데이터가 연속적으로 5개가 메모리에 연속적으로 저장되어있습니다. 메모리에 연속적으로 위치하고있기 때문에 우리는 numbers[0] 다음에 있는 데이터가 numbers[0]의 위치(포인터)에서 i32타입의 크기, 4만큼 더한 곳에 위치한다는 것을 알 수 있습니다.

```
numbers[0] ==> 0x100
numbers[1] ==> 0x100 + 1 * 4 = 0x104
...
numbers[i] ==> 0x100 + i * 4
```

그래서 배열의 인덱스 [0], [1]을 가지고 빠르게 각 데이터에 접근할 수 있다는 것이 배열의 특징입니다. 연결 리스트나 트리 등보다 빠른 접근이 가능합니다.

슬라이스는 이런 배열의 일부(혹은 전체)만을 접근하려고 만든 레퍼런스 타입입니다.

```rust
let slice: &[i32] = &numbers[1..4]; // Create a slice from index 1 to 3 (inclusive)
```

위의 슬라이스는 numbers라는 배열의 1,2,3번 데이터에만 접근할 수 있습니다. [i32]는 i32타입의 배열에 접근한다는 표시히고 참조 연산자 &를 써서 배열에 대한 참조라는 것을 나타냅니다. 내부적으로는 배열의 크기도 저장하고 있습니다. 그래서 참조하고있는 배열의 3개 데이터만을 참조한다는 것도 내부적으로 저장하고 있습니다.

여기서 배열이나 구조체같은 여러 데이터가 같이 묶여있는 타입의 디버깅을 위해 한가지 좋은 방법을 소개하겠습니다.

```rust
fn main() {
    let numbers: [i32; 5] = [1, 2, 3, 4, 5]; // Example array of numbers

    let slice: &[i32] = &numbers[1..4]; // Create a slice from index 1 to 3 (inclusive)

    println!("Array: {:?}", numbers);
    println!("Slice: {:?}", slice);
}
```

“{:?}”라는 출력 포맷을 사용하면 아래와 같이 배열의 데이터를 전부 출력해줍니다.

```rust
Array: [1, 2, 3, 4, 5]
Slice: [2, 3, 4]
```

참고로 나중에 배울 구조체도 같은 방법으로 "{:?}"을 이용해서 구조체의 각 데이터 필드를 출력할 수 있으니, 디버깅을 위해서는 유용하게 사용해보시기 바랍니다.

실행 결과를 보면 슬라이스를 통해 3개의 데이터만 접근할 수 있는 것을 알 수 있습니다. 배열의 참조이지만, 슬라이스라는 타입 내부에는 이 슬라이스가 몇개의 데이터를 참조할 수 있는지에 대한 정보가 같이 저장되어있다고 보시면 됩니다. 그래서 슬라이스의 데이터를 전부 출력해보면 3개만 출력되는 것입니다.

슬라이스가 뭔지는 알았는데 그럼 슬라이스가 왜 필요한지가 의문일 수 있습니다. 왜냐면 C언어에는 슬라이스가 없기 때문입니다. 배열이나 구조체가 존재하는 메모리를 읽다가 데이터가 저장된 영역을 넘어서 그 다음 메모리까지 읽는 것을 방지할 수 없는게  C언어입니다. numbers[4]까지만 읽어야되는데 numbers[5]를 읽거나 쓰려는 실수를 누구나 해본적이 있을 것입니다.

슬라이스는 바로 이런 실수를 방지하려고 있는 것입니다. 함수나 쓰레드를 호출 할 때, 그 함수나 쓰레드가 배열의 일부분에만 접근해야한다고 할 때, 배열 전체를 전달하는게 아니라 슬라이스를 전달하게됩니다.

위에서 슬라이스는 배열의 일부 혹은 전체에 접근하기 위한 타입이라고 설명했습니다. 다음 예제를 보겠습니다.

```rust
// src/array_slice/main.rs
fn sum_array_ref(nums: &[i32]) -> i32 {
    let mut s = 0;
    let len = nums.len();
    let mut index = 0;
    loop {
        if index >= len {
            return s;
        }
        s += nums[index];
        index += 1;
    }
}

fn sum_slice(nums: &[i32]) -> i32 {
    let mut s = 0;
    for i in nums.iter() {
        s += i;
    }
    s
}

fn main() {
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    let slice = &numbers[1..4];

    println!("Array: {:?}", numbers);
    println!("Slice: {:?}", slice);

    println!("{}", sum_array_ref(&numbers));
    println!("{}", sum_slice(slice));
}
``` 

```bash
$ cargo run --bin array_slice
   Compiling my-rust-book v0.1.0 (/home/gkim/study/my-rust-book)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.16s
     Running `target/debug/array_slice`
Array: [1, 2, 3, 4, 5]
Slice: [2, 3, 4]
15
9
```

sum_array_ref는 배열을 참조로 전달받는 함수이고,  sum_slice는 슬라이스를 전달받는 함수입니다. 슬라이스가 배열의 참조이므로 형태가 동일합니다. 각 함수는 자신이 전달받은 데이터가 배열의 참조인지, 배열의 일부분을 가르키는 슬라이스인지 알 필요가 없습니다. 두개가 완전히 동일한 것이기 때문입니다.

배열이나 문자열을 처리하는 함수를 만들 때는 항상 슬라이스로 인자를 받도록 만드는 습관을 가지면 좋습니다.

물론 슬라이스같은 참조가 아니라 배열이나 문자열을 그대로 전달하면 소유권까지 함수로 넘어가게되서 함수를 호출한 코드에서 다시는 배열이나 문자열을 접근할 수 없게되기 때문에, 그렇게 슬라이스로 처리를 할 수 밖에 없습니다. 러스트의 소유권 개념은 앞으로 좀 더 자세히 이야기하겠습니다만, 러스트는 이렇게 사용자가 데이터를 전달하면서 실수할 수 있는 부분들을 최대한 막으려는 디자인 철학을 가진 언어입니다. C의 문제점들을 해결하려고 C++에서 수십차례 버전을 올려가며 규약들을 만들고, 스마트 포인터를 만드는 등의 노력을 했었지만, 근본적으로 언어의 철학 자체가 개발자가 모든 것을 직접 처리할 수 있어야 한다는 철학을 밑바탕에 가지고 있기 때문에 완전히 막을 수 없는 헛점들이 있었습니다. 러스트는 그런 C++의 최신 기법들을 모두 모아놓고, 강제로 쓰도록 정해놓은 언어라고 생각하시면 이해하는데 도움이 될것 같습니다.

## 문자열을 저장하는 String타입

C같은 언어에서는 문자열은 char 타입의 배열입니다. char타입은 8비트 부호없는 정수를 의미합니다. 결국 바이트의 배열을 의미하게됩니다. 하지만 러스트를 비롯한 고급언어들은 문자열은 String이라는 구조체나 새로운 타입으로 표현합니다. 단순히 1바이트로 표현할 수 있는 ASCII 코드의 배열이 아니라, UTF-8 코드를 사용해서 각 문자를 관리하기 때문입니다.

그럼에도 러스트에서 슬라이스와 함께 문자열을 설명하는 경우가 많은데 그 이유를 알아보겠습니다.

우선 String(https://doc.rust-lang.org/std/string/struct.String.html)이라는 타입에 대해서 알아보겠습니다.

메뉴얼을 자세히 볼 필요는 없지만 첫줄만 봐도 결국 하나의 구조체라는 것을 알 수 있습니다. C에서와같이 문자의 배열은 아닙니다. 따라서 String을 사용하기 위해서는 우선 String타입의 객체를 생성해야합니다. 다음 짧은 예제에는 몇가지 흔하게 사용되는 String 생성 방법들을 모아봤습니다. 참고로 String은 러스트의 “The Rust Standard  Library”(약자로 std)에 포함되기 때문에 명시적으로 지정해주지않아도 자동으로 빌드에 포함됩니다. C의 include나 파이썬의 import등과 같은 추가적인 절차없이 바로 사용할 수 있습니다. 

String을 사용하는 몇가지 방식들에 대한 예제를 보면서 이야기하겠습니다.

```rust
// src/string/main.rs
fn get_moved_string(data: &str) {
    println!("{}", data);
}

fn main() {
    let _hello = String::from("Hello, world!");
    let mut _s = String::new();
    let _s = "initial contents".to_string();
    let _hello = String::from("안녕하세요");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let _s = format!("{}-{}-{}", s1, s2, s3);

    let moving_string = String::from("hello");

    get_moved_string(&moving_string);
    println!("{}", moving_string);

    let mut mutable_string = String::from("hello");
    mutable_string.push_str(" world");
    println!("{}", mutable_string.chars().nth(0).unwrap());

    {
        let hello = "hell".to_string();
        let _r1 = &hello;
        //let mut r2 = &mut hello; // Build Error!!!
    }
}
```
```bash
gkim@gkim-laptop:~/study/my-rust-book$ cargo run --bin string
   Compiling my-rust-book v0.1.0 (/home/gkim/study/my-rust-book)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.20s
     Running `target/debug/string`
hello
hello
h
```

가장 먼저 주의해야할 것은 “이렇게 큰따옴표안에있는 문자열”, 즉 리터럴(Literal)은 String이 아니라는 것입니다. 리터럴은 컴파일 시점에 고정된 크기의 데이터입니다. 리터럴은 소스 코드에 하드 코딩되었으므로 프로그램의 실행중에 데이터가 변하거나 크기가 바뀌는 일이 없습니다. 따라서 컴파일러는 힙 영역에 메모리를 할당해서 리터럴을 저장하는게 아니라 프로그램 코드가 저장되는 영역 (프로그램 프로그램의 코드가 저장된 메모리 영역)에 저장합니다. 리터럴과 String이라는 타입의 객체는 다른 것입니다. String타입의 객체는 프로세스가 실행 중에 프로세스의 힙 영역에 메모리를 할당하고, 할당된 메모리에 리터럴 데이터를 복사해서 생성합니다. 예제 코드에있는 from이라는 정적 메소드나 to_string이라는 메소드가 실행하는 리터럴을 String 객체의 메모리에 복사합니다.

각 메소드들을 설명하자면 다음과 같습니다.

- from(”msg”): “와 “안에 들어가는 리터럴을 이용해서 String 객체를 생성함
- new(): 아무런 데이터가 없는 String 객체 생성
- “msg”.to_string(): “와 “안에 있는 문자열을 String객체로 생성함, from과 같음

그리고 또 하나 흔하게 String을 만드는 방법이 다음과 같이 format! 매크로 함수를 이용한 방식입니다.

```rust
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

let s = format!("{}-{}-{}", s1, s2, s3);
```

String은 워낙 많이 사용되는 타입이므로 다양한 기능의 메소드들이 있습니다. 파이썬이나 자바등 고급 언어를 다뤄본 분이라면 이미 익숙한 메소드들이 많을 것입니다. push_str나 insert, len 등등 대부분의 언어들과 마찬가지로 String객체에 추가로 문자열을 넣거나 길이를 반환하거나 하는 메소드들이 있습니다.

그럼 “이렇게 큰따옴표 안에 들어간 문자열”은 무엇이고, String과의 관계는 어떻게 되는 걸까요?

String 구조체의 정의부터 찾아보겠습니다.

```rust
pub struct String {
    vec: Vec<u8>,
}
```

출처: https://doc.rust-lang.org/src/alloc/string.rs.html#365

고급 언어를 다뤄봤다면 반드시 사용해봤을만한 벡터가 나타났습니다. 벡터는 기본적으로 배열이지만, 데이터의 크기가 동적으로 커지거나 작아질 수 있는 기능을 추가한 타입입니다. 배열은 생성시에 지정된 크기만을 갖지만 벡터를 사용하면 데이터를 추가하면 추가하는만큼 데이터의 크기가 커지고, 데이터를 지우면 지워진만큼 크기가 작아집니다. 벡터에 대한 보다 자세한 설명은 다음 장에 이야기하겠습니다.

다시 String으로 돌아와서 String의 정의를 보고 알게된 것은 String은 그냥 8비트 값들의 배열이라는 것입니다. 그럼 결국 String의 슬라이스는 8비트 값들의 배열이라는 것도 알게됩니다. 그리고 String의 슬라이스를 나타내는 &str라는 타입이 있습니다.

예제 코드에 다음과 같이 2줄을 추가해보세요.

```rust
let greeting: String = String::from("Hello, world");
let gretting_slice: &String = &greeting[1..3];
```

이 예제 코드는 아래와 같은 에러를 내고 빌드되지 않습니다.

```rust
error[E0308]: mismatched types
   --> src/main.rs:173:35
    |
173 |     let gretting_slice: &String = &greeting[1..3];
    |                         -------   ^^^^^^^^^^^^^^^ expected `&String`, found `&str`
    |                         |
    |                         expected due to this
    |
    = note: expected reference `&String`
               found reference `&str`
```

아직 에러 메세지가 다 이해는 안되지만 다음과 같이 코드를 바꾸라는 도움말이 나왔습니다. 도움말대로 수정하면 빌드가 제대로 되는 것을 확인할 수 있습니다.

```rust
let greeting: String = String::from("Hello, world");
let gretting_slice: &str = &greeting[1..3];
```

이렇게 String객체의 슬라이스를 사용할 때는 &String이 아닌 &str 타입을 사용해야한다는것을 알 수 있습니다.

String은 흔히 말하는 Fat pointer입니다. 실제로 문자열이 저장된 메모리의 주소도 들어있지만, 데이터의 현재 길이나 버퍼의 크기 등 추가 정보들이 들어있습니다. 슬라이스는 이전 배열에서도 봤지만 코드가 실행되는 시점에서 순수 데이터의 일부분이나 데이터 전체를 보기 위한 참조입니다. String객체를 C에서 하나의 구조체라고 생각하고, 슬라이스를 데이터중 일부분에 대한 포인터 char*라고 생각하면 그 관계에 대해 이해하는데 도움이될지도 모르겠습니다.

```rust
struct String {
    int buffer_len;
    int data_len;
    char *buffer;
};
```

그럼 한가지 의문이 드는게 &str이 아니라 str으로 참조 연산자를 안쓰고 사용할 수는 없는가 입니다. 사용할 수 없습니다. 왜냐면 러스트는 메모리 관리를 위해 컴파일 시점에서 크기가 정해진 객체만을 허용하기 때문입니다.

str타입은 벡터에 들어있는 UTF-8데이터 그 자체, 순수하게 문자들만을 저정하는 메모리 버퍼를 가리키는 변수가 되어야 합니다. 하지만 문자열의 길이는 정해진 것이 아닙니다. 일반적인 구조체는 각 필드의 데이터 타입이 고정되어있으니 컴파일 시점에 크기를 알 수 있고, 배열은 배열을 선언할 때 데이터가 몇개인지 지정하기 때문에 크기를 알 수 있습니다. 하지만 벡터가 가진 메모리 덩어리의 크기는 프로그램이 실행되는 중간에 얼마든지 변할 수가 있는 것입니다. 결국 str라는 타입은 그 자체만으로는 사용할 수가 없습니다.

하지만 참조연산자 &를 붙이면 이야기가 달라집니다. 왜냐면 &str는 포인터이고 포인터의 크기는 컴파일 시점에 고정되어있기 때문입니다. 최신 씨피유를 쓴다면 64비트, 8바이트로 고정된 크기를 가잡니다. 따라서 아래와 같이 선언하는 것은 컴파일러가 봤을 때 64비트의 변수를 스택 메모리에 만들면 되는 것이니, 아무런 문제가 없습니다.

```rust
let ref_literal: &str = "hello";
let ref_string: &str = &string_object[2..5];
```

ref_literal이나 ref_string이나 그 자체는 포인터로 고정된 크기를 가지고 스택에 저장되어있습니다. 그리고 둘다 가리키는 데이터 또한 모두 동적으로 크기가 변하지 않습니다. 따라서 컴파일러는 앞으로 이런 문자열 슬라이스를 사용하는 코드를 볼때마다 메모리 안전성을 확인할 수 있습니다. 가지고있는 문자열의 크기 이상으로 읽고쓰는 것을 방지할 수 있습니다.

반대로 "hello"와 같은 리터럴을 이용해서 String을 만드는 것을 생각해보겠습니다. 가장 먼저 문자열이 저장될 벡터를 만들어야합니다. 벡터를 만들려면 힙 메모리에 메모리를 할당해야겠지요. 그리고 그 외에 문자열의 크기나 기타 정보들을 관리하기 위한 추가 데이터들을 할당해야합니다. 그렇게 슬라이스 “hello”를 가지고 String을 만들기 위한 추가 작업들을 format!과 같은 매크로 함수나 to_string등의 메소드 등이 실행하는 것입니다.

사실 이와같이 세부적인 것을 깊게 따지다보면 처음 러스트를 접하는 입장에서는 혼란만 생길 수 있습니다. 왜 메모리 크기가 컴파일 시점에 고정되어야하는지, 그럼 동적으로 할당되는 메모리는 사용할 수 없는 것인지 의문이 더 생깁니다. 지금 이 순간에는 String에 대한 참조나 슬라이스가 &str라는 것만 생각하고 넘어가는 것도 방법입니다. 아니면 러스트의 String 구조체에 대한 매뉴얼이나 The Rust Programming Language등의 추가 자료를 찾아보면서 각자 수준에 맞게 이해해 나가는 것도 필요합니다. 그렇게 지금은 간략하게 이애하고 넘어가서 점점 연습하고 문제에 부닥치면서 아 이래서 그랬구나하는 순간을 만나는 것이 프로그래밍 공부라고 생각합니다.

마지막으로 String 객체를 다른 함수에 전달할 때 슬라이스를 써야한다는 것을 이야기하겠습니다. 아직 소유권에 대해서 배우지는 않았지만 간단하게 말하면 String 을 그대로 다른 함수에 전달하면, 그 함수를 호출한 이후에는 그 객체를 다시 사용할 수 없습니다. 객체를 그대로 전달한다는 것은 소유권까지 넘겼다는 것이기 때문입니다. 그 와 반대로 String객체의 슬라이스를 넘기는 것은 객체에 있는 문자열 데이터의 참조권을 잠시 빌려주는 것으로 생각하면 됩니다. 함수가 끝나더라도 객체의 소유권은 함수를 호출한 코드에 남아있기 때문에 계속 객체를 사용할 수 있습니다.

예제 코드에 get_moved_string이라는 함수가 있습니다.

```rust
fn get_moved_string(data: &str) {
    println!("{}", data);
}

fn main() {
......
    let moving_string = String::from("hello");

    get_moved_string(&moving_string);
    println!("{}", moving_string);
......
}
```

이 함수를 참조가 아닌 객체 그대로 전달받도록 바꿔보겠습니다. get_moved_string함수의 인자 타입이 &str에서 String으로 바꾸겠습니다.

```rust
fn get_moved_string(data: String) {
    println!("{}", data);
}

fn main() {
......
    let moving_string = String::from("hello");

    get_moved_string(moving_string);
    println!("{}", moving_string);
......
}
```

빌드를 해보면 이런 에러 메세지를 볼 수 있습니다.

```bash
> 161 | fn get_moved_string(data: String) {
|        ------------       ^^^^^^ this parameter takes ownership of the value
|        |
|        in this function
> 
```

객체를 그대로 함수에 전달했기 때문에 함수에 객체의 소유권까지 옮겨졌다는 뜻입니다. 원래는 main함수가 moving_string이라는 변수의 소유권을 가지고있었고, 마음대로 사용할 수 있었지만, 객체를 get_moved_string함수에 전달했으므로 객체의 소유권까지 넘어가버려서 main함수는 더 이상 moving_string을 사용할 수 없게 되었습니다.

어떤 객체를 함수에 전달할 때는 일반적인 상황에서는 참조를 전달해야하고, String을 함수에 전달할 때는 &str을 전달해야한다는 것을 기억합시다.

### String을 배열처럼 참조할 수 없는 이유

아래와 같이 String객체에서 첫번째 글자를 출력할 수 있을까요?

```rust
let mut mutable_string = String::from("hello");
println!("{}", mutable_string[0]);
```

할 수 없습니다. 아래와 같은 에러 메세지를 얻으실겁니다.

```bash
> error[E0277]: the type `String` cannot be indexed by `{integer}`
--> src/main.rs:167:20
|
167 |     println!("{}", mutable_string[0]);
|                    ^^^^^^^^^^^^^^^^^ `String` cannot be indexed by `{integer}`
> 
```

일단 답부터 이야기하면 아래와 같이 chars메소드를 호출해서 이터레이터를 만든 후, nth메소드로 특정 인덱스의 문자를 얻을 수 있습니다.

```rust
let mut mutable_string = String::from("hello");
println!("{}", mutable_string.chars().nth(0).unwrap());
```

nth메소드는 Option이라는 Enums를 반환하므로 이 Enums에서 최종 문자를 얻어내기 위해 unwrap이라는 메소드를 호출한 것입니다.

일단 Option이라는 Enums은 추후에 알아보기로 하고, 왜 인덱스를 이용한 직접 접근이 안되게 막아놨을까요?

그 이유는 UTF-8을 완벽하게 지원하기 위해서입니다. 언어를 디자인할 때 인덱스 참조를 지원해서 [0]이 0번째 바이트를 반환하도록 만들었을 수도 있습니다. 하지만 이러면 ascii에 대한 지원은 잘 될지 몰라도 UTF-8을 제대로 지원하는 언어가 될 수는 없습니다. 첫번째 글자를 반환할 수도 있었겠지만, 첫번째 글자 하나만 놓고봤을 때 이 첫글자가 1바이트가 될지 2바이트가 될지 알 수가 없습니다. 이런 여러가지 문제들이 있기 때문에, 항상 이터레이터를 호출하도록 만들고, 이터레이터가 문자열의 전체 데이터를 분석한 후에 한 문자씩 반환하도록 만들었습니다. 그런 이유로 String의 이터레이터 메소드 chars의 처리 속도가 느린 것입니다.

만약 바이트 단위로 쪼개고싶다면 as_bytes라는 메소드를 호출하면 됩니다. 문자열 데이터가 반드시 ascii 문자열이라는 상황이라면 사용할 수 있는 옵션입니다.

## 변수를 읽고 쓸수있는 권한을 의미하는 소유권(Ownership)

배열에서의 슬라이스나 String과 &str의 관계를 보면서 소유권을 넘기지 않기 위해 참조를 사용한다는 이야기를 수차례 했습니다. 슬라이스도 그렇지만 그 외에 러스트의 문법적인 특징들의 상당수가 소유권 개념을 구현하기 위해서 만들어진 것들이라고 해도 과언이 아닙니다. 왜 이런 문법을 정했을까? 왜 이건 이렇게 복잡하지? 등등 러스트를 공부하면서 겪게되는 의문들과 진입장벽들의 상당수가 소유권과 연관이 있습니다. 러스트가 가진 장점 중에 가장 큰 장점이라고 이야기하는 메모리 안전성이 바로 소유권으로 인해 가능한 것입니다.

소유권이 뭔지 그래서 러스트가 데이터를 메모리에 어떻게 배치하고 관리하는지를 이야기해보겠습니다.

### 소유권의 의미

소유권은 단어 그대로 생각하면 변수를 내 마음대로 할 수 있는 권한 즉 변수에 데이터를 할당하고 읽고 쓰고 해지해할 수 있는 권리일 것입니다. 함수의 인자로 전달받은 데이터에 대한 소유권도 있을 수 있으니 여러 함수나 여러 쓰레드에서 공유되는 변수나 메모리에 대한 권한을 의미합니다.

가비지 콜렉터가 있는 자바 등의 언어는 메모리를 해지할 수 있는 권한이 프로그램 코드가 아닌 가비지 콜렉터에게 있습니다. 프로그램은 메모리를 할당받아서 객체를 만들어서 읽고 쓸 수 있지만 해지하지는 않습니다. 그냥 더 이상 접근하지 않고 있으면 가비지 콜렉터가 알아서 메모리를 해지해줍니다.

러스트는 컴파일러가 프로그램 코드를 컴파일 할 때 모든 메모리의 소유권을 추적합니다. 러스트가 정한 규칙에 어긋나게 메모리에 접근하는 코드가 있으면 친절한 안내 메세지를 출력하고 더 이상 컴파일을 하지 않습니다. 그래서 러스트 코드의 컴파일 시간이 오래걸린다는 불평이 많습니다. 수십 ~ 수백줄의 간단한 코드도 몇 초정도 시간이 걸리는걸 보면서 좀 답답할 때도 있긴합니다. (하지만 그정도의 간단한 코드를 만드는데 컴파일하고 고치고 다시 컴파일하는 과정을 여러번 반복해야할 정도로 컴파일 에러를 많이 만들고 있다는 것은 개발자에게도 문제가 있는게 아닌가 생각됩니다.) 하지만 빌드를 여러번 할 필요도 없는게 VSCODE등 대부분의 개발툴에서 러스트 언어를 동적으로 분석해주고, 코드를 쓸 때마다 에러 체크를 해줍니다. 빌드하기전에 미리 모든 컴파일 에러를 고칠 수 있습니다. 또 "cargo check"같은 명령을 사용하면 컴파일 에러가 있는지 확인하는 시간을 줄일 수 있습니다.

VSCODE를 예를 들면 Inlay hints https://code.visualstudio.com/docs/languages/rust#_inlay-hints 나 Linting https://code.visualstudio.com/docs/languages/rust#_linting 등의 기능이 있어서, cargo를 호출하기전에 코드를 쓰는 단계에서 미리 거의 모든 컴파일 에러를 잡을 수 있습니다.

또한 러스트 언어는 한번 빌드가 되고나면 좀처럼 메모리 관련 에러는 발생하지 않습니다. 기타 로우레벨 언어들로 만든 코드들이 빌드되서 실행은 되더라도 오랜 시간동안 에러가 없는지 검증해야되고, 정적 분석 툴 등을 돌려야 되는 시간들을 생각해보면 전체적인 개발 시간은 확실히 줄어드는 것이라 생각합니다.

The Rust Programming Language(https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)에서는 소유권이라는 것이 3가지 규칙을 의미한다고 설명합니다.

- Each value in Rust has an *owner*.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.

제가 나름대로 번역하고 그 의미에 대해서 설명을 붙이면 이렇습니다.

- 모든 값(메모리나 변수, 데이터라고 생각해도 괸찮습니다)는 누군가에게는 소유되어야 합니다. 특정한 누군가에게 소유되지않고 누구나 마음대로 쓸 수 있는 값은 없습니다.
- 한번에 하나의 소유권자가 있을 수밖에 없습니다. 여럿이 하나의 변수를 동시에 소유할 수 없습니다.
- 소유권이 자기가 존재할 수 있는 범위(Scope라고 하는데 보통 {로 시작하고 }로 끝나는 구역을 의미합니다.) 끝나면 변수는 메모리가 해지되고 더 이상 사용할 수 없게됩니다.

함수가 대표적인 하나의 스코프입니다. 몇가지 스코프를 실험하는 예제를 만들어봤습니다.

```rust
struct MyStruct {}

impl Drop for MyStruct {
    fn drop(&mut self) {
        println!("Dropping MyStruct now!");
    }
}

fn internal_scope() {
    let hello_string = String::from("hello");
    {
        let world_string = String::from("world");
        println!("{}", hello_string);
        println!("{}", world_string);
    }
    println!("{} again", hello_string);
}

fn duplicated_names() {
    let hello_string = String::from("hello");
    {
        let hello_string = String::from("world");
        println!("{}", hello_string);
    }
    println!("{}", hello_string);
}

fn main() {
    internal_scope();
    duplicated_names();
    
    println!("main starts");
    {
        println!("inner-scope starts");
        let _my: MyStruct = MyStruct{};
        println!("inner-scope ends");
    }
    println!("main ends");
}
```

```bash
$ cargo run --bin ownership_scope
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.16s
     Running `target/debug/ownership_scope`
hello
world
hello again
world
hello
main starts
inner-scope starts
inner-scope ends
Dropping MyStruct now!
main ends
```

간단하게 스코프에 대한 실험을 하는 internal_scope라는 함수를 보겠습니다.

```rust
fn internal_scope() {
    let hello_string = String::from("hello");
    {
        let world_string = String::from("world");
        println!("{}", hello_string);
        println!("{}", world_string);
    }
    println!("{} again", hello_string);
}
```

함수 시작 부분에서 생성된 hello_string이라는 변수는 함수가 끝나는 }를 만나면서 해지됩니다. 함수의 스코프가 끝나는 }에서 함수가 소용했던 hello_string이라는 변수가 해지되는 것입니다. "hello"가 저장되어있는 메모리가 해지되고, hello_string이라는 변수를 더 이상 사용할 수 없게 되는 것입니다.

hello_string이라는 변수는 _internal_scope함수가 소유하고, 따라서 스코프는 _internal_scope함수가 끝날 때까지 입니다. world_string이라는 변수의 소유권은 _internal_scope함수안에 새로 만들어진 블럭에 있습니다. 그 새로운 블럭의 시작 지점은 두번째 {이고 끝 지점은 첫번째 }가 있는 곳입니다. 따라서 아래와같이 world_string을 소유한 블럭 밖에서 world_string을 사용할 수가 없습니다.

```rust
fn internal_scope() {
    let hello_string = String::from("hello");
    {
        let world_string = String::from("world");
        println!("{}", hello_string);
    }
    println!("{}", world_string);
    println!("{} again", hello_string);
}
```

```rust
$ cargo build
error[E0425]: cannot find value `world_string` in this scope
   --> src/main.rs:15:20
    |
  7 |     println!("{}", world_string);
    |                    ^^^^^^^^^^^^ help: a local variable with a similar name exists: `hello_string`
```

world_string이 사용된 스코프는 main함수의 스코프이므로 world_string을 소유하지 않았다는 에러 메세지를 확인할 수 있습니다. 하지만 world_string이 존재하는 스코프는 main함수의 스코프이기도 하므로 hello_string변수를 사용할 수는 있습니다.

그럼 duplicated_names함수에서와 같이 같은 이름의 변수가 중첩된 스코프에 존재할 때는 어떨까요?

```rust
fn duplicated_names() {
    let hello_string = String::from("hello");
    {
        let hello_string = String::from("world");
        println!("{}", hello_string);
    }
    println!("{}", hello_string);
}
```

Cargo를 이용해서 코드를 실행해보면 다음과 같이 출력됩니다.

```rust
$ cargo run --bin ownership_scope
...
world
hello
...
```

2개의 변수가 동일한 이름으로 생성되지만 “hello”라는 값를 가진 변수는 main 함수가 끝나는 바깥 스코프에가 소유권을 가지고있고, “world”라는 데이터를 가진 변수는 main함수 안에서 새로 생성된 작은 스코프가 소유권을 가지고 있는 것입니다. 작은 스코프가 끝날 때 “world”라는 데이터를 가진 변수(혹은 객체)는 해지됩니다.

참고로 스코프가 끝날 때 자신이 소유한 변수들의 drop 메소드를 호출합니다. 예제에 MyStruct라는 아무런 데이터를 가지지않는 구조체를 선언하고, drop 메소드를 구현해준 코드가 있습니다. (아직 구조체에 대한 문법을 알아보지않았지만, 구조체의 선언만 보면 C언어와 거의 동일합니다. 구조체의 메소드를 정의하는 문법은 아직 모르지만, 일단 drop이라는 메소드가 호출되는 시점만 생각해보겠습니다.)

```rust
struct MyStruct {}

impl Drop for MyStruct {
    fn drop(&mut self) {
        println!("Dropping MyStruct now!");
    }
}
......
fn main() {
    ......
    println!("main starts");
    {
        println!("inner-scope starts");
        let my: MyStruct = MyStruct{};
        println!("inner-scope ends");
    }
    println!("main ends");
}
```

```bash
$ cargo run --bin ownership_scope
......
main starts
inner-scope starts
inner-scope ends
Dropping MyStruct now!
main ends
```

drop메소드가 호출되는 지점이 곧 변수의 메모리가 해지되는 지점인데, “inner-scope ends”라는 메세지 후에 drop메소드가 호출되는 것을 볼 수 있습니다. 즉 스코프 안의 모든 코드가 끝나고 스코프가 없어지는 최후의 순간에 스코프가 소유한 변수들을 해지하는 것을 확인할 수 있습니다.

### 소유권의 이동

사실 개념 설명만 들으면 약간 그래서 어쩌라는 건가 라는 생각이 들 수도 있습니다. 몇가지 제가 자주 겪어본 케이스 몇가지를 소개하겠습니다. 이정도만 일단 알고 시작하면 작은 프로젝트를 시작하는데는 문제가 없을거라 생각합니다.

#### 변수 할당에서 소유권이 이동하는 경우

가장 간단한 예는 변수간에 할당이 발생할 때 소유권이 이동하는 경우입니다.

```rust
let s1 = String::from("foo");
println!("{}", s1);
let s2 = s1;
println!("{} {}", s1, s2);
```

이 예제에서 러스트는 s1을 s2로 이동시킵니다. 보통의 언어들에서는 객체의 복사가 일어나거나 포인터의 복사가 일어날 것입니다. 러스트에서는 내부적으로는 포인터의 복사만 일어나고, 거기에 더해서 소유권의 이동까지 일어납니다. 객체 데이터를 복사하지 않기 때문에 속도는 빠르면서 소유권의 이동까지 일어나므로 데이터가 의도하지 않게 공유되는 것을 방지합니다.

그런데 실제로 뭔가를 만드는 경우에 이렇게 예제와 같이 단순하게 변수와 변수사이에 값을 옮기는 경우는 거의 없습니다. 실제로 많이 겪는 경우는 변수값의 이동이 일어나는지 잘 안보이는 경우들이 대부분입니다.

```rust
let mut user_input = String::from("페리스");
println!("{}", user_input);
let mut greeting = user_input + "씨 안녕하세요";
println!("{}", greeting);
println!("{}", user_input); // Compile error
```

기존 언어에 익숙하다보면 이 코드에 문제가 안보일 수 있습니다. 사실 안보이는게 당연한것입니다. 하지만 러스트에서는 user_inut의 소유권 이동이 일어나고, 거기에 메세지가 추가되서 greeting 변수에 저장된다는 차이가 있습니다.

```rust
error[E0382]: borrow of moved value: `user_input`
   --> src/main.rs:175:20
    |
171 |     let mut user_input = String::from("아이유");
    |         -------------- move occurs because `user_input` has type `String`, which does not implement the `Copy` trait
172 |     println!("{}", user_input);
173 |     let mut greeting = user_input + "씨 안녕하세요";
    |                        ---------------------------- `user_input` moved due to usage in operator
174 |     println!("{}", greeting);
175 |     println!("{}", user_input); // Compile error
    |                    ^^^^^^^^^^ value borrowed here after move
```

이와같이 소유권이 이동이 안보이는 경우가 많긴합니다만, 변수간의 소유권 이동은 컴파일러가 너무나 친절하게 어디에서 이동이 발생했고, 소유권이 없는 변수를 어디에서 접근해서 에러가 발생했는지를 다 알려줍니다. 그래서 에러를 찾기도 쉽고 고치기도 어렵지 않습니다.

#### 함수 인자로 전달되고 반환값을 받을 때 소유권이 이동하는 경우

```rust
fn make_greeting(name: String) -> String {
    let greeting = format!("{}씨 안녕하세요", name);
    greeting
}

fn main() {
    let user = "페리스".to_string();
    let greeting = make_greeting(user);
    println!("{}", greeting);
}
```

이번에도 크게 어렵지 않은 경우입니다. user라는 변수가 make_greeting의 name이라는 인자에 바인딩되었습니다. 따라서 이전에 본 변수의 할당과 유사한 경우입니다. user라는 변수가 가진 값에 대한 소유권이 name으로 옮겼갔습니다. make_greeting 함수가 끝난 뒤에는 user 변수는 사용할 수 없습니다. greeting이라는 변수는 make_greeting이라는 함수에서 생성되었지만 main 함수로 소유권이 이동된 경우입니다.

```rust
fn make_greeting(name: String) -> String {
    let greeting = format!("{}씨 안녕하세요", name);
    greeting
}

fn main() {
    let mut user = "페리스".to_string();
    user = make_greeting(user);
    println!("{}", user);
}
```

좀 웃기긴 하지만 위 예제는 user 변수의 소유권을 main에서 make_greeting으로 이동한 후, 다시 main으로 가져오는 예제입니다. 그냥 이런것도 가능하다는 것을 보여드린 예제입니다.

실질적으로 이렇게 함수런에 소유권이 이동하도록 구현하는 경우는 많지 않습니다. 함수를 호출할 때는 보통 객체의 레퍼런스를 전달해서 소유권을 넘기지 않습니다.

```rust
fn make_greeting(name: &str) -> String {
    let greeting = format!("{}씨 안녕하세요", name);
    greeting
}

fn main() {
    let mut user = "아이유".to_string();
    user = make_greeting(&user);
    println!("{}", user);
}
```

이전에 알아본 레퍼런스가 이렇게 변수의 이동없이도 다른 스코프에서 변수를 사용할 수 있도록 하기 위한 방법입니다. 내부적으로는 포인터만을 전달하는 것입니다. 따라서 C/C++ 언어와 성능 차이가 없습니다. 하지만 컴파일러가 컴파일을 하는 단계에서 소유권의 이동을 체크하고 메모리 공유를 막기 때문에 성능이 좋으면서도 메모리 안전성이 좋은 언어가 된 것입니다.

러스트는 이렇게 레퍼런스를 생성하는 것을 빌렸다(Borrowing)이라고 표현합니다. 소유권의 이동없이 다른 스코프에서 사용하도록 해주는 것이니 적절한 표현이라고 생각합니다.

위 예제에서는 단순히 읽기만 가능한 불변 레퍼런스(Immutable reference)를 사용했는데요, 쓰기도 가능한 가변 레퍼런스(Mutable reference)도 있습니다.

```rust
fn make_greeting(name: &mut String) {
    name.push_str("씨 안녕하세요");
}

fn main() {
    let mut user = "페리스".to_string();
    make_greeting(&mut user);
    println!("{}", user);
}
```

mut 키워드를 함수 호출하는 부분에도 넣고, 함수 인자에도 넣어야 된다는 것을 기억해야합니다.

아래와 같이 Immutable한 변수의 Mutable reference를 만드는 것은 불가능합니다.

```rust
fn main() {
    let user = "페리스".to_string();
    make_greeting(&mut user);
    println!("{}", user);
}
```

소유자가 바꾸고 싶지 않은 변수를 빌린쪽에서 맘대로 바꾸는 것은 당연히 허용하면 안되겠지요.

레퍼런스에 대한 규칙을 요약하자면 다음과 같습니다.

- Mutable reference는 오직 하나만 존재할 수 있다.
- Immutable reference는 여러개 존재할 수 있다.
- 레퍼런스는 언제나 실제 데이터를 참조해야한다.

상식적으로 생각해도 이해가 되는 규칙입니다. 데이터를 바꿀 수 없는 Immutable reference가 여러개 있다고해도 데이터는 변하지 않으니까 상관없습니다. 데이터를 바꿀 수 있는 Mutable reference가 있다면 이 데이터는 언제든지 바뀔 수 있으므로 다른 어떠한 형태의 레퍼런스도 존재하면 안됩니다.

#### 이터레이터에서 소유권이 이동하는 경우

마지막으로 벡터나 배열같이 여러개의 데이터를 이터레이터로 접근하는 경우를 알아보겠습니다. 사실 이 경우가 가장 흔하게 실수하는 경우이고, 소유권 개념을 알았더라도 한동안은 당황하게 되는 경우입니다.

 배열이나 벡터등의 이터레이터를 만드는 메소드는 2가지가 있습니다.

- iter(): 슬라이스 이터레이터를 만듬
- into_iter(): 변수 값으로 이터레이터를 만듬

참고: https://doc.rust-lang.org/std/iter/trait.IntoIterator.html#tymethod.into_iter

두개가 무슨 차인지 한번 실험을 해보겠습니다. 먼저 변수의 값으로 이터레이터를 만들어보겠습니다.

```rust
fn main() {
    let user: [String;3] = ["My".to_string(),
                            "Bloody".to_string(),
                            "Valentine".to_string()];
    for c in user.into_iter() {
        println!("{}", c);
    }
    println!("{:?}", user);
}
```

```rust
error[E0382]: borrow of moved value: `user`
   --> src/main.rs:8:22
    |
2   |     let user: [String;3] = ["My".to_string(),
    |         ---- move occurs because `user` has type `[String; 3]`, which does not implement the `Copy` trait
...
5   |     for c in user.into_iter() {
    |                   ----------- `user` moved due to this method call
...
8   |     println!("{:?}", user);
    |                      ^^^^ value borrowed here after move
    |
note: `into_iter` takes ownership of the receiver `self`, which moves `user`
   --> /Users/user/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/iter/traits/collect.rs:262:18
    |
262 |     fn into_iter(self) -> Self::IntoIter;
    |                  ^^^^
    = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
help: you can `clone` the value and consume it, but this might not be your desired behavior
    |
5   |     for c in user.clone().into_iter() {
    |                   ++++++++
```

Copy trait나 self, clone등 모르는 키워드들이 나와서 당황스러울 수 있습니다. 미리 into_iter가 무엇인지 모른 상태에서 이 에러메세지를 본다면 막막할 수 있습니다.

일단 지금 상태에서 into_iter말고 iter 메소드를 사용해보겠습니다.

```rust
// src/ownership_move/main.rs
fn main() {
    let user: [String; 3] = [
        "My".to_string(),
        "Bloody".to_string(),
        "Valentine".to_string(),
    ];
    for c in user.iter() {
        println!("{}", c);
    }
    println!("{:?}", user);
}
```

```bash
$ cargo run --bin ownership_move
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.18s
     Running `target/debug/ownership_move`
My
Bloody
Valentine
["My", "Bloody", "Valentine"]
```

아무 이상없이 실행됩니다. 무슨 차이일까요?

into_iter를 사용했을 때의 에러 메세지를 보면 이런 말이 있습니다.

> note: `into_iter` takes ownership of the receiver `self`, which moves `user`
> 

값으로 이터레이터를 만든다는 의미는 변수의 값을 이동시킨다는 의미입니다. 즉 소유권을 가져간다는 뜻입니다. for 루프에서 c라는 변수의 타입이 String이 되고, c변수가 루프를 돌때마다 배열에 있는 객체를 하나씩 소유하게 됩니다. 그리고 루프가 끝날 때마다 스코프가 끝나고, 객체가 해지됩니다. 모든 루프가 끝나면 배열 전체가 다 해지됩니다.

>
> Visual Studio Code 등의 IDE툴을 사용하면 변수의 타입이 생략된 경우 자동으로 IDE가 타입을 판단해서 보여주는 기능이 있습니다. 이 기능을 활용해보면 c가 어떤 타입이 되는지 쉽게 확인할 수 있습니다.
>

반대로 iter()는 슬라이스를 만든 후 슬라이스의 이터레이터를 만듭니다. 슬라이스는 레퍼런스이므로 소유권을 가져갈 수 없습니다. 결론적으로 소유권 이동없이 배열의 각 항목에 레퍼런스로 접근합니다. c 변수의 타입은 &String이 됩니다.

이터레이터에 대한 팁을 한가지 드리자면 iter와 into_iter가 각각 사용하는 경우가 다릅니다.

- iter: 루프 후에도 계속 데이터를 사용할 경우에 사용함
- into_iter: 벡터를 해지하고 완전히 새로운 타입의 데이터를 만들때 사용함

C언어에서 포인터의 배열을 만들어서 여러개의 데이터를 관리하다가 프로그램 마지막에 모든 데이터를 해지하는 패턴을 흔하게 사용합니다. 이때 루프를 돌면서 각각의 데이터를 해지하고, 마지막에 배열 자체를 해지합니다. 이럴 때 into_iter를 사용하면, 각각의 항목을 따로 해지하지않아도 각 루프의 스코프가 끝나면서 자동으로 해지됩니다. for 루프에서 만든 스코프가 끝날때 각 데이터의 스코프가 끝나기 때문입니다.

참고로 iter메소드는 불변 레퍼런스를 만듭니다. for루프안에서 user데이터를 읽기만하고 쓸수는 없습니다. 만약 user데이터를 수정하고 싶다면 iter_mut 메소드를 써서 가변 레퍼런스를 만들면됩니다. iter_mut 메소드의 메뉴얼(https://doc.rust-lang.org/std/slice/struct.IterMut.html)을 참고하세요.

### Clone과 소유권

좀전에 into_iter를 사용한 경우 컴파일 에러를 보면 user.clone().into_iter()로 고쳐보라는 에러 메세지가 있습니다. 

```rust
fn main() {
    let user: [String; 3] = [
        "My".to_string(),
        "Bloody".to_string(),
        "Valentine".to_string(),
    ];
    for c in user.clone().into_iter() {
        println!("{}", c);
    }
    println!("{:?}", user);
}
```

이렇게 바꾸면 컴파일 에러 없이 잘 동작합니다. clone은 말 그대로 데이터를 똑같이 복사해서 사본을 만드는 것입니다. Deep copy를 한다고 생각할 수도 있습니다. 위에서는 루프를 돌기전에 user의 이름없는 복사본을 만들고 그 복사본의 into_iter 메소드를 호출해서 인터레이터를 만듭니다. 그래서 user 객체는 그대로 존재하고, 복사본만 해지됩니다. 하지만 실제 제품을 이렇게 만드는 경우는 별로 없겠지요. iter메소드를 사용하는게 더 현실적입니다.

### 변수가 저장되는 위치와 소유권의 관계

지금 단계에서 중요한 내용은 아니지만 러스트의 내부를 이해하기 위한 약간의 추가 설명을 해보겠습니다.

이전에 만들었던 피보나치 함수를 다시 읽어보겠습니다.

```rust
fn fib(mut index: i32) -> i32 {
    let mut a = 1;
    let mut b = 1;
    let mut t;

    loop {
        t = a + b;
        a = b;
        b = t;

        index -= 1;
        if index <= 0 {
            break;
        }
    }
    b
}
```

t = a+b라는 코드에서 t변수는 a와 b중 어느 변수의 소유권을 가지게되는 것일까요?

사실 정수 타입의 변수는 소유권 이동이 일어나지 않습니다. 정수나 부동 소수점 타입등과 Boolean타입은 소유권의 이동이 일어나지않습니다. 함수 호출에 인자로 사용되면 값이 복사됩니다. 또 위 코드와 같이 대입이 될때도 값이 복사됩니다.

그럼 소유권 이동이 일어나는 타입과 그런 규칙에서 예외되는 타입의 구분은 무엇일까요? 바로 변수가 스택에 할당되는가 힙에 할당되는가에 따라 결정됩니다. 스택에 할당되는 변수는 소유권 이동이 일어나지않고 대신 복사가 됩니다. 힙에 할당되는 변수는 (명시적으로 복사를 하도록 강제하지않으면) 소유권이 이동됩니다. 그럼 어떤 변수가 스택에 할당되고 어떤 변수는 힙에 할당될까요?

일반적인 숫자 (정수와 부동 소수점)과 참/거짓의 Boolean 타입은 메모리 크기가 정해져있습니다. i32이면 4바이트이고 u8이면 1바이트입니다. 이렇게 컴파일 시점에 이미 메모리 크기가 정해진 변수는 스택에 할당됩니다. 스택 메모리에 할당하는 것이 빠르고 관리가 쉽기 때문입니다. 스택에 저장된 변수들은 함수가 종료될 때 스택 영역 전체를 해지하면서 한꺼번에 해지됩니다. 따라서 메모리 누수에 대한 염려도 없고 메모리 크기가 작으므로 복사하는데도 시간이 오래 걸리지 않습니다. 따라서 굳이 소유권을 설정하지 않아도 되는 것입니다.

그와 다른게 String과 같은 구조체 타입을 들 수 있습니다. 구조체의 크기만큼 힙 영역에 메모리를 할당해서 객체를 만듭니다. malloc같은 메모리 할당 함수를 내부적으로 호출해서 메모리 영역을 할당하는 것입니다. 왜냐면 컴파일 시점에 String 객체에 얼마만큼의 문자데이터를 넣을지 모르기 때문입니다. 리터럴로 String 객체를 만들 때는 데이터 크기를 알 수 있겠지만, 나중에 데이터를 추가할 수 있습니다. 또 사용자 입력을 받아서 String 객체를 만들거나 네트워크에서 받은 데이터로 객체를 만들 때도 프로그램이 실행 중일 때만 데이터의 크기를 알 수 있습니다.

```rust
fn main() {
    let s = String::new();
}
```

위와 같이 s라는 변수를 만들었습니다. 이 s는 스택에 생성된 포인터 변수입니다. 64비트 CPU를 가진 시스템에서 동작한다면 스택에 8바이트 메모리 영역을 할당하고, 힙 영역에 String객체를 생성한 후 스택에 있는 8바이트 메모리 영역에 힙 영역의 주소를 저장한 것입니다. 우리가 s라는 변수를 통해 객체에 저장된 데이터를 읽으면

1. s라는 변수에서 힙 영역의 주소 값을 읽음
2. 힙 영역에서 데이터를 읽음

이와 같이 2번의 메모리 접근이 일어납니다. String객체를 변수 대입이나 함수 호출을 통해 소유권이 이동된다는 것은 물리적으로 따지면 포인터 값 (64비트 정수 값)을 복사하는 것 뿐입니다. 컴파일러가 변수 대입이나 함수 호출 등 소유권 규칙에 따른 동작이 일어날 때마다 포인터 값의 이동을 감시하고 규칙에 부합하는지를 따지는 것 뿐입니다. 결과적으로 안정적인 메모리 관리를 할 수 있으면서도 성능 감소가 없는 프로그램을 만들 수 있는 것입니다.

정리를 하자면 러스트에서 원시 타입 Primitive type으로 분류된 타입들은 이동이 아니라 복사가 일어나입니다. 어떤 타입들이 원시 타입인지는 Rust의 Standard Library 메뉴얼을 참고하시기 바랍니다.

https://doc.rust-lang.org/std/#primitives

C나 예전 C++을 사용해본 개발자라면 이렇게 생각하면 쉽습니다.

>
> malloc/new 등으로 할당하고 free로 해지해줘야되는 메모리나 객체를 자동으로 해지해주는 대신 소유권을 관리해줘야 한다. Primitive type은 복사가 일어나고 그 외는 이동이 발생한다.
> 

모던 C++을 아는 개발자는 이렇게 생각하면 더 이해하기 쉬울 것입니다.

>
> RAII가 권장이 아니라 강제 사항이다. 모든 포인터는 스마트 포인터이다.
> 

나중에 Copy trait라는게 나오는데, 미리 간단하게 말씀드리면 데이터 타입의 크기를 컴파일러가 알기 때문에 데이터의 이동이 아니라 복사를 해주는 데이터 타입들의 속성이라고 생각하면 됩니다. 컴파일러가 크기를 안다는 것은 Primitive type은 기본적으로 Copy trait를 구현하고 있다는 말입니다. 그 외의 타입들은 동적으로 크기가 바뀔 수도 있으므로 컴파일러가 Copy trait를 자동으로 구현해주지 못합니다. 동적으로 크기가 바뀌거나 또다른 객체를 포함하고있는 등의 데이터는 Clone을 사용해야합니다.

## 구조체

러스트에는 클래스가 없고 구조체만 있습니다. 구조체에 메소드를 추가할 수 있지만, 상속 기능이 없기 때문에 완전한 OOP언어는 아닙니다. 구조체는 형태는 C 언어와 크게 다를게 없습니다.

아래 예제는 다양한 구조체의 형태들을 소개하고 있습니다.

```rust
// src/struct/main.rs
// An attribute to hide warnings for unused code.
#![allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// A unit struct
struct Unit;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    top_left: Point,
    bottom_right: Point,
}

fn main() {
    // Create struct with field init shorthand
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    // Print debug struct
    println!("{:?}", peter);

    // Instantiate a `Point`
    let point: Point = Point { x: 10.3, y: 0.4 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our
    // other one
    let bottom_right = Point { x: 5.2, ..point };

    // `bottom_right.y` will be the same as `point.y` because we used that field
    // from `point`
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using a `let` binding
    let Point { x: left_edge, y: top_edge } = point;

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: bottom_right,
    };

    // Instantiate a unit struct
    let _unit = Unit;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);
}
```
출처: https://doc.rust-lang.org/rust-by-example/custom_types/structs.html

조금이라도 프로그래밍을 해보신 분들이라면 이미 잘 알고계실만한 구조체와 튜플의 모습 그대로입니다. 그나마 유닛 구조체라는게 좀 특이합니다. 아무런 내부 데이터가 없는 구조체입니다. 이건 나중에 트레이트Trait라는 클래스의 메소드와 같은 것을 사용하기 위한 구조체입니다. 클래스인데 내부 변수는 없고 메소드만 있는 클래스라고 생각할 수도 있습니다.

다른 언어와 확실히 다른게 있다면 구조체를 만들 때 인자로 사용된 객체의 소유권이 이동한다는 것입니다. 다음 예제를 실행해보겠습니다.

```rust
struct Person {
    name: String,
    age: u8,
}

fn main() {
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };
    println!("{}", peter.name);
    println!("{}", name);
}
```

```rust
error[E0382]: borrow of moved value: `name`
 --> src/main.rs:6:20
  |
2 |     let name = String::from("Peter");
  |         ---- move occurs because `name` has type `String`, which does not implement the `Copy` trait
3 |     let age = 27;
4 |     let peter = Person { name, age };
  |                          ---- value moved here
5 |     println!("{}", peter.name);
6 |     println!("{}", name);
  |                    ^^^^ value borrowed here after move
  |
  = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider cloning the value if the performance cost is acceptable
  |
4 |     let peter = Person { name.clone(), age };
  |                              ++++++++
```

이전에 소유권의 이동에 대해서 설명하면서 소유권이 없는 변수에 접근했을 때 보여드린 에러 메세지와 거의 동일한 형태의 에러 메세지를 다시 보게 됩니다. 각 에러 메세지가 어떤 의미인지 보겠습니다.

1. move occurs because `name` has type `String`, which does not implement the `Copy` trait: String 타입은 Copy trait라는걸 구현하지 않습니다. 컴파일러가 String타입의 메모리 크기가 얼마인지 알 수 없습니다. 지금 예제 코드는 “Peter”라는 리터럴을 String으로 만들기 때문에 메모리 크기를 알 수 있는 것처럼 보이지만, 동적으로 String을 만드는 경우를 생각하면 얼마나 긴 문자열을 생성할 지 알 수 없습니다.
2. value moved here: name 변수의 소유권이 구조체 Person을 만들 때 이동했습니다.
3. value borrowed here after move: println으로 소유권이 없는 변수에 접근했으므로 에러가 발생한 것입니다.
4. consider cloning the value if the performance cost is acceptable: name.clone()으로 복사본을 만들어서 Person에 전달하는 것도 하나의 해결책이긴합니다만 불필요하게 메모리를 더 사용하게됩니다.

요약하자면 Person이라는 객체를 만들기 위해 name이라는 String 객체를 사용했는데, name의 소유권이 peter라는 변수로 넘어갔다는 것입니다. 그래서 peter변수가 생성된 이후로는 name이라는 변수를 사용할 수 없게되었습니다.

### 메소드 정의

구조체를 만드는 방법을 봤으니 이번에는 구조체의 메소드를 정의하는 예제를 보겠습니다.

```rust
struct Point {
    x: f32,
    y: f32,
}

struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

impl Rectangle {
    fn area(&self) -> f32 {
        let width = f32::abs(self.top_left.x - self.bottom_right.x);
        let height = (self.top_left.y - self.bottom_right.y).abs();
        width * height
    }
}

fn main() {
    let point1: Point = Point { x: 10.3, y: 0.4 };
    let point2: Point = Point { x: 22.5, y: 2.4 };
    let rect = Rectangle {
        top_left: point1,
        bottom_right: point2,
    };
    println!("area size={}", rect.area());
}
```

Point와 Rectangle이라는 구조체를 만듭니다. 그 다음 Rectangle 구조체의 메소드를 정의하는 impl 구문이 있습니다. 메소드를 정의할 때는 impl 키워드와 구조체 이름을 쓰고 하나의 블럭을 만듭니다. 그리고 그 블럭 안에서 &self를 첫번째 인자로 받는 함수를 만들면 메소드가 됩니다. 다른 언어들의 클래스 메소드를 만드는 것과 비슷합니다.

하나 눈여겨 볼만한건 f32라는 타입의 절대값을 구하는 abs라는 메소드가 2가지 형태로 사용된다는 것입니다.

1. 타입::메소드이름(..인자..)
2. 변수.메소드이름(..인자..)

1번 타입::메소드이름 형태는 보통 정적 메소드라고 하거나 연관 함수 Associated function 라고 부르는 것입니다. 구조체 타입에 종속되는 함수라서 구조체의 객체를 만들지 않아도 호출할 수 있습니다. 2번 변수.메소드이름 형태는 동적 메소드라고해서 객체를 반드시 만든 후에 객체를 이용해서 호출할 수 있는 메소드입니다. 그래서 첫번째 인자가 항상 &self가 됩니다.

메소드의 첫번째 인자에 &self만 사용할 수 있는게 아니라 &mut self를 쓸 수 있습니다. 구조체 내부 값을 변경하는 메소드라면 &mut self를 써야합니다. 그리고 자기 자신의 메모리를 해지하는 (원문으로는 consume이라고 표현합니다.) 메소드라면 self 인자를 갖을 것입니다. self앞에 &표시가 붙지 않으니 메소드가 자기 자신의 소유권을 전달받을거라는 표시입니다.

```rust
// src/struct_method/main.rs
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

impl Rectangle {
    fn new() -> Rectangle {
        Rectangle {
            top_left: Point { x: 0.0, y: 0.0 },
            bottom_right: Point { x: 0.0, y: 0.0 },
        }
    }

    fn area(&self) -> f32 {
        let width = f32::abs(self.top_left.x - self.bottom_right.x);
        let height = (self.top_left.y - self.bottom_right.y).abs();
        width * height
    }

    fn destroy(self) {
        // do nothing but free myself
        println!("destroyer");
    }
}

fn main() {
    let rect = Rectangle::new();

    {
        let point1: Point = Point { x: 10.3, y: 0.4 };
        let point2: Point = Point { x: 22.5, y: 2.4 };
        let rect2 = Rectangle {
            top_left: point1,
            bottom_right: point2,
        };
        rect2.destroy();

        //println!("area size={} {:?}", rect2.area(), rect2); // compile error!!!
    }

    println!("area size={} {:?}", rect.area(), rect);
}
```
```bash
$ cargo run --bin struct_method
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.31s
     Running `target/debug/struct_method`
destroyer
area size=0 Rectangle { top_left: Point { x: 0.0, y: 0.0 }, bottom_right: Point { x: 0.0, y: 0.0 } }
```

new라는 이름의 메소드는 러스트의 코딩 관례상 빈 객체를 생성하는 메소드의 이름으로 많이 쓰입니다. 그래서 보통 정적 메소드로 구현됩니다.

destroy라는 메소드는 인자를 self로 받아오므로 객체의 소유권을 가져옵니다. 따라서 메소드가 종료된 후부터는 객체를 더 이상 쓸 수 없습니다. new같이 특별히 정해진 이름이 있는 것은 아닙니다. 그리고 destroy와 같이 명시적으로 객체를 해지하는 메소드를 만드는건 특별한 일이 아니라면 잘 쓰지 않는 방법입니다. 

메소드에서 self를 이용해서 소유권을 받아오는 것을 확인해보기위해 주석 처리된 부분을 다시 코드로 바꾸고 빌드해보겠습니다.


```rust
......

fn main() {
    let rect = Rectangle::new();

    {
        let point1: Point = Point { x: 10.3, y: 0.4 };
        let point2: Point = Point { x: 22.5, y: 2.4 };
        let rect2 = Rectangle {
            top_left: point1,
            bottom_right: point2,
        };
        rect2.destroy();

        println!("area size={} {:?}", rect2.area(), rect2); // compile error!!!
    }

    println!("area size={} {:?}", rect.area(), rect);
}
```

```bash
gkim@gkim-laptop:~/study/my-rust-book$ cargo run --bin struct_method
   Compiling my-rust-book v0.1.0 (/home/gkim/study/my-rust-book)
error[E0382]: borrow of moved value: `rect2`
  --> src/struct_method/main.rs:45:39
   |
39 |         let rect2 = Rectangle {
   |             ----- move occurs because `rect2` has type `Rectangle`, which does not implement the `Copy` trait
...
43 |         rect2.destroy();
   |               --------- `rect2` moved due to this method call
44 |
45 |         println!("area size={} {:?}", rect2.area(), rect2); // compile error!!!
   |                                       ^^^^^ value borrowed here after move
   |
note: `Rectangle::destroy` takes ownership of the receiver `self`, which moves `rect2`
  --> src/struct_method/main.rs:27:16
   |
27 |     fn destroy(self) {
   |                ^^^^

For more information about this error, try `rustc --explain E0382`.
error: could not compile `my-rust-book` (bin "struct_method") due to 1 previous error
```

이제는 조금 익숙해진 에러 메세지들이 보입니다.


### 구조체 디버깅 방법

이전 예제를 보면 Point 구조체와 Rectangle 구조체의 정의 윗줄에 #[derive(Debug)]라는 코드가 있습니다.

```rust
// src/struct_method/main.rs
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

......

fn main() {
    let rect = Rectangle::new();

......

    println!("area size={} {:?}", rect.area(), rect);
}
```

이 예제를 실행하면 구조체 이름과 각 필드의 이름과 값까지 출력해줘서 굉장히 편리합니다.

```bash
$ cargo run --bin struct_method
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.31s
     Running `target/debug/struct_method`
destroyer
area size=0 Rectangle { top_left: Point { x: 0.0, y: 0.0 }, bottom_right: Point { x: 0.0, y: 0.0 } }
```

#[derive(Debug)]라는 구문은 std::fmt::Debug (Standard library에 속한 fmt라는 모듈에 정의된 Debug라는  trait)를 자동으로 구현하라는 의미입니다. 나중에 Trait에 대해서 설명할 때 정확한 의미를 알아보겠지만, 지금은 일단 “{:?}”라는 표현식을 써서 구조체의 각 필드의 값을 출력한다고 생각하면 됩니다. 구조체의 필드가 String같은 std에 정의된 타입이면 대부분 동작합니다. 만약 구조체의 한 필드가 또 다른 구조체 타입이라면, 그 다른 구조체도 #[derive(Debug)]를 선언해주면 됩니다. Rectangle에만 #[derive(Debug)]을 사용한게 아니라 Point에도 #[derive(Debug)]를 선언한 이유가 Rectangle의 디버깅 메세지를 출력할 때 Point의 디버깅 메세지도 같이 출력되어야하기 때문입니다.

## 열거형 Enums

### 기본 열거형

열거형도 패턴 매칭과 마찬가지로 러스트를 처음 접한 C/C++개발자들이 낯설어하는 특징 중에 하나입니다. 하지만 조금만 쓰다보면 너무나 편리하고 하기 때문에 자주 쓰게됩니다.

러스트 언어다운 프로그래밍을 하려면 이 열거형를 잘 활용하는게 중요합니다.

보통 C에서 열거형를 쓰는 이유는 주로 특정 값만을 가지는 타입을 새로 만들기 위해서입니다.

```c
// src/enum_basic/enum.c
#include <stdio.h>

enum WEEK {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday
};

int main()
{
    enum WEEK today;
    today = Sunday;
    printf("%d\n", today);
    today = 22;
    printf("%d\n", today);
    return 0;
}
```

위와 같이 WEEK이라는 새로운 타입을 만들었습니다. WEEK 타입의 변수는 Sunday부터  Saturday라는 값만을 갖도록 만드는게 목표입니다. 하지만 사실 C의 대부분의 타입이 그렇듯이 Sunday부터 Saturday가 사실상 모두 정수값이기 때문에, today 변수에 아무 정수값이나 넣어도 문제가 없습니다. today변수에 22라는 아무 정수값이나 저장하고 사용해도 컴파일에러도 없고 잘 동작합니다. 에러를 방지할 수 있는 방법이 전혀 없습니다.

러스트의 열거형도 마찬가지로 가장 기본적인 사용법은 특정 값만을 갖는 새로운 타입을 만드는 것입니다.

```rust
// src/enum_basic/main.rs
#[derive(Debug)]
enum WEEK {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday
}

fn main() {
    let today: WEEK = WEEK::Sunday;
    //println!("{}", today); // compile error!!!
    println!("{:?}", today);
    //let tomorrow: WEEK = 22; // compile error!!!

    match today {
        WEEK::Sunday => println!("Sleep"),
        _ => println!("Study"), // try again after removing this line
    }
}
```

```bash
$ cargo run --bin enum_basic
warning: variants `Monday`, `Tuesday`, `Wednesday`, `Thursday`, `Friday`, and `Saturday` are never constructed
 --> src/enum_basic/main.rs:4:5
  |
2 | enum WEEK {
  |      ---- variants in this enum
3 |     Sunday,
4 |     Monday,
  |     ^^^^^^
5 |     Tuesday,
  |     ^^^^^^^
6 |     Wednesday,
  |     ^^^^^^^^^
7 |     Thursday,
  |     ^^^^^^^^
8 |     Friday,
  |     ^^^^^^
9 |     Saturday
  |     ^^^^^^^^
  |
  = note: `WEEK` has a derived impl for the trait `Debug`, but this is intentionally ignored during dead code analysis
  = note: `#[warn(dead_code)]` on by default

warning: `my-rust-book` (bin "enum_basic") generated 1 warning
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.14s
     Running `target/debug/enum_basic`
Sunday
Sleep
```

정의하고 사용하는 방법은 C언어와 거의 유사합니다. 하지만 가장 큰 차이는 WEEK타입의 변수에 정말로 WEEK타입의 값인 WEEK::Sunday부터 WEEK::Saturday값 외의 값을 저장하려고하면 컴파일 에러가 발생한다는 것입니다. WEEK타입의 인자를 받는 함수를 사용할 때도 WEEK타입의 값 외에 잘못된 값을 전달할 수 없습니다. 의도하지않은 잘못된 값을 사용하는 것을 방지해줍니다.

그리고 컴파일러가 주는 경고 메세지를 보면 WEEK타입의로 선언된 값들 중에 사용되지 않는 값이 있는 것도 알려줍니다. 또 아주 중요한 기능이 있는데 패턴 매칭에서 처리가 안되는 경우가 있으면 컴파일 에러가 난다는 것입니다.

```rust
......
    match today {
        WEEK::Sunday => println!("Sleep"),
        _ => println!("Study"), // try again after removing this line
    }
......
```

예제에있는 패턴 매칭을 보면 오직 WEEK:Sunday만을 처리하고 있습니다. 그 외의 모든 값은 "Study"라는 메세지를 출력합니다. 만약에 나머지 모든 패턴을 의미하는 "_" 케이스를 제거하면 어떻게 될까요?

```bash
$ cargo run --bin enum_basic
   Compiling my-rust-book v0.1.0 (/home/gkim/study/my-rust-book)
error[E0004]: non-exhaustive patterns: `WEEK::Monday`, `WEEK::Tuesday`, `WEEK::Wednesday` and 3 more not covered
  --> src/enum_basic/main.rs:18:11
   |
18 |     match today {
   |           ^^^^^ patterns `WEEK::Monday`, `WEEK::Tuesday`, `WEEK::Wednesday` and 3 more not covered
   |
note: `WEEK` defined here
  --> src/enum_basic/main.rs:2:6
   |
2  | enum WEEK {
   |      ^^^^
3  |     Sunday,
4  |     Monday,
   |     ------ not covered
5  |     Tuesday,
   |     ------- not covered
6  |     Wednesday,
   |     --------- not covered
7  |     Thursday,
   |     -------- not covered
8  |     Friday,
   |     ------ not covered
   = note: the matched value is of type `WEEK`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern as shown, or multiple match arms
   |
19 ~         WEEK::Sunday => println!("Sleep"),
20 ~         _ => todo!(),
   |

For more information about this error, try `rustc --explain E0004`.
error: could not compile `my-rust-book` (bin "enum_basic") due to 1 previous error
```

위와 같이 WEEK타입에 여러가지 값들이 있는데 그런 값들이 처리되지않고 있다는 에러 메세지를 보여줍니다. 저는 프로젝트가 거대해지고, 다른 사람이 만든 코드를 유지보수할 경우에 실수로 모든 경우에 대한 처리를 하지 않아서 잘 드러나지않는 에러가 나는 경우를 많이 겪어봤습니다. 해결하기에 어려운 문제는 아닙니다만 릴리즈된 후에 이런 문제를 발견하면 새로운 버전을 출시해야하는 번거로움이 있고, 사용자에게 새로운 버전을 설치하라고 안내해야되는 등 후속 처리가 쉽지 않습니다. 이렇게 개발자의 실수를 컴파일러가 방지하는 것이 러스트의 디자인 철학입니다. 사람은 사람이 잘하는 것을 하고 기계는 기계가 잘하는 것을 할 수 있어서 정말 마음에 듭니다.










============================= 2024.09.21 =========================================









### 데이터를 포함하는 열거형

Rust 언어의 열거형(Enums)은 다음과 같이 데이터를 포함할 수도 있습니다.

별도의 쓰레드나 비동기 함수에서 특정 디렉토리를 감시하다가 이런 이벤트를 전달하고, 메인 쓰레드에서 각 이벤트에 맞게 처리하는 프로그램을 상상해보겠습니다.

```rust
enum MyEvent {
    NewFile(String),
    NewData { path: String, contents: String },
    Close,
}

fn events_handler(events: Vec<MyEvent>) {
    for ev in events.into_iter() {
        // ownership is moved!!
        match ev {
            MyEvent::NewFile(path) => println!("New file {} is created", path),
            MyEvent::NewData { path, contents } => {
                println!("New data \"{}\" in file {}", contents, path)
            }
            MyEvent::Close => println!("Event monitor is closed"),
        }
    }
}

fn main() {
    let create_file = MyEvent::NewFile("/root/conf.ini".to_string());
    let write_data = MyEvent::NewData {
        path: "/root/conf.init".to_string(),
        contents: "Hello!".to_string(),
    };
    let close_monitoring = MyEvent::Close;
    let mut events: Vec<MyEvent> = Vec::new();
    events.push(create_file);
    events.push(write_data);
    events.push(close_monitoring);
    events_handler(events);
}
```

이벤트를 표현하는 MyEvent라는 열거형을 만듭니다. 각 이벤트는 다음과 같습니다.

- NewFile: 새로 파일이 생성되는 것을 알리는 이벤트, 파일 이름을 데이터로 전달함
- NewData: 어느 파일에 어떤 데이터가 새로 추가되었는지를 알리는 이벤트, 파일 이름과 추가된 데이터를 전달함
- Close: 이벤트 모니터링을 끝내라는 명령을 전달함

열거형에 특정 타입의 값들 뿐 아니라 각 값에 추가 데이터를 저장할 수 있도록 지원하기 때문에 이렇게 간단하게 처리가 되는 것입니다.

주의할 점은 match에서 MyEvent::NewData {path, contents}와 같이 열거형에 정의된 변수 이름을 똑같이 써줘야 한다는 것입니다.

그리고 events.into_iter를 사용했으므로 이벤트 처리가 끝나면 이벤트에 속한 데이터들이 모두 해지됩니다. 따로 이벤트 안에 데이터가 있는지 없는지 검사하고, 해지하는 등의 처리가 필요없습니다. 만약에 다른 쓰레드로부터 이벤트를 전달받았다고해도, 이벤트의 소유권을 넘겨받았으므로 메모리를 해지하는데 아무런 문제가 없습니다. 이벤트를 생성한 쓰레드는 소유권을 넘기고 다시는 접근하지 못하게 되었으므로, 이벤트를 처리하는 쓰레드에서는 간편하게 아무런 확인작업없이 메모리를 해지할 수 있는 것입니다.

만약 나중에 다른 개발자가 실수로 이벤트 전달 후에 이벤트에 접근하려고하는 코드를 작성하려고해도 쉽게 에러를 찾아낼 수 있습니다. 만약 C같은 언어였으면 동기화 문제를 찾아내는데 얼마나 시간과 노력이 소모될지 알 수 없었을 것입니다.

만약 C나 다른언어였으면 대략 아래와 같은 방식으로 구현했을 것입니다.

```rust
type enum {
    NEWFILE,
    NEWDATA,
    CLOSE,
} EventType;

struct MyEvent {
    EventType type,
    char *path,
    char *data,
};

...skip
if (ev->type == NEWFILE) {
    printf("%s\n", ev->path);
} else if (ev->type == NEWDATA) {
    printf("%s, %s\n", ev->path, ev->data);
} else if (ev->type = CLOSE) {
    call_close_handler();
} else {
    call_error_handler();
}
...
```

이런 처리 방법에는 2가지 문제가 있습니다.

첫번째로 type 필드에 NEWFILE, NEWDATA, CLOSE 외에 다른 정수값이 들어가는 것을 막을 수 없습니다. 따라서 항상 에러처리를 해야합니다.

둘째로 이벤트마다 갖는 path, data라는 메모리가 이벤트 처리와는 별개로 할당되고 해지된다는 것입니다. 이벤트를 처리한다고해도 메모리 해지는 별개입니다. 혹시라도 메모리 처리를 잘못하면 메모리 릭이 발생하게되거나, use-after-free 에러가 발생할 수도 있습니다.

러스트에서는 이벤트가 해지될 때 이벤트에 속한 데이터도 모두 해지해주므로 메모리 릭이 발생할 위험을 없앨 수 있습니다. 소유권 개념을 이해하는게 정말 중요합니다.

## 에러 처리를 위한 Result

열거형의 기본 정의에 대해서 알아봤으니 열거형 타입의 데이터 구조 중에 가장 많이 쓰일만한 Result에 대해서 이야기하겠습니다.

Result의 소스 코드부터 보겠습니다.

```rust
enum Result<T, E> {
   Ok(T),
   Err(E),
}
```

https://doc.rust-lang.org/std/result/

Result는 프로그램 실행 중 발생한 에러를 표현하는 타입입니다. 그 중 가장 대표적인 예가 함수의 반환값입니다.  Result에는 2개의 타입이 존재합니다. (variant라고 부르지만 마땅한 한글 단어가 없어서 타입이라고 부르겠습니다.) Ok는 성공했을 때의 값을 내장하는 타입이고, Err는 실패했을 때의 값을 내장하는 타입입니다. 에러메세지가 될 수도 있고, 에러 상태를 나타내는 데이터가 될 수도 있겠지요.

아주 간단한 예제부터 보겠습니다.

```rust
fn divide(numerator: i32, denominator: i32) -> Result<i32, String> {
    if denominator == 0 {
        return Err(String::from("denominator cannot be zero"));
    }
    Ok(numerator / denominator)
}

fn main() {
    let result = divide(10, 0);
    match result {
        Ok(value) => println!("Result: {}", value),
        Err(message) => println!("Error: {}", message),
    }
}

```

나눗셈이 정상적이면 Ok안에 결과 값을 전달하고, 나눗셈을 실행할 수 없으면 Err타입에 에러 메세지를 넣어서 전달합니다. main함수는 반환값의 타입을 보고, 정상적인 결과인지 문제가 발생한 상황인지를 알 수 있습니다. 타입을 확인하는 것은 패턴 매칭을 이용하면 항상 모든 에러 값을 놓치지 않고 처리할 수 있습니다. 여기서 패턴 매칭의 편리함과 강력함을 다시 느끼게 됩니다.

```rust
fn say_hello() -> Result<String, String> {
    Ok(String::from("hello"))
}
    
fn main() {
    let result = say_hello();
    match result {
        Ok(message) => println!("Say: {}", message),
        Err(message) => println!("Error: {}", message),
    }
}
```

위와 같이 에러 상황이나 정상 상황에서나 반환되는 값이 같은 경우에 Ok나 Err 타입을 가지고 에러 상황을 판단할 수 있어서 쓸모가 있습니다. 사실 C/C++언어에서 포인터를 반환하는 함수들이 에러 상황에 NULL (사실은 정수 0을 다른 이름으로 바꾸기만 한 것)을 반환하는게 보통인데 이게 에러 상황인 것은 나타낼 수 있지만, 왜 에러가 발생했는지를 표현할 수도없고, 실수하기도 쉬운 불편한 방식이었습니다.

NULL이라는 개념을 처음 만들었다는 Tony Hoare님의 후회한다고(https://www.infoq.com/presentations/Null-References-The-Billion-Dollar-Mistake-Tony-Hoare/) 이야기한 것도 그렇고 모던C++ (C++ 17)에서 optional, expected 등을 도입하는 것 등을 보면 Result를 잘 활용하는 것이 얼마나 프로그램의 안정성에 필수적인지 알 수 있습니다.

반드시 반환값을 갖는 함수는 최대한 전부 Result타입으로 반환하도록 작성하려고 노력해보세요.

참고로 Result에서는 한가지 타입의 에러만 반환할 수 있습니다. say_hello에서 반환할 수 있는 에러는 String타입 뿐입니다. 만약에 좀더 긴 함수를 작성하고있고, 이 함수가 몇가지 라이브러리를 호출하는데, 각 라이브러리마다 반환하는 에러의 타입이 다르다면 어떻게 해야할까요? 각 라이브러리마다 자신의 에러를 표현하기 위한 구조체를 만들어서 사용한다면, 모든 에러 값들을 하나의 타입으로 또다시 바꿔야할까요? 뒤에나올 trait라는 것을 사용해서 다양한 에러 타입들을 하나의 타입으로 표현할 수 있습니다. 지금은 어떤 상황에서도 Result를 사용할 수 있다는 것만 기억하시기 바랍니다.

### 반환값이 없는 함수

그럼 반환값이 없는 함수는 Result를 쓸 필요가 없을까요? 다음과 같은 경우를 생각해보겠습니다.

```rust
fn check_command_valid(cmd: &str) -> Result<(), String> {
    match cmd {
        "good" => Ok(()),
        "unsupported" => Err("Unsupported command".to_owned()),
        "bad" => Err("Bad command".to_owned()),
        _ => Err("Wierd command".to_owned()),
    }
}
```

cmd로 전달받은 명령어에 문제가 있다면 에러 메세지를 반환하는 함수입니다. 그리고 문제가 없을 때는 아무 반환값도 없습니다. 이렇게 반환값이 없는 함수라 하더라도 에러 상황에 대한 정보를 전달해야할 때가 많습니다. 이럴 때는 위 예제와 같이 비어있는 값 ()를 반환하도록 하면 됩니다.

## 함수 결과값 반환을 위한 Option

Result는 특정 처리가 성공했냐 실패했냐를 표현할 수 있었습니다. 그런데 모든게 다 성공과 실패로 판단되는 것은 아닙니다. 예를 들어 이전에 처리한 결과 파일을 읽는 경우 처음 실행되는 경우에는 파일이 없을 수 있습니다. 그런 경우는 실패도 아니고 에러 상황도 아닙니다. 굳이 따지자면 에러 상황으로 처리할 수도 있지만 러스트는 보다 더 명확한 처리를 위해 Option이라는 열거형 타입을 제공합니다.

Option의 정의는 값이 있고 없고를 표현하는 타입입니다. 소스 코드를 먼저 확인해보겠습니다.

```rust
enum Option<T> {
    Some(T),
    None,
}

```

https://doc.rust-lang.org/std/option/enum.Option.html

값이 있을 때는 Some타입 안에 존재하는 값을 저장하고, 값이 없을 때는 None으로 표현합니다. 특히 함수 반환 값을 Option으로 반환한다면, 함수를 호출 한 후에는 반드시 제대로 된 값이 있는지 없는지를 확인해야하므로 에러 처리를 확실하게 할 수 밖에 없게됩니다. 보다 안정적인 코드가 될 수 밖에 없습니다.

Result와 마찬가지로 되도록 모든 함수의 반환값을 Option으로 처리할 수 있도록 노력해야합니다.

이제 사용 예제를 한번 보겠습니다.

```rust
let x: Option<i32> = Some(5);
let y: Option<i32> = None;

match x {
    Some(n) => println!("x is {}", n),
    None => println!("x is not present"),
}

match y {
    Some(n) => println!("y is {}", n),
    None => println!("y is not present"),
}

if let Some(n) = x {
    println!("x is {}", n);
}

if let Some(n) = y {
    println!("y is {}", n);
} else {
    println!("y is not present");
}

```

사용법 자체는 크게 어렵지 않습니다. 패턴 매칭을 사용해서 결과 값을 확인하는 것도 Result에서 해본 방식입니다.

그 외에 처음 소개되는 방법이 if let을 사용해서 값을 확인하는 방법입니다. if let 을 사용하면 값이 존재할 때의 처리를 할 수 있고, else에서는 값이 없을 때의 처리를 할 수 있습니다.

### Option이 제공하는 메소드들

제가 처음 러스트를 접하면서 겪은 바로는 처음에 Option을 사용하면 값을 여러번 읽을 때마다 매번 if let이나 패턴 매칭을 사용해서 값이 있는지 없는지를 확인하게되는게 번거로웠습니다. 그래서 저는 간단한 코드를 만들 때는 unwrap이라는 메소드를 자주 사용했었습니다.

unwrap 메소드는 Option 안에 존재하는 값을 꺼내주는 일을 합니다. 만약 Some안에 값이 있다면 값을 반환해주는데, None이라면 패닉을 발생시키고 프로그램을 멈춥니다. 따라서 반드시 값이 있는 상황에서만 사용해야 합니다.

```rust
let x: Option<i32> = Some(5);
let y: Option<i32> = None;

println!("x is {}", x.unwrap());
println!("y is {}", y.unwrap());

```

사용법은 간단합니다. unwrap이라는 메소드를 호출하기만 하면 됩니다. 물론 실제 제품을 개발하는데 unwrap을 사용하면 안됩니다. 굳이 Option에서 값을 꺼내는게 필요하다면 unwrap_or나 unwrap_or_default 등을 사용하면 됩니다.

```rust
let x: Option<i32> = Some(5);
let y: Option<i32> = None;

println!("x is {}", x.unwrap_or(-1));
println!("y is {}", y.unwrap_or_default());

```

i32타입의 디폴트 값은 0입니다. 따라서 “y is 0”이라는 메세지가 출력됩니다.

그리고 unwrap보다 더 권장되는 방식이  expect메소드입니다. 

```rust
let x: Option<i32> = Some(5);
let y: Option<i32> = None;
    
let item = y.expect("slice should not be empty");
```

unwrap은 패닉만 발생시킵니다. 패닉이 발생한 소스 코드 위치는 알 수 있지만 어떤 상황인지 판단할 정보가 부족할 때가 많습니다. expect를 사용하면 직접 에러 메세지를 추가할 수 있습니다. 여기에 다양한 정보를 추가한다면 문제 해결에 큰 도움이 될 수 있습니다.


## ? 연산자

Result와 Option타입을 배웠으면 이제 이렇게 프로그래밍을 하게 될 것입니다.

```rust
fn main() {
    let r = foo();
    match r {
        Ok(n) => println!("Do something with {}", n),
        Err(s) => println!("Do error handling with {}", s),
    }
}

fn foo() -> Result<i32, String> {
    let r = bar();
    match r {
        Ok(n) => {
            println!("Do something with {}", n);
            return Ok(1);
        }
        Err(s) => {
            println!("Do error handling with {}", s);
            return Err(s);
        }
    }
}

fn bar() -> Result<i32, String> {
    let r = foobar();
    match r {
        Ok(n) => {
            println!("Do something with {}", n);
            return Ok(1);
        }
        Err(s) => {
            println!("Do error handling with {}", s);
            return Err(s);
        }
    }
}

fn foobar() -> Result<i32, String> {
    let r = "foobar error".to_string();
    Err(r)
}
```

매번 함수나 라이브러리를 호출할 때마다 패턴 매칭을 실행하고, 동일한 에러 값을 다시 상위 레벨로 전달하는 일을 한다는게 이상하지 않나요? 사실 러스트를 접하기 전까지 저는 C/C++이나 파이썬 프로그래밍을 하면서 지금까지

이렇게 하위 레이어에서 발생한 에러를 상위 레이러로 전달하는게 어쩔 수 없는 필요악이라고 생각했습니다. 단지 개발자가 실수하면 안된다고 생각해서 모든 책임이 개발자에게 있다고만 생각했었습니다.

그런데 러스트 언어에서는 try 연산자라고 부르기도하는 ? 연산자를 제공해줘서, Result나 Option에서의 에러값(Err나 None 모두)를 전달하는 것을 편리하게 해줬습니다. 러스트같이 함수의 반환값을 암묵적으로 무시하지 못하는 언어에서는 정말 필수적인 연산자라고 생각합니다.

그래서 위의 예제는 아래와 같이 바뀔 수 있습니다.

```rust
fn main() {
    let r = foo();
    match r {
        Ok(n) => println!("Do something with {}", n),
        Err(s) => println!("Do error handling with {}", s),
    }
}

fn foo() -> Result<i32, String> {
    let r = bar()?;
    println!("Do something with {}", r);
    return Ok(1);
}

fn bar() -> Result<i32, String> {
    let r = foobar()?;
    println!("Do something with {}", r);
    return Ok(1);
}

fn foobar() -> Result<i32, String> {
    let r = "foobar error".to_string();
    Err(r)
}
```

에러를 확인하는 코드를 모두 없앨 수 있습니다. ?연산자는 값이 None이거나 Err타입이면 바로 현재 함수의 반환값으로 반환해버립니다. 아니면 Ok나 Some안에 저장된 원래 값을 꺼내서 반환해서 계속 처리를 진행하게 해줍니다. 에러 처리뿐 아니라 unwrap이 하는 일까지 같이 해주는 것입니다.

조금 더 실질적인 예제를 하나 더 보겠습니다.

```rust
use std::fs::File;
use std::io::prelude::*;

fn read_file_contents(filename: &str) -> std::io::Result<String> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() {
    match read_file_contents("example.txt") {
        Ok(contents) => println!("File contents: {}", contents),
        Err(error) => println!("Error reading file: {}", error),
    }
}

```

read_file_contents 함수는 std::fs::File 라이브러리의 함수를 호출 할 때마다 ?연산자로 에러 처리를 합니다. 에러가 발생하면 에러가 무엇이든 상관없이 바로 상위 함수로 반환됩니다.

?연산자가 없었다면 read_file_contents 함수에 최소 2개의 match 표현식이 들어갔을 것입니다.

물론 단점도 있습니다. 에러값을 그대로 상위 레벨로 전달하는 경우보다는, 중간 중간 에러 처리를 하는 경우도 많다는 것입니다. 위의 예제에서도 파일을 닫거나 새로 생성하거나 하는 에러 처리를 해야한다면 ?연산자는 쓸 수 없겠지요. 

그래도 제 경험상 에러 처리 코드가 절반가까이 줄어들 수 있었습니다. ?연산자를 더 잘 활용하기 위해 함수를 더 잘게 쪼개고 디자인을 바꾸다보면 더 유연한 코드가 되기도 하니까 적극적으로 활용하시기 바랍니다.

## 함수 포인터와 클로저

### 함수 포인터

다음은 fizzbuzz 함수를 함수 포인터로 구현한 예제입니다.

```rust
fn fizzcheck(n: i32) -> bool {
    n % 3 == 0
}

fn buzzcheck(n: i32) -> bool {
    n % 5 == 0
}
fn fizzbuzz_fn(fizzfn: fn(i32) -> bool, buzzfn: fn(i32) -> bool) {
    for i in 1..=100 {
        if fizzfn(i) && buzzfn(i) {
            println!("FizzBizz");
        } else if fizzfn(i) {
            println!("Fizz");
        } else if buzzfn(i) {
            println!("Buzz");
        }
    }
}

fn main() {
    fizzbuzz_fn(fizzcheck, buzzcheck);
}
```

fn이 하나의 키워드로서 함수 포인터를 나타냅니다. C/C++에서 함수 포인터를 표현할 때

```c
int (*변수이름)(int)
```

위와 같이 사용하는데 함수 포인터라는 타입이 있지만 사실상 타입의 이름이 없다는 것을 알 수 있습니다. 그래서 단순히 변수 이름없이 타입만 정의할 때는 이렇게 사용하기 도 합니다.

```c
int (*)(int)
```

러스트에서는 명확하게 fn이라는 타입 이름을 붙여서 사용합니다. 

저자코멘트) 함수 포인터를 표헌하는 타입이 fn만 있는게 아닙니다. 추후에 소유권에 대한 개념을 설명한 후 Fn, FnMut, FnOnce를 설명하겠습니다.

### 클로저

거의 모든 최신 언어들이 지원하고 있는 클로저를 러스트에서도 사용할 수 있습니다.

```rust
fn fizzbuzz_fn(fizzfn: fn(i32) -> bool, buzzfn: fn(i32) -> bool) {
    for i in 1..=100 {
        if fizzfn(i) && buzzfn(i) {
            println!("FizzBizz");
        } else if fizzfn(i) {
            println!("Fizz");
        } else if buzzfn(i) {
            println!("Buzz");
        }
    }
}

fn main() {
    fizzbuzz_fn(|i| i % 3 == 0, |k| k % 5 == 0);
}
```

fn이라는 키워드가 함수 포인터를 위한 키워드일뿐 아니라 클로저를 위한 키워드도 된다는 것을 알 수 있습니다.


## map 메소드

클로저를 사용하는 방법중에 가장 많이 사용하게 되는게 배열이나 벡터의 이터레이터의  map 메소드와 사용하는 방법일 것입니다. 그리고 Option과 사용하는 것도 자주 사용되는 방식이니까 이 두가지 경우를 이야기해보려고 합니다. 

### 배열과 collect

배열이나 range, 벡터등에서 각 데이터에 접근하기 위한 방법으로 for 루프대신 map을 사용하는게 더 편리할 때가 많습니다. 그리고 많은 경우에 map을 이용하는게 처리 속도가 더 빠르기도 합니다.

가장 간단한 예를 가지고 시작해보겠습니다. 다음 예제는 이전에 패턴 매칭의 예제로 만들어봤던 fizzbuzz 함수를 이터레이터와 map으로 다시 만들어본 예제입니다.

```rust
fn fizzbuzz_2(max: i32) {
    for i in 1..=max {
        match (i % 3, i % 5) {
            (0, 0) => println!("{} - FizzBuzz", i),
            (0, _) => println!("{} - Fizz", i),
            (_, 0) => println!("{} - Buzz", i),
            (_, _) => (),
        }
    }
}

fn fizzbuzz_3(max: i32) {
    let ret = (1..=max)
        .into_iter()
        .map(|i| match (i % 3, i % 5) {
            (0, 0) => format!("{} - FizzBuzz\n", i),
            (0, _) => format!("{} - Fizz\n", i),
            (_, 0) => format!("{} - Buzz\n", i),
            (_, _) => "".to_string(),
        })
        .collect::<Vec<String>>()
        .join("");
    println!("{}", ret);
}
```

1부터 max까지의 각 숫자들에 대해 한번씩 map에 지정된 클로저를 호출하는 것입니다. 프로그래밍 언어와 상관없이 조금의 개발 경험만 있다면 이해할 수 있을 것입니다.

하지만 러스트 언어를 사용하기 위해 주의해야할 점이 있는데 collect 메소드를 호출해야한다는 것입니다. map메소드는 반환값으로 이터레이터를 반환합니다. 즉 이터레이터를 받아서 처리하고 또 다른 이터레이터를 반환하는 것이 map이 하는 일입니다. 그러면 ret 변수에 저장되는 값은 이터레이터가 됩니다. 이터레이터를 최종적으로는 값의 집합으로 바꿔서 벡터를 만들어야 map에 지정된 클로저가 호출됩니다.

언어마다 이터레이터의 연산이 실행되는 시점이 다릅니다만, 러스트에서는 collect메소드가 호출되었을 때 최종적으로 map에 지정된 연산을 실행하여 결과값을 생성합니다. 위의 fizzbuzz_3함수에서는 최종적으로 생성하는 값이 문자열의 벡터이기 때문에 collect에게 다음과 같이 반환값의 타입을 알려줍니다.

```rust
collect::<Vec<String>>()
```

<Vec<String>> 부분이 바로 반환값이 타입을 지정하는 부분입니다.

러스트 언어의 매뉴얼에는 다음과 같은 예제 코드가 있습니다.

```rust
let a = [1, 2, 3];

let mut iter = a.iter().map(|x| 2 * x);

assert_eq!(iter.next(), Some(2));
assert_eq!(iter.next(), Some(4));
assert_eq!(iter.next(), Some(6));
assert_eq!(iter.next(), None);
```

https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.map

이 예제가 보여주는 것이 바로 map 메소드가 호출되는 순간에 클로저가 호출되고 연산이 발생하지 않는다는 것입니다. next 메소드가 호출되면 그때 한번의 연산이 발생하고 하나의 값을 반환하는 것입니다. 물론 연산에 필요한 입력 데이터가 있을 수도 있고 없을 수도 있습니다. 그럴때 사용하는 것이 바로 Option 타입입니다. 따라서 next메소드는 Option 타입의 값을 반환하는 것입니다. next 메소드가 호출되면 입력 데이터를 하나씩 하나씩 클로저에 넘겨서 처리를 하고 마지막으로 남은 입력데이터가 없으면 None을 반환합니다.

그렇게 한번씩 호출되는 연산을 한꺼번에 모두 실행하고 모든 결과값을 벡터에 담아서 전달하는 메소드가 collect 입니다.

상황에 따라서 next를 사용할 수도 있고 collect를 사용할 수도 있습니다.

다시한번 주의해서 생각해야할 것이 바로 이터레이터를 생성할 때 iter 메소드를 사용할 것인지, into_iter를 사용할 것인지 판단하는 것입니다. 아래 예제를 가지고 다양하게 실험해보면서 연습해보시기 바랍니다.

```rust
fn ownership(nums: Vec<i32>) {
    // What happens if switch .iter and .into_iter?
    // What are the types of first i and the second i?
    let ret = nums
        .iter()
        .map(|i| format!("{}", i))
        .collect::<Vec<String>>()
        .join("");
    println!("{}", ret);

    let ret = nums
        .into_iter()
        .map(|i| format!("{}", i))
        .collect::<Vec<String>>()
        .join("");
    println!("{}", ret);
}
```

### Option와 map

이터레이터뿐 아니라 Option 타입도 map메소드를 가지고 있습니다.

```rust
fn main() {
    let some_number = Some(5);
    let none_number: Option<i32> = None;

    let double_some = some_number.map(|x| x * 2);
    let double_none = none_number.map(|x| x * 2);

    println!("Double Some: {:?}", double_some); // Double Some: Some(10)
    println!("Double None: {:?}", double_none); // Double None: None
}
```

보통 함수의 반환값으로 Option타입의 값을 받겠지만 이 예제에서는 일단 Some타입과 None타입의 2가지 변수를 만들었습니다. 그리고 각각 map메소드를 호출해주었습니다.

이 예제 소스는 워낙 간단하니까 우리 눈에 변수가 Some타입일지 None타입일지 알 수 있지만, 당연히 보통의 경우에는 어떤 함수의 반환값이 어느 타입일지는 알 수 없습니다. 그러면 매번 패턴 매칭이나 if let을 사용해서 값을 꺼내서 필요한 연산을 해주게 되면 코드가 길어질 것입니다. 코드가 길어진다는 것은 읽기 힘들어지고, 에러가 날 경우도 많아진다는 것입니다. 단순히 성능의 최적화를 위해 코드를 짧게 유지하는게 필요한게 아니라, 읽기 좋고 버그가 적은 코드를 만들기 위해서도 코드를 짧게 유지하는게 좋습니다.

Option의 메소드인 map은 타입이 Some일때는 값을 꺼내서 클로저를 호출해주고 결과값을 Option타입으로 반환해줍니다. None 타입을 위해 호출되면 아무런 처리도 하지않고 None을 그대로 반환해줍니다.

그리고 Some이나 None 값을 출력하기 위해서 “{:?}” 리터럴을 사용하는 것도 알아두시면 좋습니다.

### Option의 as_ref 메소드

마지막으로 주의해야 할 점이 하나 있는데 바로 map을 호출하면 객체가 해지된다는 것입니다. 영어로는 consume이라고 표현하는데, 그 의미는 자기 자신의 값을 소비해서 없애버리고 반환값을 생성한다는 것입니다.

```rust
let maybe_some_string = Some(String::from("Hello, World!"));
// `Option::map` takes self *by value*, consuming `maybe_some_string`
let maybe_some_len = maybe_some_string.map(|s| s.len());
assert_eq!(maybe_some_len, Some(13));
//println!("{:?}", maybe_some_string); // error
```

https://doc.rust-lang.org/std/option/enum.Option.html#method.map

maybe_some_string은 소비consume되어버렸으니 map연산을 호출한 이후에는 다시 사용할 수 없는 변수가 됩니다. 따라서 map 메소드를 호출하는 것은 이 값을 Some(x) 값을 Some(y) 값으로 바꾸고 이전 값은 다시 사용할 필요가 없을 때 사용합니다. 대부분의 값들이 최종 값을 얻기 위한 중간 결과물이니까 이런 방식의 처리를 하도록 디자인되었을 것입니다. maybe_some_string이라는 객체가 더 이상 필요하지 않으면 괜찮지만 만약 계속 써야하는 데이터라면 객체가 해지되지 않도록 해야합니다.

그럼 map을 써도 원본 객체가 해지되지 않으려면 어떡해야할까요? 답은 컴파일러가 이미 알려주고 있습니다. 아래는 위 예제의 마지막 줄을 주석처리하지않고 빌드했을 경우 에러 메세지입니다.

```rust
error[E0382]: borrow of moved value: `maybe_some_string`
    --> main.rs:60:22
     |
56   |     let maybe_some_string = Some(String::from("Hello, World!"));
     |         ----------------- move occurs because `maybe_some_string` has type `Option<String>`, which does not implement the `Copy` trait
57   |     // `Option::map` takes self *by value*, consuming `maybe_some_string`
58   |     let maybe_some_len = maybe_some_string.map(|s| s.len());
     |                          ----------------- ---------------- `maybe_some_string` moved due to this method call
     |                          |
     |                          help: consider calling `.as_ref()` or `.as_mut()` to borrow the type's contents
59   |     assert_eq!(maybe_some_len, Some(13));
60   |     println!("{:?}", maybe_some_string);
     |                      ^^^^^^^^^^^^^^^^^ value borrowed here after move
```

중간에보면 as_ref 메소드를 호출해서 객체의 레퍼런스를 만든 후에 map 메소드를 호출하라고 알려줍니다. 객체의 값으로 map을 호출하면 객체가 해지되니, 레퍼런스를 통해서 map 메소드를 호출하면 객체가 해지되지 않는다는 것을 알려줍니다.

```rust
let maybe_some_string = Some(String::from("Hello, World!"));
// `Option::map` takes self *by value*, consuming `maybe_some_string`
let maybe_some_len = maybe_some_string.as_ref().map(|s| s.len());
assert_eq!(maybe_some_len, Some(13));
println!("{:?}", maybe_some_string);
```

as_ref 메소드를 사용한 후에는 정상적으로 빌드됩니다.

### Option의 as_deref, as_deref_mut 메소드

이번에는 또 다른 경우를 보겠습니다. 아마 가장 자주 접하게 되는 경우일텐데 Option안에 있는 객체를 수정하는 것입니다.

```rust
maybe_some_string
    .as_deref_mut()
    .map(|x| x.make_ascii_uppercase());
println!("{:?}", maybe_some_string)
```

as_deref_mut 메소드는 Option은 그대로 유지한채 그 안의 객체를 수정할 수 있도록 &mut 레퍼런스를 넘겨줍니다. 이 예제 코드에서 map메소드에서 받은 x의 타입은 &mut str 타입이 되서 "Hello, World!"라는 스트링 자체를 수정할 수 있게 됩니다. 

참고로 Option을 그대로 유지한 채 그 안의 객체에 대한 레퍼런스를 얻을 수 있는 as_deref라는 메소드도 있습니다.

```rust
    let ref_to_string: Option<&str> = maybe_some_string.as_deref();
```

as_deref 메소드는 객체에 대한 레퍼런스가 필요할 때 사용됩니다. 예를 들어 값을 그대로 다른 함수나 쓰레드 등에 전달하는게 비효율적이니 레퍼런스를 만들어서 전달하는데 그럴때 사용됩니다.

### Result의 map_err 메소드

map_err은 map과 반대로 Result의 값이 에러일때 실행할 코드를 지정하는 것입니다.

하나의 예를 들면 아래처럼 serde_json에서 전달된 에러를, 자신이 정의한 에러 타입으로 변환할 때 사용할 수 있습니다.

```rust
let new_value = serde_json::to_string(&row).map_err(|e| {
                            MyError::StorageMsg(format!(
                                "failed to serialize row={:?} error={}",
                                row, e
                            ))
                        })?;
```

위의 예제에서 만약 serde_json::to_string메소드의 반환값이 에러가 아니라면 ? 연산자는 Ok()안에 있는 문자열의 값을 Ok밖으로 꺼내서 new_value에 저장합니다.

하지만 serde_json::to_string 메소드가 에러를 반환하면 그것을 MyError::StorageMsg라는 타입으로 변환합니다. 결국 map_err은 Err(MyError::StorageMsg)타입의 에러를 반환하고 ? 연산자는 에러 값을 상위 함수로 전달합니다.

이렇게 에러 상황일때만 실행될 코드를 지정할 때 map_err을 사용합니다.

### 연습문제

1. Option의 메소드 중에는 map외에 map_or와 map_or_else가 있습니다. 지금까지의 예제들을 map_or나 map_or_else 로 바꿔보세요.
2. 이터레이터의 메소드중에 for_each라는 메소드가 있습니다. map과 for_each는 어떤 차이가 있을까요? 힌트는 for_each호출에는 collect가 필요하지 않다는 점입니다. 왜 필요하지 않을까요?

## 프로젝트 관리

이번 장에서는 여러개의 파일에 코드를 나누어서 관리하는 방법을 알아보겠습니다.

### Crate 크레이트와 Package 패키지

러스트 컴파일러(rustc)가 한번에 처리하는 코드를 크레이트라고 정의한다고 합니다.  (https://doc.rust-lang.org/book/ch07-01-packages-and-crates.html) 사실 정의만 보면 잘 이해가 안되는데 쉽게 말해서 지금 내가 만들고있는게 하나의 실행 파일이나, 하나의 라이브러리이면 이게 바로 하나의 크레이트입니다. 컴파일러가 빌드해서 하나나 여러개의 결과물을 생성하는게, 그 각각을 크레이트라고 부른다는 것입니다. 

우리는 지금까지 하나의 실행파일이 생성되는 예제들을 만들었습니다. 그럼 지금까지 하나의 크레이트를 만들었다는 것입니다. Binary crate바이너리 크레이트는 말 그대로 실행 파일 하나를 만드는 코드입니다. Library crate는 라이브러리를 만들기 위한 코드입니다. 코드가 파일 하나에만 있던지 여러개에 있던지는 상관없습니다. 여러 코드 파일들이 하나의 결과물을 만들면, 모든 파일이 하나의 크레이트를 구현하는 것입니다.

패키지는 필요에 따라 여러 크레이트를 모아놓은 것입니다. Cargo를 이용해서 빌드를 하면 Cargo.toml파일에 가장 먼저 [package]라고 패키지 정보를 셋팅합니다. 그것은 내가 Cargo를 이용해서 하나의 패키지를 만든다는 뜻입니다.

그런데 왜 패키지일까요? 예제 프로그램만 만들다보면 다른 라이브러리를 사용할 일이 없었을 것입니다. 그럼 하나의 크레이트만 있는 패키지를 만드신 것입니다. 그리고 그 하나의 크레이트는 자신이 만든 크레이트인 패키지입니다. Cargo.toml파일의 [dependendies] 섹션에 외부 라이브러리를 추가하게되면, 하나의 바이너리 크레이트와 여러개의 라이브러리 크레이트로 이루어진 패키지를 만들게되는 것입니다. 당연히 여러개의 바이너리를 하나의 Cargo.toml에서 빌드할 수 있습니다. 그럼 여러개의 라이브러리 크레이트와 여러개의 바이너리 크레이트로 구성된 패키지를 만드는 것입니다.

이전에 Cargo를 사용해서 패키지 디렉토리를 생성하는 방법을 이야기했었습니다. cargo new <package-name> 명령을 사용하면 된다고 이야기했었는데요 사실은 —bin옵션을 생략한 것입니다.

```rust
study % cargo new mybin --bin
     Created binary (application) `mybin` package
study % ls -R mybin
Cargo.toml	src

mybin/src:
main.rs
```

라이브러리 패키지를 만들때는 —lib옵션을 사용합니다. src/main.rs대신에 src/lib.rs를 만들어줍니다.

```rust
study % cargo new mylib --lib
     Created library `mylib` package
study % ls -R mylib
Cargo.toml	src

mylib/src:
lib.rs
```

### Modules 모듈

패키지와 크레이트는 라이브러리나 실행 파일등의 최종 결과물을 생성하는 단위입니다. 하나의 프로젝트 안에서 여러개의 파일들이 있을때 참조하는 방법은 모듈이라는 방식을 사용합니다. 네임스페이스에 익숙한 분들은 비슷한 것이라고 생각해도 될듯합니다.

아래 예제를 보면 네임스페이스나 기타 언어들이 다른 파일의 함수나 변수등에 접근하는 방식과 유사하다는 것을 알 수 있습니다.

```rust
fn main() {
    my_module::test_my_mod();
}

mod my_module {
    pub fn test_my_mod() {
        println!("This is my_module::test_my_mod()");
    }
}
```

예제에서 my_module이라는 모듈안에 구현된 test_my_mod 함수는 pub이라는 키워드를 붙여야 모듈 밖에서도 참조가 가능합니다. 그리고 특정 모듈안의 함수 등을 참조할 때는 <모듈이름>::<이름> 같은 방식으로 접근이 가능합니다.

만약 모듈 이름이 길거나 모듈안에 다른 모듈이 있거나 해서 이름이 길어지는 경우 아래와같이 use 키워드를 사용해서 모듈 경로를 생략할 수도 있습니다.

```rust
use my_module::test_my_mod;

fn main() {
    test_my_mod();
}

mod my_module {
    pub fn test_my_mod() {
        println!("This is my_module::test_my_mod()");
    }
}
```

그럼 다른 파일에 있는 함수 등은 어떻게 접근할까요? 실험을 위해 아래와 같이 my_module.rs 파일을 새로 추가합니다.

```rust
% ls src
main.rs         my_module.rs
```

my_module.rs 파일에 아래와같이  my_module에 정의했던 함수들을 옮겨줍니다. 주이할 것은 mod my_module 선언을 따로 해주지않고 바로 함수 정의를 시작한다는 것입니다. 파일 하나가 하나의 모듈이 되기 때문입니다. 파일 이름이 my_module.rs이기때문에 my_module이라는 모듈이 자동으로 선언된 것입니다.

```rust
pub fn test_my_mod() {
    println!("This is my_module::test_my_mod()");
}
```

main.rs에서 my_module을 참조하기 위해서는 아래와같이 mod <모듈 이름>을 사용합니다.

```rust
mod my_module;

fn main() {
    my_module::test_my_mod();
}
```

use 키워드를 사용할 수도있는데 mod로 모듈 참조를 선언한 이후에 use 키워드를 사용할 수 있습니다.

```rust
mod my_module;
use my_module::test_my_mod;

fn main() {
    test_my_mod();
}
```

만약 소스 디렉토리를 분리하고 싶다면 아래와 같이 각 하위 디렉토리마다 mod.rs라는 파일을 만들어야 합니다. 그리고 mod.rs에 같은 디렉토리에 있는 파일들을 참조해야합니다.

간단한 실험을 위해 아래와 같이 src/second_mod 라는 디렉토리를 만듭니다. 그리고 [mod.rs](http://mod.rs) 파일과 sec_mod_file.rs 파일을 만듭니다.

```rust
% ls src/second_mod 
mod.rs          sec_mod_file.rs
```

mod.rs 파일을 자신과 같은 디렉토리에 있는 모듈들을 모아서 참조하는 일을 합니다. 아래와 같이 sec_mod_file.rs 파일을 public으로 참조합니다.

```rust
pub mod sec_mod_file;
```

sec_mod_file.rs에는 main에서 호출된 함수를 하나 만들어줍니다.

```rust
pub fn second_module() {
    println!("Here second-module");
}
```

이제 main.rs에서 어떻게 참조할 수 있는지 확인해보겠습니다.

```rust
mod my_module;
mod second_mod;

use my_module::test_my_mod;

fn main() {
    test_my_mod();
    second_mod::sec_mod_file::second_module();
}
```

“mod second_mod” 와 같이 디렉토리 이름으로 모듈을 참조합니다. 그리고 함수를 호출할 때는 <모듈이름>::<파일이름>::<함수이름>으로 호출할 수 있습니다.

### 참고 링크

- Cargo 사용법: https://doc.rust-lang.org/cargo/index.html

### 연습문제

1. 아래와같이 second_module함수를 모듈 이름을 붙이지 않고 호출할 수 있도록 고쳐보세요.

```rust
mod my_module;
mod second_mod;

use my_module::test_my_mod;
// 연습문제1

fn main() {
    test_my_mod();
    second_module();
}
```

1. second_module안에 sec_mod_file.rs파일외에 sec_mod_file2.rs 파일을 만들고 간단한 함수를 추가하고, main함수에서 호출해보세요.
2. second_module안에 하위 모듈 third_module을 추가해보세요.

## Trait 트레이트

트레이트은 객체에 특정한 메소드를 구현하도록 지정하는 방법입니다. 서로 다른 타입의 객체들이라고 해도 같은 트레이트을 구현하도록 만들면 공통된 특징을 갖도록 만들 수 있습니다. 결국 함수 인자로 객체를 넘길 때, 객체의 타입을 지정하는게 아니라, 트레이트을 구현한 모든 객체 타입을 지정할 수 있게 됩니다. 다른 객체지향 언어를 경험해봤다면 인터페이스를 이야기한다는 것을 눈치채셨겠지요. 다양한 타입의 객체들을 묶는 추상화를 할 수 있기도 하고, 코드를 재사용할 수도 있는 등 러스트로 규모있는 프로그램을 만들기 위해서는 필수적으로 잘 활용할 수 있어야되는 문법입니다.

아주 간단한 예제를 가지고 시작해보겠습니다. 다음 예제는 서로 다른 타입의 두 구조체 Person과 Book에 공통의 트레이트 Printable을 구현하는 예제입니다.

```
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
        println!("Name: {}, {} years old", self.name, self.age);
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
            self.title, self.author, self.published
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

첫번째로 Printable이라는 트레이트를 선언하는 코드가 나옵니다.

```rust
trait Printable {
    type Age;
    fn print(&self);
    fn get_age(&self) -> Self::Age;
}
```

Printable이라는 트레이트에는 2개의 함수와 1개의 타입이 포함됩니다. 이제 이 트레이트를 구현할 구조체들은 2개의 함수를 정의해야합니다. 그리고 Age라는 타입을 무슨 데이터 타입으로 사용할 것인지를 정해야합니다. i32같은 기본 타입을 사용할 수도있고, 새로운 구조체를 만들어서 사용할 수도있습니다.

그러니 트레이트라는 것이 반드시 특정 함수를 구현하도록 하는 것만이 아니라, 어떤 데이터 타입을 쓸지도 정할 수 있다는 것을 기억하시기 바랍니다.

다음으로는 Person 구조체의 구현 코드가 나옵니다.

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

impl Printable for Person {
    type Age = u32;
    fn print(&self) {
        println!("Name: {}, {} years old", self.name, self.age);
    }
    fn get_age(&self) -> Self::Age {
        self.age
    }
}
```

우선 Person타입이 트레이트와 상관없이 가지는 Person타입만의 고유한 함수 new를 구현합니다. 그리고 그 다음으로 Printable 트레이트를 구현하는 코드가 나옵니다.

먼저 Age라는 타입을 정의합니다. 타입 이름은 Age이지만, 사실은 u32타입의 alias별칭이라고 생각하면 됩니다. 그리고 print 함수와 get_age함수의 구현이 있습니다. get_age함수는 Age타입을 반환해야하니까 결과적으로는 u32타입을 반환하는 것이 됩니다.

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
            self.title, self.author, self.published
        );
    }
    fn get_age(&self) -> Self::Age {
        self.published
    }
}
```

Age타입을 u32로 정의했습니다. 왜 Person과 Book의 Age 타입을 같은 u32로 정의했는지는 곧 알게됩니다. 그리고 print함수와 get_age함수의 구현이 나옵니다. 아주 단순한 예제 함수이니까 이해하는데 어려움은 없을거라 생각합니다.

이제 정말 중요한게 나옵니다. 바로 dyn이라는 키워드입니다.

```rust
fn print_info(item: &dyn Printable<Age = u32>) {
    item.print();
}
```

print_info라는 일반 함수인데, 인자의 타입이 특이합니다. dyn이라는 키워드가 있는 참조 타입입니다. “dyn Printable”의 의미는 Printable 트레이트를 구현한 객체입니다. 참조 키워드 &가 있으므로 Printable 트레이트를 구현한 객체를 참조하겠다는 뜻이 됩니다. 그리고 하나가 더 있는데 <Age=u32>가 추가되어서, Age타입을 u32로 정의한 객체만 참조하겠다는 것이 됩니다.

정리하자면 Printable 트레이트를 구현하되 Age타입을 u32로 정의한 객체들을 참조하게됩니다. print_info함수를 호출할 때 서로 다른 타입 Person과 Book의 참조를 전달할 수 있게되는 이유가 바로, Person과 Book이 Printable 트레이트를 구현했고, Age타입을 u32로 정의했기 때문입니다. 만약 Printable 트레이트를 구현했다해도, Age의 타입이 u32이 아닌 다른 타입이었다면 print_info함수에 전달할 수 없습니다.

러스트 컴파일러는 print_info함수가 호출되는 코드를 만날 때마다, 전달되는 인자의 타입을 보고 Printable틀이브를 구현했는지, Age타입을 u32로 사용하는지를 확인합니다. 컴파일 시간은 길어질 수 있지만, 추상화에 필요한 코드가 더해지지않으므로 더 빠른 코드를 만들 수 있습니다. 이렇게 컴파일타임에 다형성을 체크하는 것을 정적 다형성이라고 합니다만, OOP프로그램에 익숙하지않다면, 그냥 트레이트를 구현하는 객체의 참조나 포인터로 이해하는 것도 괸찬을거라 생각합니다.

만약에 여러개의 트레이트를 구현하는 타입들을 지정하고 싶다면 아래와같이 사용하면 됩니다.

```rust
fn some_function(param: &(dyn Trait1 + Trait2)) {
    // Function body
}
```

### 연관 타입(Associated Type)과 연관 함수 (Associated Function)

이전 예제에서 Printable 트레이트에 있는 Age 타입을 연관 타입이라고 부릅니다. 트레이트 구현에 공통적으로 필요한 변수가 있는데 어떤 타입이 될지는 구현에 따라 달라질 수 있습니다. print_info 함수에서와 같이 특정 연관 타입을 동일하게 갖는 객체들을 공통적으로 사용할 수 있습니다.

그리고 Person구조체에 있는 new 함수를 연관 함수라고 부릅니다. 자세히보면 함수의 인자에 self가 없습니다. 그러므로 특정 구조체 객체에 종속되서 동작하는게 아니라 객체가 없는 상태에서 Person::new와 같이 호출할 수 있습니다. 보통 new라는 이름으로 객체를 생성하는 연관 함수를 만드는게 관례입니다.

사실 이름이 어떻든 직관적으로 사용할 수 있는 것들이지만 제대로된 명칭정도는 알고있는게 협업하는데 필요할 것이라고 생각해서 소개했습니다. 둘 다 실제 구현되기 이전에 존재한다는 특징이 있습니다. 

### Trait object 트레이트 객체

dyn키워드는 트레이트 객체를 나타내는 키워드라고 설명했습니다. 이 트레이트 객체를 좀더 자세히 이해할 필요가 있습니다. 자연스럽게 dyn 키워드 다음에는 대부분 트레이트의 이름이 나오겠지요.

```rust
dyn TraitName
```

다음에 제너릭을 배우면 좀더 간단한 표현을 배우겠지만, 특정 트레이트를 구현한 객체를 가르키는 것은 동일합니다.

위의 예제에서 print_info함수는 전달받은 item 객체의 print함수를 호출합니다. 이게 어떻게 가능할까요?

함수를 호출한다는 것은 사실 어떤 코드 지점으로 점프한다는 것입니다. 더 정확하게 말하면 어셈블리어 jmp등을 이용해서 특정 메모리 주소로 EIP레지스터의 값을 바꾸는 것입니다. 그러면 그 메모리 지점에 있는 코드가 하나씩 실행됩니다. item.print라는 코드에서 Book이라는 객체의 print함수의 주소를 찾아낼 수 있을까요? Person타입의 print함수는 0x1000_0000에 있고, Book타입의 print함수는 0x3000_0000에 있다고 생각해봅시다. 그럼 item은 Person객체의 주소가 될 것이니 print_info함수의 코드를 jmp 0x1000_0000으로 만들면 될까요?

안되겠지요. 그러면 Book의 print함수를 호출할 수 없게됩니다. 컴피일러는 보통 vtable이라는 것을 만들어서 트레이트에서 구현된 함수들의 포인터를 관리합니다. (이것은 자바나 C++의 인터페이스와 동일합니다.)

```rust
book.vtable.print = 0x1000_0000;
person.vtable.print = 0x3000_0000;

fn print_info(item: &dyn Printable<Age = u32>) {
    jmp item.vtable.print
}
```

위와같이 트레이트를 구현하는 객체마다 vtable을 추가해서 트레이트 함수들의 포인터를 저장합니다. 그러면 같은 트레이트를 구현하는 객체마다 동일한 위치에 함수 포인터가 저장될 것이므로, 객체의 타입에 상관없이 트레이트 함수를 호출할 수 있게됩니다. 이것을 Dynamic dispatch라고 부릅니다만 용어보다는 공통된 특성 혹은 함수들의 구현을 위해 추가적인 데이터가 들어간다는 것을 알고있는게 중요할 것입니다.

물론 변수를 한번 더 읽게되는 단점도 있습니다만 특별하게 CPU를 많이 사용하는 연산을 수행하는 부분에서 호출되지 않는한 별 차이는 없을 것입니다.

약간 내부 구현에 관해 생각해봤지만, 굳이 거기까지 이해할 필요는 없습니다. 특정 함수나 타입을 공통으로 구현하는 공통된 특성을 가진 타입들을 사용할 수 있다는 추상적인 의미를 이해하는 것으로도 충분합니다.

### 표준 라이브러리에 포함된 트레이트

트레이트를 직접 만들 수 있지만, 러스트의 표준 라이브러리에서 미리 정의해놓은 트레이트들이 있습니다. 그만큼 어떤 상황에서도 만들어놓으면 좋은 트레이트겠지요. 여러개가 있지만 그중에서 몇가지만 예제를 만들어보겠습니다.

참고 링크: https://youtu.be/Nzclc6MswaI

#### Display 트레이트

참고 링크: https://doc.rust-lang.org/std/fmt/trait.Display.html

Display라는 트레이트가 있습니다. 구조체 타입을 println! 등의 출력 함수에서 곧바로 출력할 수 있도록 만드는 트레이트입니다.

```rust
use std::fmt;

pub trait Display {
    // Required method
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>;
}
```

위에서 Printable이라는 트레이트를 만들어봤는데 동일한 일을 하는 트레이트입니다. Book타입의 Display트레이트를 구현해보겠습니다.

```rust
use std::fmt;

struct Book {
    title: String,
    author: String,
    published: u32,
}

impl fmt::Display for Book {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Title: {}\nAuthor: {}\nPublished: {}\n",
            self.title, self.author, self.published
        )
    }
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

Display라는 트레이트는 fmt 하나의 함수로 이루어져있습니다. 함수 인자로 std::fmt에 선언된 Formatter라는 구조체가 들어가는데 아직은 자세하게 설명할 수 있는 것은 아닙니다. 그리고 굳이 자세히 알 필요도 없습니다. 그냥 잘 써먹으라고 충분히 추상화되어있는 것이니 사용법에 맞게 쓰기만 하면 됩니다.

```rust
write!(f, "내가 쓰고싶은 메세지{}", 출력할 데이터)
```

이런 형태로 얼마든지 자유롭게 쓰기만하면 됩니다. 주의할 것은 write!호출 후에 “;” 세미콜론을 쓰지 않는 다는 점입니다. fmt함수의 반환값이 fmt::Result입니다. 그러므로 해당 타입의 값을 반환해야합니다. write! 매크로 함수가 fmt::Result 값을 반환하므로 “;” 세미콜론을 쓰면 안됩니다. 만약 쓰게되면 아무런 값도 반환하지 않게되서 컴파일 에러가 발생할 것입니다.

#### Debug 트레이트

참고링크: https://doc.rust-lang.org/std/fmt/trait.Debug.html

Debug트레이트라는 것도 있습니다. 참고 링크를 보시면 Display와 다를게 없어보입니다.

```rust
use std::fmt;

pub trait Debug {
    // Required method
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>;
}
```

그럼 Display 트레이트와 뭐가 다른 걸까요? 아래에 Debug트레이트를 직접 구현한 예제를 잘 보시면 다른 점이 하나 있습니다.

```rust
use std::fmt;

struct Book {
    title: String,
    author: String,
    published: u32,
}

impl fmt::Debug for Book {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Title: {}\nAuthor: {}\nPublished: {}\n",
            self.title, self.author, self.published
        )
    }
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

예 println에서 사용하는 포맷이 다릅니다. 보통 println에서 사용하는 포맷은 “{}”입니다만 Debug 트레이트를 사용할 때는 “{:?}”가 됩니다.

그리로 사실 Debug트레이트는 이전에 구조체를 설명하면서 한번 사용해봤었습니다. 그때는 지금처럼 직접 구현하지않아도 사용할 수 있었습니다. 지금 우리가 만든 Book 구조체에도 사실은 직접 구현할 필요없이 다음처럼 derive를 사용해서 트레이트를 구현할 수 있습니다.

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

이렇게 derive를 이용해서 자동으로 Debug 트레이트를 구현하면 좋은 점이 있습니다. 위 예제를 실행해보면 좀더 자세한 정보가 나오는 것을 알 수 있습니다.

```rust
Book { title: "The Rust Programming Language", author: "Steve Klabnik and Carol Nichols", published: 20230228 }
```

구조체의 이름과 각 필드의 이름이 자동으로 출력됩니다. 따라서 Debug트레이트는 직접 구현하는 것보다 derive를 이용하는 것을 추천합니다.

만약 굳이 구조체의 정보를 출력해야한다면 Display 트레이트를 구현하는 것을 추천합니다.

#### Clone

참고링크: https://doc.rust-lang.org/std/clone/trait.Clone.html

Clone 트레이트는 clone이라는 메소드를 구현하는 것인데, 간단하게 설명하면 바로 deep copy를 수행하는 것입니다.

```rust
pub trait Clone: Sized {
    // Required method
    fn clone(&self) -> Self;

    // Provided method
    fn clone_from(&mut self, source: &Self) { ... }
}
```

한가지 먼저 알아야되는게 있습니다. self와 Self의 차이입니다. &str과 String의 차이와 비슷합니다. self는 객체 자신을 가르키는 변수입니다. 그리고 Self는 객체의 타입입니다. &str이 스트링 객체의 참조이고, String이 문자열의 타입인것과 같습니다.

그래서 clone함수는 자기 자신을 참조하면서, 같은 타입을 반환하는 함수입니다. 그럼 한번 직접 만들어보겠습니다.

```rust
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
}
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

만약 Copy 트레이트가 구현되어있다면 clone 메소드를 호출해서 자동으로 객체 복사를 해주게되지만, String 타입은 Copy 트레이트를 구현하지않았으므로 위와같은 에러가 발생합니다.

그리고 published는 u32타입의 변수이므로 clone이 필요없이 값이 복사됩니다.

```rust
fn print_info(item: &dyn Clone) {
    println!("item implements Clone trait");
}
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

컴파일러 메세지가 약간 이해하기 어렵지만 간단하게 생각해보면 이해할 수 있습니다. 트레이트 객체로 전달한다는 것은 원래의 타입이 뭔지를 숨기고 여러 타입이 공통으로 구현한 함수를 호출하는 것입니다. 만약 구현하는 함수가 clone 메소드처럼 Self를 반환한다면, 트레이트 객체의 원래 타입이 뭔지 알아야됩니다. 따라서 clone 메소드를 구현해야하는 Clone 트레이트는 트레이트 객체로 사용할 수 없는 것입니다. 

Object safety(https://doc.rust-lang.org/reference/items/traits.html#object-safety) 라는 규칙들이 있는데 트레이트 오브젝트를 만들기 위한 규칙들입니다. 여기에 Sized 트레이트를 구현하면 안된다고 써있는데 Clone 트레이트는 상위 트레이트로 Sized 트레이트를 구현하고 있습니다. Sized 트레이트를 상위 트레이트로 가진다는 것은 바로 트레이트에서 사용하는 모든 타입들이 컴파일 시점에 어느 크기를 갖는지 알 수 있다는 제한을 부여하는 것입니다. 컴파일 타임에 Self 타입의 크기를 알아야 메모리 복사에서 안정성을 보장할 수 있기 때문입니다.

참고로 이와같이 어떤 메소드를 구현하고자하는 트레이트가 아니라, 속성만 부여하고자하는 트레이트가 여러가지가 있습니다. 그리고 그런 트레이트들을 Marker trait라고 부릅니다. 그래서 Sized트레이트가 정의된 위치가 std::marker입니다.

https://doc.rust-lang.org/std/marker/trait.Sized.html

러스트는 이와같이 메모리 안정성을 위해서 복잡해보이는 규칙들을 가지고 있습니다. 과하다싶을 때도 있지만, 이런 규칙들을 개발자가 생각하면서 개발해야하는 언어들의 문제점을 해결하는게 이토록 쉽지않은 일이라는 것을 알 수 있습니다. 처음에는 복잡해보이고 의미를 알 수 없는 규칙들이지만, 메모리 안정성을 염두해두고 연습을 해나간다면 이해할 수 있습니다.

지금까지 길게 Clone 트레이트의 구현을 설명했지만, 사실은 이렇게 직접 구현할 필요가 없습니다. derive를 쓰면 자동으로 생성됩니다.

```rust
#[derive(Debug, Clone)]
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

    let another = Book {
        title: String::from("The another book"),
        author: String::from("Unknown"),
        published: 20111111,
    };

    let mut book_clone = book.clone();
    println!("{:?}", book_clone);
    book_clone.clone_from(&another);
    println!("{:?}", book_clone);
}
```

#### Default

Default 트레이트는 구조체의 각 필드를 디폴트값을 초기화해서 객체를 생성해줍니다.

```rust
#[derive(Debug, Clone, Default)]
struct Book {
    title: String,
    author: String,
    published: u32,
}

fn main() {
    let book = Book::default();
    let mut book_clone = book.clone();
    println!("{:?}", book_clone);
}
```

새로운 객체를 만들 때 사용하므로 정적 메소드입니다. 따라서 “타입이름::default()” 형태로 호출합니다. 

물론 러스트 언어에서 기본값으로 지정한 값이 개발자의 의도와 다를 때가 있습니다. 그럴때는 직접 구현하면 됩니다.

#### PartialEq

PartialEq는 두 객체가 같은 값을 가지고 있는지를 확인하는 트레이트입니다.

```rust
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

위와 같이 두 책이 같은 책인지 확인하려면 책 제목과 저자 이름을 확인하게 됩니다.

트레이트 이름이 PartialEq라고해서 왜 Partial이라는 이름이 들어갔는지 의아하게 생각할 수도 있습니다. 하지만 완전하게 동일한 객체를 비교하는게 아니라 위와같이 좀더 넓은 의미에서 일부 같은 값을 가진 객체도 비교할 수 있다는 유연성을 갖는다고 이해하면 쉬울듯합니다. 바로 아래 예제를 생각해보면 됩니다.

```rust
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

Book 타입과 Person 타입은 동일한 객체가 될 수는 없습니다. 하지만 일부 같은 값을 갖는지를 비교할 수는 있겠지요. 아직 제너릭에 대한 이야기를 하지 않았지만, PartialEq에 어떤 타입 바인딩을 쓰는지에 따라서 비교할 대상의 타입을 지정할 수 있다는 것만 생각해보시면 되겠습니다.

PartialEq 외에도 Eq 트레이트가 있습니다. 하지만 보통 우리가 == 연산자로 비교할 때는 PartialEq를 사용한다는 것을 기억하시기 바랍니다.

## Generic 프로그래밍

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

### 트레이트에 제너릭 사용하기

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

### 트레이트와 구조체 모두 제너릭 사용하기

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

## Life-time 라이프타임

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

## 에러 처리 실습

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