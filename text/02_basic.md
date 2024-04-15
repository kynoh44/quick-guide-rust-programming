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

예제 자체는 따로 설명이 필요없을 정도로 간단합니다. 어떤 언어로든 프로그래밍을 해봤다면 쉽게 이해할 수 있습니다. 예제에 사용된 러스트 문법만 간단히 소개하겠습니다.

변수의 정의는 let 키워드를 사용합니다.

```rust
let 변수이름: 타입 = 초기값;
```

함수의 정의를 fn 으로 시작합니다.

```rust
fn 함수이름(인자: 타입, 인자: 타입, ...) -> 반환값 {
	함수 라인1;
	라인2;
	걀과값
}
```

각 라인은 C/C++과 같이 “;”로 끝나야됩니다. 그리고 마지막에 함수 반환값은 “;”없이 쓰게됩니다. Scala등의 함수형 언어와 유사한 점입니다. 반환값이 없으면 안써주거나 반드시 반환값일 지정해야되는 경우 (if-else만 있는 함수같은 경우)에는 ()라고 써주기도 합니다.

한가지 주의해야할 점이 있다면 이터레이터를 사용하는 구문입니다. 러스트에서는 Range라는 타입으로 for문의 범위를 지정합니다만 지금은 크게 신경쓸 필요는 없습니다. 아래와 같이 작성하면 10을 제외한 9까지만 처리하는 코드가 됩니다. Bash나 몇몇 언어에서는 “1..10”이 10을 포함하지만 러스트에서는 10은 포함하지 않습니다.

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

if문에서 마지막 else는 아무것도 하지 않습니다만 else를 꼭 넣어줘야 완벽하게 모든 케이스를 처리하는 코드가 됩니다. else로 따로 할 일이 없다해도 else를 넣고 아무 처리를 하지 않는다는 코멘트라도 넣어줘야 보안에 신경쓴 코드가 됩니다. 러스트는 이런 보안 헛점들을 방지하기 위한 문법들을 가지고 있습니다. 러스트에서는 어떤 문법을 지원하는지 차차 설명하겠습니다.

## if 구문은 값을 가짐

C로 간단한 if-else 구문 사용 방법을 보겠습니다.

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

물론 러스트도 아래와 같이 C코드와 완전히 동일하게 구현할 수 있지만 추천하는 방식은 아닙니다.

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

왜 굳이 if문이 값을 가지도록 만들었을 까요? 프로그래밍 언어의 철학에 대해서 본격적으로 이야기하자면 저도 잘 모르는 깊은 이론들이 있겠지만, 제가 개발하면서 느꼈던 장점은 값이 초기화되지 않은 변수를 최소화할 수 있다는 것입니다. 초기화되지 않은 변수를 일부러 만드는 사람은 없을 것입니다. 하지만 코드가 길어지고 다른 사람의 코드를 유지보수하다보면 실수로 변수의 초기화 코드를 제거하기도 합니다. 그리고 아주 심각한 문제는 초기화가 안된 변수를 알아차리지 못하게 계속 사용하는 경우가 생길 수 있다는 것입니다.

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

그리고 함수를 정의할 때는 타입을 생략할 수 없습니다.

## as를 이용한 타입 바꾸기

사실 C 언어가 가진 큰 단점중에 하나가 타입이 암묵적으로 바뀐다는 것입니다. C++은 C보다는 좀 더 엄격하게 타입이 바뀌는 것을 방지했지만, 그래도 대부분의 현대 언어들에 비하면 많이 느슨한 편입니다.

러스트는 타입 변환을 항상 명확하게 선언해주어야 합니다. 다음 예제 코드는 문자열로 된 숫자를 정수로 반환하는 함수입니다. 

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

함수 선언 부 외에는 변수의 타입을 지정하지 않았지만 c라는 변수가 char라는 타입인지 알 수 있습니다. 러스트에서 for문에는 항상 이터레이터만 사용할 수 있다고 설명했는데, String이라는 구조체의 chars라는 메소드가 문자열의 각 문자들을 반환하는 이터레이터를 생성해주는 메소드라고 생각하면 됩니다.

러스트에서는 char 타입의 변수 c에서 char 타입의 문자 ‘0’를 뺄 수는 없습니다. 현대 언어들에 익숙한 분들에게는 당연할 수도 있는 사항인데요, C 계열 언어에 익숙한 분들에게는 당황스러운 것일 수도 있습니다.

C 언어는 사실 어셈블리로 개발하던 프로젝트의 생산성을 높이기 위해 나온 언어입니다. 따라서 어셈블리가 익숙한 개발자들의 관점에서 디자인된 언어이다보니 모든게 숫자입니다. 참과 거짓도 숫자이고, 오류를 나타내는 NULL이나 에러 값도 숫자이고, 포인터도 숫자, char라는 타입도 숫자입니다. 그러니 더하기 빼기가 안될 수가 없지요. 하지만 이게 논리적으로 좋은 디자인이라는건 의문이 남습니다. 숫자가 아닌 타입을 더하거나 뺀다는건 논리적으로는 잘못된 연산이 되는게 더 옳지 않을까요.

잡설이 좀 길었지만 어쨌든 러스트에서는 타입의 변환을 as라는 키워드로 합니다. 추후에 몇가지 타입 변환과 관련된 키워드를 보겠지만 가장 기본적인 것은 바로 as 입니다. 러스트도 사실은 C 계열 개발자들의 생산성을 높이기 위해 나온 언어입니다. 이렇게 언어 자체에 키워드가 있어서 타입이 변환된다는 것도 논리적으로 옳은건지는 모르겠습니다만 Syntax sugar라고 생각해도 될듯합니다.

참고로 C언어에 atoi 라이브러리 함수가 있는 것처럼 러스트에도 이미 같은 일을 하는 라이브러리 함수가 있습니다.

```rust
fn parse_example(input: &str) -> Result<i32, ParseIntError> {
    input.parse()
}

fn main() {
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

string_to_digit 함수의 문제점은 문자열이 숫자 외의 문자를 가지고있는 등의 에러 상황에서 에러 값을 반환할 수가 없다는 것입니다. 에러 값을 반환하기 위해서 함수 인자에 결과값에 대한 포인터를 전달하거나 하면 코드가 복잡해지게됩니다. C 언어의 디자인 단계에서부터 실수한 것이 바로 에러 값에 대한 처리가 없다는 것입니다.

러스트는 에러 값으로 Result나 Option같은 타입을 사용하게 됩니다. 추후에 좀 더 자세히 알아볼 것인데 미리 이런 상황에서 필요하다는 것을 알고 넘어가면 나중에 좀 더 이해하기 쉬울것 같아서 예제를 만들어보았습니다.

## 패턴 매칭

러스트의 기본 문법들은 대부분 C에서 가저오거나 최신 함수형 언어들에서 가져온 것들이라 개발 경력이 조금 있는 분들은 금방 익숙해질 수 있습니다. 그러다가 처음으로 흥미를 느끼는 문법이 바로 패턴 매칭 부분입니다. 간단하게 생각하면 if-else가 여러개 있는 케이스나 C에서 switch문으로 작성하는 케이스들을 좀더 간단하게 만들어 줬다고 생각할 수 있습니다.

패턴 매칭은 사실 엄격한 정의가 있다기 보다는 쓰다보면서 적응해나가는게 더 효율적인 접근방법이라고 생각합니다. 일단 가장 쉬운 예제를 하나 보겠습니다. 이전에 만든 fizzbuzz 예제를 패턴 매칭으로 바꿔보았습니다.

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
```

