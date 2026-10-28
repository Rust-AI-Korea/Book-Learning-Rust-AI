// Chapter: chapter 2-14장. 반복자(Iterator)와 클로저(Closure)
// Section: ② 주변 환경의 변수 훔쳐오기 (Variable Capture)
// Source: src/chapter_2_14.md

fn main() {
    let num = 100;
    let add_num = |x| x + num;
    println!("{}", add_num(5));
}
