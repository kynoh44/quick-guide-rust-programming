#![allow(dead_code)]

fn check_fibonacci(_v: i32) -> bool {
    // v로 전달받은 수가 피보나치 수열에 있는 수인지 확인하는 함수입니다.
    // 피보나치 수열에 속한 수이면 true, 아니면 false를 반환하세요.
    false
}

fn get_fibonacci(_limit: i32) -> Vec<i32> {
    // 피보나치 수열에서 limit보다 작은 수만 반환해주는 함수입니다.
    // map과 filter를 이용해서 구현해보세요.
    vec![1, 1, 2, 3, 5, 8]
}

#[test]
fn test_functional_filter() {
    assert_eq!(false, check_fibonacci(4));
    assert_eq!(vec![1, 1, 2, 3, 5, 8], get_fibonacci(10));
}