위의 예제를 보면 (i % 3, i % 5) 는 하나의 튜플 Tuples를 만든 것입니다. 이 튜플의 값이 각 패턴에 해당될 때 다른 메세지를 출력하도록 만든 것입니다. 언더바 _ 는 모든 값을 의미합니다. 튜플의 값을 비교해야하는데 두개의 값이 있어야 하므로 하나는 0이고 다른 값은 무엇이든 상관없을 때 언더바를 사용한 것입니다. 패턴 매칭이므로 2개의 값이 있는 튜플의 패턴을 매칭하려면 2개의 값이 있어야된다는 것을 주의해야합니다.

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

흔하게 사용하는 패턴을 하나 더 보겠습니다. 어떤 변수의 값이 어느 범위에 속하는지 판단하는 코드입니다.

```rust
let gen = match age {
    0..=20 => "MZ",
    21..=50 => "X",
    51..=100 => "A",
    _ => "?",
};
```

age라는 변수를 0..20이라는 Range타입과 비교하는 것인데, 문법적으로 세세하게 따지면 복잡할 수도 있는 코드입니다만 지금은 이런 것도 가능하다 정도로 생각하고 일단 자주 활용하면서 익숙해지면 될듯 합니다. 단순히 age라는 변수의 범위만 따지는게 아니라 각 경우에 따른 반환 값을 gen이라는 변수를 생성하는데 사용한 것을 주의해서 보시기 바랍니다.

단순히 패턴에 일치하는 것만 확인하는게 아니라 아래와 같이 if와 결합해서 조건식을 써줄 수도 있습니다. 

```rust
for i in 1..=100 {
    let msg = match i {
        n if n % 15 == 0 => format!("{} - FizzBizz", n),
        n if n % 3 == 0 => format!("{} - Fizz", n),
        n if n % 5 == 0 => format!("{} - Buzz", n),
        _ => format!("{}", i),
    };
    println!("{}", msg);
};
```

format!()은 문자열 객체를 반환해주는 매크로 함수입니다.

러스트 관련 소개 자료나 공식 문서등을 보면 러스트의 패턴 매칭은 강력하다라는 설명이 있습니다. 보통 강력하다라는 말은 응용되는 케이스가 많다는 의미입니다. 러스트로 개발하다보면 이런것도 되네 하면서 감탄하는 경험을 하게되실 것입니다.

## 러스트의 표현식

if-else나 패턴 매칭의 예제를 보면 변수를 선언하는 부분에 복잡한 코드를 넣은 것을 볼 수 있습니다.

```rust
let gen = match age {
    0..=20 => "MZ",
    21..=50 => "X",
    51..=100 => "A",
    _ => "?",
};

let var = if num % 3 == 0 {
    3
} else {
    if num % 5 == 0 {
        5
    } else {
        0
    }
};
```

match를 이용한 패턴 매칭 구문에서 { 로 시작하고 } 로 끝나는 하나의 블럭을 표현식Expressions 라고 합니다. 마찬가지로 if-else도 하나의 표현식입니다.

그리고 표현식은 값을 반환합니다.

값을 반환하는건 무엇이 있나요? 함수가 있습니다. 함수도 하나의 표현식입니다.

```rust
fn ret_zero() -> i32 {
    0
}
```

함수도 {와 }로 시작과 끝을 나타내고 반환값을 마지막에 써놓은 표현식입니다.

위 예제의 if-else도 각 {} 블럭 안에 반환값이 정해져있습니다.

그리고 match 구문에서도 각 상황에 따른 반환값이 있습니다.

이 표현식은 ;를 만나면 그 반환값이 무시됩니다. 그래서 아래와 같은 변수 초기화 코드가 만들어질 수 있게됩니다.

```rust
let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // This expression will be assigned to `y`
        x_cube + x_squared + x
    };
```

y라는 변수의 값을 계산하기 위해 복잡한 코드가 나열되는게 아니라 y의 선언부에 모여있을 수 있습니다. 물론 위에서 본것과 같이 match, if-else등 도 올 수 있습니다. 다른 언어와 마찬가지로 당연히 함수 호출도 올 수 있겠지요.

참고로 원어로 표현식은 Expression이고 표현식에 ;가 붙은 것을 Statement라고 구분해서 부르는 경우도 있습니다.

## 배열과 슬라이스

문자열을 사용하는 예제를 봤으니 우선 배열과 슬라이스에 대해서 알아본 후 본격적으로 문자열(사실은 문자열이라기 보다는 String 객체가 맞는 표현입니다만)을 알아보겠습니다.

배열은 간단합니다. 같은 타입의 데이터가 메모리에 연속적으로 나열된 데이터 구조를 말합니다.

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

그래서 배열의 인덱스 [0], [1]을 가지고 빠르게 각 데이터에 접근할 수 있습니다.

슬라이스는 이런 배열의 일부(혹은 전체)만을 접근하려고 만든 레퍼런스 타입입니다.

```rust
let slice: &[i32] = &numbers[1..4]; // Create a slice from index 1 to 3 (inclusive)
```

위의 슬라이스는 numbers라는 배열의 1,2,3번 데이터에만 접근할 수 있습니다. [i32]는 배열이고 참조 연산자 &를 써서 배열의 참조라는 것을 나타냅니다. 

여기서 배열이나 구조체같은 여러 데이터가 같이 묶여있는 타입의 디버깅을 위해 좋은 방법을 소개하겠습니다.

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

나중에 배울 구조체도 같은 방법으로 구조체의 각 데이터 필드를 출력할 수 있습니다.

어쨌든 슬라이스를 통해 3개의 데이터만 접근할 수 있는 것을 알 수 있습니다. 배열의 참조이지만, 슬라이스라는 타입 내부에는 이 슬라이스가 몇개의 데이터를 참조할 수 있는지에 대한 정보가 같이 저장되어있다고 보시면 됩니다. 그래서 슬라이스의 데이터를 전부 출력해보면 3개만 출력되는 것입니다.

슬라이스가 뭔지는 알았는데 그럼 슬라이스가 왜 필요한지가 의문일 수 있습니다. 왜냐면 C언어에는 슬라이스가 없기 때문입니다. 배열이나 구조체가 존재하는 메모리를 읽다가 그 다음 메모리까지 읽는 것을 방지할 수 없는게  C언어입니다. numbers[5]를 읽거나 쓰려는 실수를 누구나 해본적이 있을 것입니다.

