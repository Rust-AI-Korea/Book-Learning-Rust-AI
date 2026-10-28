// Chapter: chapter 2-15장. 스마트 포인터 (Box, Rc, Arc)
// Section: 1. 데이터를 힙(Heap) 공간에 고정하기: Box< T >
// Source: src/chapter_2_15.md



fn main() {

    let num = Box::new(10);

    println!("{}", *num);
}
