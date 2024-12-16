fn main() {}

#[cfg(test)]
mod tests {
    #[test]
    fn test_smart_pointer_basic() {
        // 아래 코드는 패닉을 발생시키고 테스트를 종료시킵니다.
        // 왜 패닉이 발생할까요? 어떡게 고칠 수 있을까요?
        // 힌트: Box만 사용하면 고칠 수 없습니다.
        // Box::new([0; 1024*1024*1024])와 같이 Box에 전달할 슬라이스를 만들 때 이미 스택에 배열을 생성한 후 Box에 전달하기 때문입니다.
        // reference: https://doc.rust-lang.org/std/vec/struct.Vec.html#impl-TryFrom%3CVec%3CT%3E%3E-for-Box%3C%5BT;+N%5D%3E
        //let big_array: [i32; 1024 * 1024 * 1024] = [0; 1024 * 1024 * 1024];
        //let l = big_array.len();
        //println!("last element:{}", big_array[l - 1]);
    }
}
