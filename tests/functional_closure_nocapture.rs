#![allow(dead_code)]

/// 함수를 받아서 함수를 두번 실행하는 함수를 반환하는 함수를 만들고
/// inc함수를 인자로 전달해본다.

fn double<FA: Fn(i32) -> i32>(callee: FA) -> impl Fn(i32) -> i32 {
    move |x| callee(callee(x))
}

fn inc(x: i32) -> i32 {
    println!("inc-{}", x);
    x + 1
}

fn main() {
    let r = double(|x| x + 1);
    println!("result={}", r(5));

    let r = double(inc);
    println!("result={}", r(5));
}

#[test]
fn test_functional_closure_nocapture() {
    let f = double(double(inc));
    assert_eq!(9, f(5));

    let _f = double(double(double(inc)));
    //assert_eq!(0 /* FIX this number */, f(5));

    let _f = double(double(double(double(inc))));
    //assert_eq!(0 /* FIX this number */, f(5));
}