슬라이스는 바로 이런 실수를 방지하려고 있는 것입니다. 함수나 쓰레드를 호출 할 때, 그 함수나 쓰레드가 배열의 일부분에만 접근해야한다고 할 때, 배열 전체를 전달하는게 아니라 슬라이스를 전달하게됩니다.

위에서 슬라이스는 배열의 일부 혹은 전체에 접근하기 위한 타입이라고 설명했습니다. 다음 예제를 보겠습니다.

```rust
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
   let numbers: [i32; 5] = [1,2,3,4,5];
   let slice = &numbers[1..4];

   println!("{}", sum_array_ref(&numbers));
   println!("{}", sum_slice(slice));
}
``` 

sum_array_ref는 배열을 참조로 전달받는 함수이고,  sum_slice는 슬라이스를 전달받는 함수입니다. 당연히 슬라이스가 배열의 참조이므로 형태가 동일합니다. 각 함수는 자신이 전달받은 데이터가 배열의 참조인지, 배열의 일부분을 가르키는 슬라이스인지 알 수가 없습니다. 사실 구분할 필요도 없는 것입니다.

배열이나 문자열을 처리하는 함수를 만들 때는 항상 슬라이스로 인자를 받도록 만드는 습관을 가지면 좋습니다.

물론 슬라이스같은 참조가 아니라 배열이나 문자열을 그대로 전달하면 소유권까지 함수로 넘어가게되서 함수를 호출한 코드에서 다시는 배열이나 문자열을 접근할 수 없게되기 때문에, 그렇게 슬라이스로 처리를 할 수 밖에 없습니다. 러스트의 소유권 개념은 너무나 유명해서 아마 한두번은 들어보셨을 것입니다. 어쨌든 러스트는 이렇게 사용자가 실수할 수 있는 부분들을 최대한 막으려는 디자인 철학을 가진 언어입니다. C의 문제점들을 해결하려고 C++에서 수십차례 버전을 올려가며 규약들을 만들고, 스마트 포인터를 만드는 등의 노력을 했었지만, 근본적으로 언어의 철학 자체가 개발자가 모든 것을 직접 처리할 수 있어야 한다는 철학을 밑바탕에 가지고 있기 때문에 완전히 막을 수 없는 헛점들이 있었습니다. 러스트는 그런 C++의 최신 기법들을 모두 모아놓고, 강제로 쓰도록 정해놓았다고 이해를 해되 된다고 생각합니다.

## String

C같은 언어에서는 문자열은 char 타입의 배열입니다. 하지만 러스트를 비롯한 고급언어들은 문자열은 String이라는 구조체나 새로운 타입으로 표현합니다.

그럼에도 러스트에서 슬라이스와 함께 문자열을 설명하는 경우가 많은데 그 이유를 알아보겠습니다.

