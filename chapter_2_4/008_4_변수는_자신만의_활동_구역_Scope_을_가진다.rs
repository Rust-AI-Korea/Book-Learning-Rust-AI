// Chapter: chapter 2-4장. 변수(가변성)와 상수
// Section: 4. 변수는 자신만의 활동 구역(Scope)을 가진다

fn main() {
    {
        let score = 100;
    }

    println!("{}", score);
}
