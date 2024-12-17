#![allow(dead_code)]

/*func adder() func(int) int {
    sum := 0
    return func(x int) int {
        sum += x
        return sum
    }
}

func main() {
    pos, neg := adder(), adder()
    for i := 0; i < 10; i++ {
        fmt.Println(
            pos(i),
            neg(-2*i),
        )
    }
}
 */

fn main() {}

fn adder() -> impl FnMut(i32) -> i32 {
    let mut sum = 0;
    move |x| {
        sum += x;
        sum
    }
}

#[test]
fn test_functional_closure_capture() {
    // pos와 neg는 각각 sum이라는 변수를 캡쳐합니다.
    // 두 함수는 같은 이름의 sum이라는 변수를 캡쳐했지만, 서로 다른 함수에서 다른 값을 가집니다.
    // 아래 주석에 ??에 들어갈 값을 예상해보세요.
    let mut pos = adder();
    let mut neg = adder();
    assert_eq!(0, pos(0));
    assert_eq!(0, neg(0));

    // i = 1
    assert_eq!(1, pos(1)); // pos(i)
    assert_eq!(-2, neg(-2 * 1)); // neg(-2 * i)

    // i = 2
    assert_eq!(3, pos(2)); // pos(i)
    assert_eq!(-6, neg(-2 * 2)); // neg(-2 * 2)

    // i = 3
    //assert_eq!(??, pos(3)); // pos(i)
    //assert_eq!(??, neg(-2 * 3)); // neg(-2 * 3)

    // i = 4
    //assert_eq!(??, pos(4)); // pos(i)
    //assert_eq!(??, neg(-2 * 4)); // neg(-2 * 4)

    // i = 5
    //assert_eq!(??, pos(5)); // pos(i)
    //assert_eq!(??, neg(-2 * 5)); // neg(-2 * 5)
}
