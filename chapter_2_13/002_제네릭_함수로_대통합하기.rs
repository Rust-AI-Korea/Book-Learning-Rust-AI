// Chapter: chapter 2-13장. 제네릭(Generic)과 라이프타임과 트레잇(Trait)
// Section: ② 제네릭 함수로 대통합하기
// Source: src/chapter_2_13.md



// T는 "아직 정해지지 않은 임의의 타입"을 의미합니다.
struct Point <T> {
    x:T, y:T
}

fn main() {
    let p1 = Point { x: 5, y: 8 };
    let p2 = Point { x: 1.0, y: 2.5 };

    println!("p1=({},{})",p1.x,p1.y); 
    println!("p2=({},{})",p2.x,p2.y);
}