우선 String(https://doc.rust-lang.org/std/string/struct.String.html)이라는 타입에 대해서 알아보겠습니다.

메뉴얼을 자세히 볼 필요는 없지만 첫줄만 봐도 결국 하나의 구조체라는 것을 알 수 있습니다. C에서와같이 문자의 배열은 아닙니다. 따라서 String을 사용하기 위해서는 우선 구조체에 속한 메소드를 호출해서 객체를 생성해야합니다. 다음 짧은 예제에는 몇가지 흔하게 사용되는 String 생성 방법들을 모아봤습니다. 참고로 String은 러스트의 “The Rust Standard  Library”에 포함되기 때문에 C와 같은 include등의 추가적인 라이브러리 참조 코드를 쓸 필요없이 바로 사용할 수 있습니다.

```rust
fn main() {
    let hello = String::from("Hello, world!");
    let mut s = String::new();
    let s = "initial contents".to_string();
    let hello = String::from("안녕하세요");
}
```

가장 먼저 익숙해져야할 것은 “이렇게 큰따옴표안에있는 문자열”, 즉 리터럴(Literal)은 String이 아니라는 것입니다. 리터럴은 컴파일 시점에 고정된 크기의 데이터입니다. 소스 코드에 하드 코딩된 문자열이니까요. 리터럴은 소스 코드에 하드 코딩되었으므로 프로그램의 실행중에 데이터가 변하거나 크기가 바뀌는 일이 없습니다. 따라서 컴파일러는 메모리를 할당해서 리터럴을 저장하는게 아니라 프로그램 코드가 저장되는 영역 (프로그램 바이너리 파일이나 프로세스가 실행된 뒤에는 프로그램의 코드가 저장된 메모리 영역)에 저장합니다. 리터럴은 리터럴이고, String이라는 타입(구조체)의 객체는 다른 것입니다. String타입의 객체는 프로세스가 실행 중에 프로세스의 힙 영역에 메모리를 할당하고, 할당된 메모리에 리터럴 데이터를 복사해서 생성합니다. 그런 메모리 할당과 복사를 from이라는 정적 메소드나 to_string이라는 메소드가 실행하는 것입니다.

물론 러스트는 최신 언어답게 UTF-8을 지원하고있어서 어떤 언어든지 String객체를 만들 수 있습니다.

각 메소드들을 설명하자면 다음과 같습니다.

- from(”msg”): “와 “안에 들어가는 문자열을 String 객체로 생성함
- new(): 아무런 데이터가 없는 String 객체 생성
- “msg”.to_string(): “와 “안에 있는 문자열을 String객체로 생성함, from과 같음

그리고 또 하나 흔하게 String을 만드는 방법이 format! 매크로 함수를 이용한 방식입니다.

```rust
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

let s = format!("{}-{}-{}", s1, s2, s3);
```

String이 객체이니까 당연히 많은 메소드들이 있을 것입니다. 자바등 고급 언어를 다뤄본 분이라면 익숙한 메소드들이 많을 것입니다. push_str나 insert, len 등등 대부분의 언어들과 마찬가지로 String객체에 추가로 문자열을 넣거나 길이를 반환하거나 하는 메소드들이 있습니다.

그럼 “이렇게 큰따옴표 안에 들어간 문자열”은 무엇이고, String과의 관계는 어떻게 되는 걸까요?

String 구조체의 정의부터 찾아보겠습니다.

```rust
pub struct String {
    vec: Vec<u8>,
}
```

출처: https://doc.rust-lang.org/src/alloc/string.rs.html#365

고급 언어를 다뤄봤다면 반드시 사용해봤을만한 벡터가 나타났습니다. 사실 String은 그냥 8비트 값들의 벡텨였던 것입니다. UTF-8 코딩의 정의를 생각해본다면 당연한 것일지도 모르겠습니다.

그럼 String의 슬라이스는 무엇일까요? 결론부터 말하면 &str 타입이 String의 슬라이스입니다.

```rust
let greeting: String = String::from("Hello, world");
let gretting_slice: &String = &greeting[1..3];
```

그런데 이 예제 코드는 컴파일되지 않습니다. 아래와 같은 에러 메제시가 나타납니다.

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

결국 우리는 String객체의 참조가 &String이 아닌 &str인것을 알 수 있습니다.

String은 흔히 말하는 Fat pointer입니다. 실제로 문자열이 저장된 메모리의 주소도 들어있지만, 데이터의 현재 길이나 버퍼의 크기 등 추가 정보들이 들어있습니다. 슬라이스는 이전 배열에서도 봤지만 코드가 실행되는 시점에서 순수 데이터의 일부분이나 데이터 전체를 보기 위한 참조입니다. String객체를 C에서 하나의 구조체라고 생각하고, 슬라이스를 구조체의 필드 중에 하나인 char*라고 생각하면 그 관계에 대해 이해하기 쉬울지도 모르겠습니다.

```rust
struct String {
    int buffer_len;
    int data_len;
    char *buffer;
};
```

그럼 한가지 의문이 드는게 &str이 아니라 str으로 참조 연산자를 안쓰고 사용할 수는 없는가 입니다. 사용할 수 없습니다. 왜냐면 러스트는 메모리 관리를 위해 컴파일 시점에서 크기가 정해진 객체만을 허용하기 때문입니다.

str은 벡터에 들어있는 UTF-8데이터 그 자체, 그 메모리 덩어리를 가지는 변수가 되어야 합니다. 하지만 문자열의 길이는 정해진 것이 아닙니다. 일반적인 구조체는 각 필드의 데이터 타입이 고정되어있으니 컴파일 시점에 크기를 알 수 있습니다. 배열은 배열을 선언할 때 데이터가 몇개인지 지정하기 때문에 크기를 알 수 있습니다. 하지만 벡터가 가진 메모리 덩어리의 크기는 String객체를 어떤 문자열로 만들느냐에 따라서 변할 수가 있는 것입니다. 결국 str라는 타입은 사용할 수가 없습니다.

하지만 참조연산자 &를 붙이면 이야기가 달라집니다. 왜냐면 &str는 포인터이고 포인터의 크기는 컴파일 시점에 고정되어있기 때문입니다. 최신 씨피유를 쓴다면 64비트이니까요. 따라서 아래와 같이 선언하는 것은 컴파일러가 봤을 때 64비트의 변수를 스택 메모리에 만들면 되는 것이니, 아무런 문제가 없습니다.

```rust
let s: &str = "hello";
```

&str는 순수 문자열에 대한 참조입니다. 따라서 “hello”와 같이 문자열만 쓴 것으로 생성할 수 있습니다. 하지만 이런 문자열에 대한 참조가 String은 아닙니다. String을 만들려면 벡터를 만들어야되고, 벡터를 만들려면 힙 메모리에 메모리를 할당해야하고, 그 외에 문자열을 관리하기 위한 추가 데이터들을 할당해야합니다. 따라서 슬라이스 “hello”를 가지고 String을 만들기 위한 추가 작업들 format!의 호출이나 to_string메소드 호출 등을 해야하는 것입니다.

사실 왜를 따지다보면 처음 러스트를 접하는 입장에서는 혼란만 생깁니다. 왜 메모리 크기가 컴파일 시점에 고정되어야하는지, 그럼 동적으로 할당되는 메모리는 사용할 수 없는 것인지 의문이 더 생깁니다. 현재 이 순간에는 String에 대한 참조나 슬라이스가 &str라는 것만 생각하고 넘어가는 것도 방법입니다. 아니면 러스트의 String 구조체에 대한 매뉴얼이나 The Rust Programming Language등의 추가 자료를 찾아보는 것도 좋은 공부방법이라고 생각합니다.

마지막으로 String 객체를 다른 함수에 전달할 때 슬라이스를 써야한다는 것을 이야기하겠습니다. 아직 소유권에 대해서 배우지는 않았지만 간단하게 말하면 String 을 그대로 다른 함수에 전달하면, 그 함수를 호출한 이후에는 그 객체를 다시 사용할 수 없습니다. 객체를 그대로 전달한다는 것은 소유권까지 넘겼다는 것이기 때문입니다. 그 와 반대로 String객체의 슬라이스를 넘기는 것은 객체에 있는 문자열 데이터의 참조권을 잠시 빌려주는 것으로 생각하면 됩니다. 함수가 끝나더라도 객체의 소유권은 함수를 호출한 코드에 남아있기 때문에 계속 객체를 사용할 수 있습니다.

아래 코드를 빌드해보겠습니다.

```rust
fn moved_string(data: String) {
    println!("{}", data);
}

fn main() {
    let mut mutable_string = String::from("hello");

    moved_string(mutable_string);
    println!("{}", mutable_string);
}
```

이런 에러 메세지를 볼 수 있습니다.

> 161 | fn moved_string(data: String) {
|    ------------       ^^^^^^ this parameter takes ownership of the value
|    |
|    in this function
> 

객체를 그대로 함수에 전달했기 때문에 함수의 파라미터에 객체의 소유권까지 옮겨졌다는 뜻입니다.

어떤 객체를 함수에 전달할 때는 보통 참조를 전달해야하고, 그러므로 String을 함수에 전달할 때는 &str을 전달해야한다는 것을 기억합시다.

```rust
fn moved_string(data: &str) {
    println!("{}", data);
}

fn main() {
    let mut mutable_string = String::from("hello");

    moved_string(&mutable_string);
    println!("{}", mutable_string);
}
```

### String을 배열처럼 참조할 수 없는 이유

아래와 같이 String객체에서 첫번째 글자를 출력할 수 있을까요?

```rust
let mut mutable_string = String::from("hello");
println!("{}", mutable_string[0]);
```

할 수 없습니다. 아래와 같은 에러 메세지를 얻으실겁니다.

> error[E0277]: the type `String` cannot be indexed by `{integer}`
--> src/main.rs:167:20
|
167 |     println!("{}", mutable_string[0]);
|                    ^^^^^^^^^^^^^^^^^ `String` cannot be indexed by `{integer}`
> 

일단 답부터 이야기하면 아래와 같이 chars메소드를 호출해서 이터레이터를 만든 후, nth메소드로 특정 인덱스의 문자를 얻을 수 있습니다.

```rust
let mut mutable_string = String::from("hello");
println!("{}", mutable_string.chars().nth(0).unwrap());
```

nth메소드는 Option이라는 Enums를 반환하므로 이 Enums에서 최종 문자를 얻어내기 위해 unwrap이라는 메소드를 호출한 것입니다.

일단 Option이라는 Enums은 추후에 알아보기로 하고, 왜 인덱스를 이용한 직접 접근이 안되게 막아놨을까요? 연산자 오버라이딩 등의 방법이 있었을텐데요.

그 이유는 UTF-8을 완벽하게 지원하기 위해서입니다. 언어를 디자인할 때 인덱스 참조를 지원해서 [0]이 0번째 바이트를 반환하도록 만들었을 수도 있습니다. 하지만 이러면 ascii에 대한 지원은 잘 될지 몰라도 UTF-8을 제대로 지원하는 언어가 될 수는 없습니다. 첫번째 글자를 반환할 수도 있었겠지만, 첫번째 글자 하나만 놓고봤을 때 이 첫글자가 1바이트가 될지 2바이트가 될지 알 수가 없습니다. 이런 여러가지 문제들이 있기 때문에, 항상 이터레이터를 호출하도록 만들고, 이터레이터가 문자열의 전체 데이터를 분석한 후에 한 문자씩 반환하도록 만들었습니다. 그런 이유로 String의 이터레이터 메소드 chars의 처리 속도가 느린 것입니다.

만약 바이트 단위로 쪼개고싶다면 as_bytes라는 메소드를 호출하면 됩니다. 문자열 데이터가 반드시 ascii 문자열이라는 상황이라면 괸찬은 옵션입니다.

## 러스트의 소유권(Ownership) 개념

배열에서의 슬라이스나 String과 &str의 관계를 보면서 소유권을 넘기지 않기 위해 참조를 사용한다는 이야기를 수차례 했습니다. 슬라이스도 그렇지만 그 외에 러스트의 문법적인 특징들의 상당수가 소유권 개념을 구현하기 위해서 만들어진 것들이라고 해도 과언이 아닙니다. 왜 이런 문법을 정했을까? 왜 이건 이렇게 복잡하지? 등등 러스트를 공부하면서 겪게되는 의문들과 진입장벽들의 상당수가 소유권과 연관이 있을 것입니다. 그리고 러스트가 가진 장점 중에 가장 큰 장점이라고 이야기하는 메모리 안전성이 바로 소유권 개념으로 인해 가능한 것입니다.

소유권이 뭔지 그래서 러스트가 데이터를 메모리에 어떻게 배치하고 관리하는지를 이야기해보겠습니다.

### 소유권의 의미

소유권은 단어 그대로 생각하면 메모리를 마음대로 할 수 있는 권한 즉 데이터나 변수를 할당하고 읽고 쓰고 해지해할 수 있는 권리일 것입니다. 함수의 인자로 전달받은 메모리에 대한 소유권도 있을 수 있으니 여러 함수 혹은 여러 쓰레드에서 공유되는 데이터나 메모리에 대한 권한을 생각해야합니다.

가비지 콜렉터가 있는 자바 등의 언어는 메모리를 해지할 수 있는 권한이 프로그램 코드가 아닌 가비지 콜렉터에게 있습니다. 프로그램은 메모리를 할당받아서 읽고 쓸 수 있지만 해지하지는 않습니다. 그냥 더 이상 접근하지 않고 있으면 가비지 콜렉터가 알아서 메모리를 해지해줍니다.

러스트는 컴파일러가 프로그램 코드를 컴파일 할 때 모든 메모리의 소유권을 추적합니다. 러스트가 정한 규칙에 어긋나게 메모리에 접근하는 코드가 있으면 친절한 안내 메세지를 출력하고 더 이상 컴파일을 하지 않습니다. 그래서 러스트 코드의 컴파일 시간이 오래걸린다는 불평이 많습니다. 수십 ~ 수백줄의 간단한 코드도 몇 초정도 시간이 걸리는걸 보면서 좀 답답할 때도 있긴합니다. 하지만 그정도의 간단한 코드를 만드는데 여러번 빌드해야할 정도로 컴파일 에러를 많이 만들고 있다는 것은 사용자에게 문제가 더 있는게 아닌가 생각됩니다. 그리고 빌드를 여러번 할 필요도 없는게 VSCODE등 대부분의 개발툴에서 러스트 언어를 동적으로 분석해주고, 코드를 쓸 때마다 에러 체크를 해줍니다. 빌드하기전에 미리 모든 컴파일 에러를 고칠 수 있습니다.

VSCODE를 예를 들면 Inlay hints https://code.visualstudio.com/docs/languages/rust#_inlay-hints 나 Linting https://code.visualstudio.com/docs/languages/rust#_linting 등의 기능이 있어서, cargo를 호출하기전에 미리 거의 모든 컴파일 에러를 잡을 수 있습니다.

또한 러스트 언어는 한번 빌드가 되고나면 좀처럼 메모리 관련 에러는 발생하지 않습니다. 기타 로우레벨 언어들이 빌드되서 실행이 되더라도 오랜 시간동안 에러가 없는지 검증해야되고, 정적 분석 툴 등을 돌려야 되는 시간들을 생각해보면 전체적인 개발 시간은 확실히 줄어드는 것이라 생각합니다.

The Rust Programming Language에서는 소유권이라는 것이 3가지 규칙을 의미한다고 설명합니다.

- Each value in Rust has an *owner*.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.

(https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)

제가 번역과 함께 좀더 설명을 붙이면 이렇습니다.

- 하나의 변수는 하나의 스코프에 소유됩니다. 당연히 하나의 스코프는 여러 변수를 소유할 수 있습니다.
- 하나의 변수의 소유권을 여러 스코프가 동시에 가질 수 없습니다. 하지만 대여할 수는 있습니다.
- 소유권을 가진 스코프가 끝나면 변수는 해지됩니다.

러스트 언어에서 변수의 스코프는 {로 시작하고 }로 끝나는 구역을 의미합니다.

함수가 대표적인 하나의 스코프입니다.

```rust
fn main() {
    let hello_string = String::from("hello");
    println!("{}", hello_string);
}
```

main함수 시작 부분에서 생성된  hello_string이라는 변수는 main함수가 끝나는 }를 만나면서 해지됩니다.

간단하게 함수 내부에 스코프를 하나 더 만들어서 실험해보겠습니다.

```rust
fn main() {
    let hello_string = String::from("hello");
    {
        let world_string = String::from("world");
        println!("{}", hello_string);
        println!("{}", world_string);
    }
}
```

```rust
$ cargo run
hello
world
```

hello_string이라는 변수의 스코프는 main함수가 끝날 때까지 입니다. 따라서 그 이전에는 어디에서도 사용될 수 있습니다. world_string이라는 변수의 소유권은 내부 블럭에 있습니다. 따라서 아래와같이 내부 블럭의 밖에서는 사용할 수가 없습니다.

```rust
fn main() {
    let hello_string = String::from("hello");
    {
        let world_string = String::from("world");
        println!("{}", hello_string);
    }
    println!("{}", world_string);
}
```

```rust
$ cargo build
error[E0425]: cannot find value `world_string` in this scope
   --> src/main.rs:7:20
    |
  7 |     println!("{}", world_string);
    |                    ^^^^^^^^^^^^ help: a local variable with a similar name exists: `hello_string`
```

world_string이 사용된 스코프는 world_string을 소유하지 않았다는 에러 메세지를 확인할 수 있습니다.

그럼 다음과 같이 같은 이름의 변수가 중첩된 스코프에 존재할 때는 어떨까요?

```rust
fn main() {
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
$ cargo run
world
hello
```

2개의 변수가 동일한 이름으로 생성되지만 “hello”라는 데이터를 가진 변수는 main 함수가 끝나는 바깥 스코프에가 소유권을 가지고있고, “world”라는 데이터를 가진 변수는 main함수 안에서 새로 생성된 작은 스코프가 소유권을 가지고 있는 것입니다. 작은 스코프가 끝날 때 “world”라는 데이터를 가진 변수(혹은 객체)는 해지됩니다.

참고로 스코프가 끝날 때 자신이 소유한 변수들의 drop 메소드를 호출합니다. 다음은 MyStruct라는 아무런 데이터를 가지지않는 구조체를 선언하고, drop 메소드를 구현해준 코드입니다. (아직 구조체에 대한 문법을 알아보지않았지만, 구조체의 선언만 보면 C언어와 거의 동일합니다. 구조체의 메소드를 정의하는 문법은 아직 모르지만, 일단 drop이라는 메소드가 호출되는 시점만 생각해보겠습니다.)

```rust
struct MyStruct {}

impl Drop for MyStruct {
    fn drop(&mut self) {
        println!("Dropping MyStruct now!");
    }
}

fn main() {
    println!("main starts");
    {
        println!("inner-scope starts");
        let my: MyStruct = MyStruct{};
        println!("inner-scope ends");
    }
    println!("main ends");
}
```

```rust
main starts
inner-scope starts
inner-scope ends
Dropping MyStruct now!
main ends
```

drop메소드가 호출되는 지점이 곧 변수의 메모리가 해지되는 지점인데, “inner-scope ends”라는 메세지 후에 drop메소드가 호출되는 것을 볼 수 있습니다. 즉 스코프 안의 모든 코드가 끝나고 스코프가 없어지는 최후의 순간에 스코프가 소유한 변수들을 해지하는 것을 확인할 수 있습니다.

### 소유권의 이동

사실 개념 설명만 들으면 약간 그래서 어쩌라는 건가 라는 생각이 들 수 밖에 없습니다. 몇가지 제가 자주 겪어본 케이스 몇가지를 소개하겠습니다. 이정도만 일단 알고 시작하면 작은 프로젝트를 시작하는데는 문제가 없을거라 생각합니다.

#### 변수 할당에서 소유권이 이동하는 경우

가장 간단한 예는 변수간에 할당이 발생할 때 소유권이 이동하는 경우입니다.

```rust
let s1 = String::from("foo");
println!("{}", s1);
let s2 = s1;
println!("{} {}", s1, s2);
```

이 예제에서 러스트는 s1을 s2로 이동시킵니다. 보통의 언어들에서는 객체의 복사가 일어나거나 포인터의 복사가 일어날 것입니다. 러스트에서는 내부적으로는 포인터의 복사만 일어나고, 거기에 더해서 소유권의 이동까지 일어납니다. 객체 데이터를 복사하지 않기 때문에 속도는 빠르면서 소유권의 이동까지 일어나므로 데이터가 의도하지 않게 공유되는 것을 방지합니다.

그런데 실제로 뭔가를 만드는 경우에 이렇게 예제와 같이 변수에 변수를 옮기는 경우는 거의 없습니다. 실제로 많이 겪는 경우는 변수의 이동이 일어나는지 잘 안보이는 경우들이 대부분입니다.

```rust
let mut user_input = String::from("아이유");
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

이와같이 소유권이 이동이 안보이는 경우가 많긴합니다만, 변수의 이동은 컴파일러가 너무나 친절하게 어디에서 이동이 발생했고, 소유권이 없는 변수를 어디에서 접근해서 에러가 발생했는지를 다 알려줍니다. 그래서 에러를 찾기도 쉽고 고치기도 어렵지 않습니다.

#### 함수 인자로 전달되고 반환값을 받을 때 소유권이 이동하는 경우

```rust
fn make_greeting(name: String) -> String {
    let greeting = format!("{}씨 안녕하세요", name);
    greeting
}

fn main() {
    let user = "아이유".to_string();
    let greeting = make_greeting(user);
    println!("{}", greeting);
}
```

이번에도 크게 어렵지 않은 경우입니다. user라는 변수가 make_greeting의 name이라는 인자에 바인딩되었습니다. 따라서 이전에 본 변수의 할당과 같은 경우입니다. make_greeting 함수가 끝난 뒤에는 user 변수는 사용할 수 없습니다. greeting이라는 변수는 make_greeting이라는 함수에서 생성되었지만 main 함수로 이동된 경우입니다.

```rust
fn make_greeting(name: String) -> String {
    let greeting = format!("{}씨 안녕하세요", name);
    greeting
}

fn main() {
    let mut user = "아이유".to_string();
    user = make_greeting(user);
    println!("{}", user);
}
```

좀 웃기긴 하지만 위 예제는 user 변수의 소유권을 main에서 make_greeting으로 이동한 후, 다시 main으로 가져오는 예제입니다. 그냥 이런것도 가능하다는 것을 보여드린 예제입니다.

실질적으로 이렇게 구현하는 경우는 드물겠지요. 함수를 호출할 때는 보통 객체의 레퍼런스를 전달해서 소유권을 넘기지 않습니다.

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

러스트는 이렇게 레퍼런스를 생성하는 것을 빌렸다 Borrowing 이라고 표현합니다. 소유권의 이동없이 다른 스코프에서 사용하도록 해주는 것이니 적절한 표현이라고 생각합니다.

위 예제에서는 단순히 읽기만 가능한 Immutable reference를 사용했는데요, 쓰기도 가능한 Mutable reference 도 있습니다.

```rust
fn make_greeting(name: &mut String) {
    name.push_str("씨 안녕하세요");
}

fn main() {
    let mut user = "아이유".to_string();
    make_greeting(&mut user);
    println!("{}", user);
}
```

mut 키워드를 호출하는 부분에도 넣고, 함수 인자에도 넣어야 된다는 것을 기억해야합니다.

아래와 같이 Immutable한 변수의 Mutable reference를 만드는 것은 불가능합니다.

```rust
fn main() {
    let user = "아이유".to_string();
    make_greeting(&mut user);
    println!("{}", user);
}
```

소유자가 바꾸고 싶지 않은 변수를 빌린쪽에서 맘대로 바꾸는 것은 당연히 허용하면 안되겠지요.

레퍼런스에 대한 규칙을 요약하자면 다음과 같습니다.

- Mutable reference는 하나만 존재할 수 있다.
- Immutable reference는 여러개 존재할 수 있다.
- 레퍼런스는 언제나 실제 데이터를 참조해야한다.

상식적으로 생각해도 이해가 되는 규칙입니다. 데이터를 바꿀 수 없는 Immutable reference가 여러개 있다고해도 데이터는 변하지 않으니까 상관없습니다. 데이터를 바꿀 수 있는 Mutable reference가 있다면 이 데이터는 언제든지 바뀔 수 있으므로 다른 어떠한 형태의 레퍼런스도 존재하면 안됩니다.

#### 이터레이터와 소유권

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

```rust
My
Bloody
Valentine
["My", "Bloody", "Valentine"]
```

아무 이상없이 실행됩니다. 무슨 차이일까요?

into_iter를 사용했을 때의 에러 메세지를 보면 이런 말이 있습니다.

> note: `into_iter` takes ownership of the receiver `self`, which moves `user`
> 

값으로 이터레이터를 만든다는 의미는 변수의 값을 이동시킨다는 의미입니다. 즉 소유권을 가져간다는 뜻입니다. for 루프에서 c변수의 타입이 String이 되고, c변수가 루프를 돌때마다 배열에 있는 객체를 하나씩 소유하게 됩니다. 그리고 루프가 끝날 때마다 스코프가 끝나고, 객체가 해지됩니다. 모든 루프가 끝나면 배열 전체가 다 해지됩니다.

반대로 iter()는 슬라이스를 만든 후 슬라이스의 이터레이터를 만듭니다. 슬라이스는 배열이나 벡터, 문자열등에 접근하기 위한 레퍼런스입니다. 따라서 소유권을 가져갈 수 없습니다. 결론적으로 소유권 이동없이 배열의 각 항목에 레퍼런스로 접근합니다. c 변수의 타입은 &str이 됩니다.

이터레이터에 대한 팁을 한가지 드리자면 iter와 into_iter가 각각 사용하는 경우가 다릅니다.

- iter: 루프 후에도 계속 데이터를 접근할 경우, 대부분 iter를 더 자주 사용함
- into_iter: 데이터 구조의 포인터를 모아놓은 벡터를 해지할 때 사용

데이터 구조의 포인터들을 모아놓은 벡터를 해지할 때, 루프를 돌면서 각 데이터 구조를 해지하고, 마지막에 벡터를 해지하는 경우가 흔할 것입니다. 이럴 때 into_iter를 사용하면, 각각의 항목을 따로 해지하지않아도 각 루프의 스코프가 끝나면서 자동으로 해지됩니다.

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

이렇게 바꾸면 컴파일 에러 없이 잘 동작합니다. clone은 말 그대로 데이터를 똑같이 복사해서 클론을 만드는 것입니다. Deep copy를 한다고 생각할 수도 있습니다.

위에서는 루프를 돌기전에 user의 이름없는 복사본을 만들고 그 복사본의 into_iter 메소드를 호출해서 인터레이터를 만듭니다. 그래서 user 객체는 그대로 존재하고, 복사본만 해지됩니다.

### 변수가 스택 영역과 힙 영역 중 어디에 할당되느냐에 따라 소유권 

크게 중요한 내용은 아니지만 러스트의 내부를 이해하기 위한 설명을 약간 해보겠습니다.

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

t = a+b라는 코드가 있는데요 t변수는 a와 b중 어느 변수의 소유권을 가지게되는 것일까요? 

사실 정수 타입의 변수는 소유권 이동이 일어나지 않습니다. 정수나 부동 소수점 타입등과 Boolean타입은 소유권의 이동도 일어나지않습니다. 함수 호출에 인자로 전달되면 값이 복사됩니다. 또 위 코드와 같이 대입이 될때도 값이 복사됩니다.

그럼 소유권 이동이 일어나는 타입과 그런 규칙에서 예외되는 타입의 구분은 무엇일까요? 변수의 할당에 대해서 직접 신경써야되는 로우레벨 언어에 경험이 있으신 분들은 금방 눈치챘을 것입니다. 바로 스택에 할당되는 변수이나 힙에 할당되는 영역이냐의 차이입니다.

일반적인 숫자 (정수와 부동 소수점)과 참/거짓의 Boolean 타입은 메모리 크기가 정해져있습니다. i32이면 4바이트이고 u8이면 1바이트입니다. 이렇게 컴파일 시점에 이미 메모리 크기가 정해진 변수는 스택에 할당됩니다. 세부적인 것 까지 이야기할 수는 없지만 스택 메모리에 할당하는 것이 빠르고 관리가 쉽기 때문입니다. 스택에 저장된 변수들은 함수가 종료될 때 스택 영역 전체를 해지하면서 한꺼번에 해지됩니다. 따라서 메모리 누수에 대한 염려도 없고 메모리 크기가 작으므로 복사하는데도 시간이 오래 걸리지 않습니다. 따라서 굳이 소유권을 설정하지 않아도 되는 것입니다.

그와 다른게 String과 같은 구조체 타입을 들 수 있습니다. 구조체 타입과 크기에 따라 메모리를 할당해서 객체를 만드는 것은 힙 영역의 메모리를 사용합니다. malloc같은 메모리 할당 함수를 내부적으로 호출해서 메모리 영역을 할당하는 것입니다. 왜냐면 컴파일 시점에 String 객체의 어떤 데이터를 넣을지 모르기 때문입니다. 리터럴로 String 객체를 만들 때는 데이터 크기를 알 수 있겠지만, 사용자 입력을 받아서 String 객체를 만들거나 네트워크에서 받은 데이터로 객체를 만들 때는 프로그램이 실행 중일 때만 데이터의 크기를 알 수 있습니다.

```rust
fn main() {
    let s = String::new();
}
```

위와 같이 s라는 변수를 만들었습니다. 이 s는 스택에 생성된 포인터 변수입니다. 64비트 CPU를 가진 시스템에서 동작한다면 스택에 8바이트 메모리 영역을 할당하고, 힙 영역에 String객체를 생성한 후 스택 메모리 영역에 힙 영역의 주소를 저장한 것입니다. 우리가 s라는 변수를 통해 객체에 저장된 데이터를 읽으면

1. s라는 변수에서 힙 영역의 주소 값을 읽음
2. 힙 영역에서 데이터를 읽음

이와 같이 2번의 메모리 접근이 일어납니다. 따라서 아래와 같이 변수 대입이나 함수 호출을 통해 소유권을 전달한다는 것은 물리적으로 따지면 포인터 값 (64비트 정수 값)을 복사하는 것 뿐입니다. 컴파일러가 변수 대입이나 함수 호출 등 소유권 규칙에 따른 동작이 일어날 때마다 소유권의 이동을 감시하고 규칙에 부합하는지를 따지는 것 뿐입니다. 그러므로 컴파일러가 많은 일을 하지만, 최종 생성되는 프로그램 코드가 늘어나거나 하지 않고, 결과적으로 메모리 관리가 안정적인 프로그램을 만들 수 있는 것입니다.

정리를 하자면 러스트에서 원시 타입 Primitive type으로 분류된 타입들은 이동이 아니라 복사가 일어나입니다. 어떤 타입들이 원시 타입인지는 Rust의 Standard Library 메뉴얼을 참고하시기 바랍니다.

https://doc.rust-lang.org/std/#primitives

C나 예전 C++을 사용해본 개발자라면 이렇게 생각하면 쉽습니다.

> malloc/new 등으로 할당하고 free로 해지해줘야되는 메모리나 객체를 자동으로 해지해주는 대신 소유권을 관리해줘야 한다. Primitive type은 복사가 일어나고 그 외는 이동이 발생한다.
> 

모던 C++을 아는 개발자는 이렇게 생각하면 더 이해하기 쉬울 것입니다.

> RAII가 권장이 아니라 강제 사항이다. 모든 포인터는 스마트 포인터이다.
> 

나중에 Copy trait라는게 나오는데, 미리 간단하게 말씀드리면 데이터 타입의 크기를 컴파일러가 알기 때문에 데이터의 이동이 아니라 복사를 해주는 데이터 타입들의 속성이라고 생각하면 됩니다. 컴파일러가 크기를 안다는 것은 Primitive type은 기본적으로 Copy trait를 구현하고 있다는 말입니다. 그 외의 타입들은 동적으로 크기가 바뀔 수도 있으므로 컴파일러가 Copy trait를 자동으로 구현해주지 못합니다.

## 구조체

러스트에는 클래스가 없고 구조체만 있습니다. 상속 기능이 없기 때문에 완전한 OOP언어는 아닙니다.

구조체는 형태는 C 언어와 크게 다를게 없습니다. 아래 예제에 구조체 정의에 대한 모든 케이스가 모여있다고 생각됩니다.

```rust
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

딱히 설명할 것도 없이 흔하게 사용하는 구조체, 튜플입니다.

유닛 구조체라는게 좀 특이합니다. 아무런 변수도 없는 구조체입니다. 이건 나중에 트레이트Trait라는 클래스의 메소드와 같은 것을 사용하기 위한 구조체입니다. 클래스인데 내부 변수는 없고 메소드만 있는 클래스라고 생각하면 편합니다.

다른 언어와 확실히 다른게 있다면 구조체를 만들 때 인자로 사용된 객체의 소유권이 이동한다는 것입니다.

```rust
fn main() {
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };
    println!("{}", peter.name);
    println!("{}", name);
}

struct Person {
    name: String,
    age: u8,
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

이전에 소유권의 이동에 대해서 설명하면서 소유권이 없는 변수에 접근했을 때 보여드린 에러 메세지와 거의 동일한 형태의 에러 메세지를 다시 보게 됩니다.

- move occurs because `name` has type `String`, which does not implement the `Copy` trait: String 타입은 Copy trait라는걸 구현하지 않습니다. 컴파일러가 String타입의 메모리 크기가 얼마인지 알 수 없습니다. 지금 예제 코드는 “Peter”라는 리터럴을 String으로 만들기 때문에 메모리 크기를 알 수 있는 것처럼 보이지만, 동적으로 String을 만드는 경우를 생각하면 얼마나 긴 문자열을 생성할 지 알 수 없습니다.
- value moved here: name 변수의 소유권이 구조체 Person을 만들 때 이동했습니다.
- value borrowed here after move: println으로 소유권이 없는 변수에 접근했으므로 에러가 발생한 것입니다.
- consider cloning the value if the performance cost is acceptable: name.clone()으로 복사본을 만들어서 Person에 전달하는 것도 하나의 해결책이긴합니다만 불필요하게 메모리를 더 사용하게됩니다.

### 메소드 정의

구조체의 메소드를 정의하는 예제를 보겠습니다.

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

impl 키워드와 구조체 이름을 쓰고 하나의 블럭을 만듭니다. 그리고 &self를 첫번째 인자로 받는 함수를 만들면 메소드가 됩니다. 다른 언어들의 클래스 메소드를 만드는 것과 비슷합니다.

하나 눈여겨 별몬한건 f32라는 타입의 절대값을 구하는 abs라는 메소드가 2가지 형태로 사용된다는 것입니다.

1. 타입::메소드이름(..인자..)
2. 변수.메소드이름(..인자..)

1번은 보통 정적 메소드라고 하거나 연관 함수 Associated function 라고 부르는 것입니다. 구조체 타입에 종속되는 함수라서 구조체의 객체를 만들지 않아도 호출할 수 있습니다. 2번 동적 메소드는 객체를 반드시 만든 후에 객체를 이용해서 호출할 수 있는 메소드입니다. 그래서 첫번째 인자가 항상 &self가 됩니다.

메소드의 첫번째 인자에 &self만 사용할 수 있는게 아니라 &mut self를 쓸 수 있습니다. 구조체 내부 값을 변경하는 메소드라면 &mut self를 써야겠지요. 그리고 자기 자신의 메모리를 해지하는 메소드라면 self 인자를 갖을 것입니다.

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
    fn new() -> Rectangle {
        Rectangle {
            top_left: Point { x: 0.0, y: 0.0},
            bottom_right: Point { x: 0.0, y: 0.0},
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
    rect.destroy();
    println!("{}", rect.area());
}
```

```rust
error[E0382]: borrow of moved value: `rect`
  --> src/main.rs:34:20
   |
32 |     let rect = Rectangle::new();
   |         ---- move occurs because `rect` has type `Rectangle`, which does not implement the `Copy` trait
33 |     rect.destroy();
   |          --------- `rect` moved due to this method call
34 |     println!("{}", rect.area());
   |                    ^^^^^^^^^^^ value borrowed here after move
   |
note: `Rectangle::destroy` takes ownership of the receiver `self`, which moves `rect`
  --> src/main.rs:25:16
   |
25 |     fn destroy(self) {
   |                ^^^^
```

new라는 이름의 메소드는 러스트의 코딩 관례상 빈 객체를 생성하는 메소드의 이름으로 많이 쓰입니다. 그래서 보통 정적 메소드로 구현됩니다.

destroy라는 메소드는 인자를 self로 받아오므로 객체의 소유권을 가져옵니다. 따라서 메소드가 종료된 후부터는 객체를 더 이상 쓸 수 없습니다. new같이 특별히 정해진 이름이 있는 것은 아닙니다. 그리고 destroy와 같이 명시적으로 객체를 해지하는 메소드를 만드는건 특별한 일이 아니라면 잘 쓰지 않는 방법입니다. 예시로 보여드리기 위해서 제가 실험해본 것입니다.

### 구조체 디버깅 방법

구조체의 각 필드에 어떤 값이 저장된 상태인지 확인할 일이 자주 있습니다. C언어에서는 구조체마다 출력 함수를 만들기도 하고, 자바 등에서는 클래스에 출력 메소드를 만들기도 합니다. 러스트에서는 간단한 방법이 있습니다.

```rust
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

fn main() {
    let p = Point { x: 1.1, y:2.2 };
    println!("{:?}", p);
}
```

```rust
Point { x: 1.1, y: 2.2 }
```

구조체 이름과 각 필드의 이름과 값까지 출력해줘서 굉장히 편리합니다.

#[derive(Debug)]라는 구문은 std::fmt::Debug (Standard library에 속한 fmt라는 모듈에 정의된)라는  trait를 자동으로 구현하라는 의미입니다. 나중에 Trait에 대해서 설명할 때 정확한 의미를 알아보겠지만, 지금은 일단 “{:?}”라는 표현식을 써서 구조체의 각 필드의 값을 출력한다고 생각하면 됩니다. 구조체의 필드가 String같은 std에 정의된 타입이면 대부분 동작합니다. 만약 구조체의 한 필드가 또 다른 구조체 타입이라면, 그 다른 구조체도 #[derive(Debug)]를 선언해주면 됩니다.

```rust
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
    println!("area size={} {:?}", rect.area(), rect);
}
```