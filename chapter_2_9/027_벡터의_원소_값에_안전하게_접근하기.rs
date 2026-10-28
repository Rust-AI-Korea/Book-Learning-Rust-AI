// Chapter: chapter 2-9장. 컬렉션 타입 (Vector, HashMap, HashSet)
// Section: ② 벡터의 원소 값에 안전하게 접근하기


fn main() {
    let v = vec![1,2,3,4,5];

    println!("v[5]={}", v[5]);   //인덱스 오류 발생: index out of bounds: the len is 5 but the index is 5

    if let Some(n) = v.get(5) {
        println!("v.get(5)={:?}", n);
    }    //프로그램은 정상 실행되나 None
}
