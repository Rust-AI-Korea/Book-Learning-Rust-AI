// Chapter: chapter 2-9장. 컬렉션 타입 (Vector, HashMap, HashSet)
// Section: 모든 값을 동일하게 변경하기 fill()


fn main() {

    let mut numbers = vec![1, 2, 3, 4];

    numbers.fill(0);

    println!("{:?}", numbers);
}
