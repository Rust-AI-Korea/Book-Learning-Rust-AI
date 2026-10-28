// Chapter: chapter 2-14장. 반복자(Iterator)와 클로저(Closure)
// Section: ① 클로저의 기본 구조
// Source: src/chapter_2_14.md



fn main() {

    let print_num = |x| {
        println!("숫자: {}", x);
    };

    print_num(5);
    print_num(100);
}
