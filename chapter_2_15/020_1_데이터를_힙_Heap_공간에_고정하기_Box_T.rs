// Chapter: chapter 2-15장. 스마트 포인터 (Box, Rc, Arc)
// Section: 1. 데이터를 힙(Heap) 공간에 고정하기: Box< T >
// Source: src/chapter_2_15.md



fn main() {

    let gift = Box::new("사과");

    println!("상자 안에는 {}가 들어있습니다.", *gift);
}
