// Chapter: chapter 2-9장. 컬렉션 타입 (Vector, HashMap, HashSet)
// Section: ② 벡터의 원소 값에 안전하게 접근하기


fn main() {
    let v = vec![1,2,3,4,5];
    if let Some(n) = v.get(2) {
        println!("v.get(2)={:?}", n);
    }    
}
