## 옵션 추가

표준 인터페이스와 플러그인을 만드는 것은 지금까지는 잘 동작하고 있습니다.
그런데 아직 뭔가 실무에 쓰일만한 프로그램이라고 보기에는 조금 부족한 면이 있습니다.
사실 우리가 만들었던 것과 같이 사용자에게 일일이 뭔가를 물어봐서 입력을 받는 방식으로 동작하는 것은 편리할 때도 있지만, 시리얼 번호를 만드는 경우에는 그다지 좋은 방식이라고는 생각되지 않습니다.
어짜피 사용법을 아는 같은 회사의 직원이 사용하는 프로그램이므로 그렇게까지 친절해야될 필요는 없고, 빨리 실행해서 결과를 얻을 수 있는게 더 좋을것 같습니다.
그래서 이번에는 커맨드 라인에서 입력을 받을 수 있도록 바꿔보겠습니다.

대략 다음과 같은 방식으로 동작하면 될것 같습니다.

```bash
$ serial --help
모든 입력 데이터의 help 메세지 출력
$ serial generate --userid 1234 --productid qwerasdf
..생성
```

### Clap 크레이트 사용법

암호화를 위해서 크레이트를 조사해보고 MagicCrypt 크레이트를 찾아내서 예제 코드를 찾아서 사용법을 알아보는 과정을 이야기했었습니다.
비슷한 방식으로 커맨드 라인 입력을 처리하는 크레이트를 찾아서 사용할 수도 있습니다.
그런데 저는 Clap이라는 크레이트를 예전에 회사 업무로 사용한 경험이 있어서 별다른 조사없이 Clap을 사용하기로 결정했습니다.

