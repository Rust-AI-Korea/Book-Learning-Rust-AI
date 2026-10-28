// Chapter: chapter 2-9장. 컬렉션 타입 (Vector, HashMap, HashSet)
// Section: 기존 벡터 뒤에 다른 컬렉션(배열, 벡터 등)  원소들을 연속으로 추가(extend 메서드)


fn main() {
    let mut v = vec![5,6,7];
    v.extend([10,11,12]);
    println!("{:?}", v);
}
