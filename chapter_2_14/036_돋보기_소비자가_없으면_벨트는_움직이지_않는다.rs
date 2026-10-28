// Chapter: chapter 2-14장. 반복자(Iterator)와 클로저(Closure)
// Section: 🔍 돋보기: 소비자가 없으면 벨트는 움직이지 않는다
// Source: src/chapter_2_14.md



fn main() {
    let nums = vec![1, 2, 3];
    
    // 어댑터(map)로 가공 계획만 세우고, 최종 소비자(collect, sum 등)를 호출하지 않음
    nums.iter().for_each(|x| println!("연산 가동: {}", x)); 
}
