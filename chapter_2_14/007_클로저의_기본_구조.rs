// Chapter: chapter 2-14장. 반복자(Iterator)와 클로저(Closure)
// Section: ① 클로저의 기본 구조
// Source: src/chapter_2_14.md



fn main() {

    let name = "홍길동";

    let greet = || {
        println!("안녕하세요 {}님!", name);
    };

    greet();
}