먼저 Clap 크레이트의 홈페이지(https://docs.rs/clap/latest/clap/)에 있는 예제 코드를 한번 보면서 대략 어떤 크레이트인지를 알아보겠습니다. 참고로 clap의 예제 코드를 지금 우리가 작업하는 패키지에서 실행하면 기존 코드와 섞이게되니 새로운 패키지를 하나 만들어서 실행해보시기 바랍니다.

다음과 같이 ex-clap이라는 패키지를 만들고, clap 크레이트를 추가하고, derive라는 기능을 활성화합니다.

```bash
$ cargo new ex-clap
$ cd ex-clap
$ cargo add clap --features derive
```

그리고 다음 예제를 main.rs에 복사합니다.
```rust
use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let args = Args::parse();

    for _ in 0..args.count {
        println!("Hello {}!", args.name);
    }
}
```

Args 구조체의 윗줄에 있는 `#[derive(Parser, Debug)]` 구문은 커맨드 라인 옵션을 처리하는 파서를 자동으로 생성해주라는 의미입니다. Args 구조체의 트레이트 구현으로 옵션 처리 코드를 구현해줍니다. 그래서 main 함수를 보면 커맨드 라인 입력을 처리하는 코드가 전혀 없고 단지 Args구조체의 parse라는 메소드를 호출하는 것 뿐입니다. 이렇게 derive 기능으로 커맨드 라인 옵션 처리 코드를 구현하므로 cargo add 명령을 실행할 때 `--features derive` 옵션을 추가한 것입니다.

그 다음 줄에 있는 `#[command(version, about, long_about = None)]` 구문은 version과 help 옵션을 자동으로 생성해주라는 의미입니다. about이 help에 해당합니다.

그리고 어떤 옵션들이 있는지를 지정하는 것은 Args 구조체의 이 프로그램은 2개의 옵션, --name(-n)과 --cont(-c)를 처리할 수 있다는 것을 알 수 있습니다. Args 구조체에 name과 count필드 위에 `#[arg(short, long)]`이라는 derive가 있기 때문에, clap 크레이트에서 자동으로 각 필드 이름, name과 count에 해당하는 옵션을 만들어줍니다. count필드에는 디폴트 값이 1이라는 표시도 있으므로, 입력이 없으면 자동으로 1을 저장해줍니다. name필드는 디폴트 값이 없으므로 옵션을 지정하지 않으면 프로그램이 실패할 것입니다. 그리고 각 옵션에 대한 설명은 각 필드의 바로 윗줄에 있는 주석에 쓰여있습니다.

예제를 실행해보면서 name과 count 옵션이 어떻게 처리되는지를 확인해보겠습니다.

```bash
$ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.06s
     Running `target/debug/bin-example`
error: the following required arguments were not provided:
  --name <NAME>

Usage: bin-example --name <NAME>

For more information, try '--help'.
$ cargo run -- --help
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/bin-example --help`
Simple program to greet a person

Usage: bin-example [OPTIONS] --name <NAME>

Options:
  -n, --name <NAME>    Name of the person to greet
  -c, --count <COUNT>  Number of times to greet [default: 1]
  -h, --help           Print help
  -V, --version        Print version
```

아무런 커맨드 라인 옵션을 주지 않으면 필수 옵션이 없다는 안내 메세지가 출력됩니다.
그리고 --help 옵션을 주면 각 옵션에 대한 설명을 출력합니다.
예제 코드에서 본대로 Args 구조체의 각 필드에 써준 주석이 도움말로 출력되는 것을 확인할 수 있습니다. 그래서 아래와 같이 프로그램 옵션을 지정해서 실행하게됩니다.

```bash
$ cargo run -- -n Gioh --count 2
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.05s
     Running `target/debug/bin-example -n Gioh --count 2`
Hello Gioh!
Hello Gioh!
```

### Clap의 다른 사용법

우리가 만들 시리얼 프로그램은 예제와 같이 항상 동일한 옵션을 가지지 않습니다.
어떤 경우에는 지금까지 우리가 만든 4개의 입력 데이터를 모두 사용할 때도 있지만, 어떤 제품은 4개 입력 데이터 중에 한두개만 사용할 수도 있습니다.

따라서 이전 예제의 Args와 같이 고정된 데이터 구조를 만들어서 늘 고정된 옵션을 처리하도록 만들 수가 없습니다. 좀 더 옵션을 유연하게 만들 수 있도록 수정해야합니다.
그렇게 유연한 옵션 처리를 위해서 clap은 다음과 같이 Arg와 Command라는 데이터 구조를 제공하고 있습니다.
아래 예제는 Arg와 Command를 이용해서 이전 예제와 같이 count 옵션에 받은 수만큼 이름을 출력하는 예제입니다.

```rust
use clap::{Arg, Command};

fn main() {
    // clap::Command타입의 객체 생성
    // --version과 --help 옵션에 대한 설정을 추가함
    let mut command = Command::new("serial") // 프로그램의 이름은 serial
        .version("0.1.0") // cargo run -- --version 명령을 실행하면 0.1.0이 출력됨
        .about("Serial number generator"); // cargo run -- --help 명령을 실행하면 출력되는 프로그램 설명

    // "Name"이라는 ID를 가진 Arg타입의 객체 생성
    // --name 혹은 -n 옵션으로 지정가능
    // 디폴트 값은 없고 반드시 옵션으로 지정되어야함
    let arg = Arg::new("NAME")
        .long("name")
        .short('n')
        .help("Name of the person to greet")
        .required(true);
    // command의 arg 메소드를 호출해서 "Name" 옵션을 추가함. arg의 반환값은 새로 생성된 command 객체임
    command = command.arg(arg);
    // "Count"라는 ID를 가진 Arg타입의 객체 셍성
    // --count 혹은 -c 옵션으로 지정가능
    // 커맨드 라인 옵션으로 값을 입력받지 못하면 1값을 디폴트로 사용함
    let arg = Arg::new("COUNT")
        .long("count")
        .short('c')
        .help("Number of times to greet")
        .default_value("1");
    // 옵션 이름으로 Arg타입 객체를 만든 후 command 객체에 추가해줌
    command = command.arg(arg);

    // get_matches함수는 프로그램에 전달된 모든 커맨드 라인 옵션을 읽어옴 - parser와 같은 역할
    let matches = command.get_matches();

    // --name/-n 옵션을 읽어옴
    if let Some(name) = matches.get_one::<String>("NAME") {
        // --count/-c 옵션을 읽어옴
        if let Some(count) = matches.get_one::<String>("COUNT") {
            let count: usize = count.parse::<usize>().unwrap_or(1);
            for _ in 0..count {
                println!("Hello {}!", name);
            }
        }
    }
}
```

가장 처음에 해야 할 일은 Command타입의 객체를 만드는 것입니다.
예제 코드에서는 Command 구조체의 3가지 메소드를 사용하고 있습니다.
* new: Command 타입의 객체를 생성합니다
* version: 프로그램의 버전을 지정합니다
* about: help 옵션을 실행하면 출력될 안내 메세지를 지정합니다.

그 다음은 우리가 만들고자하는 2개의 옵션 `--name`과 `--count`에 