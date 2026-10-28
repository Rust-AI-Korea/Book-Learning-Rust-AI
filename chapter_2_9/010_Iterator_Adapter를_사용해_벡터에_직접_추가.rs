// Chapter: chapter 2-9장. 컬렉션 타입 (Vector, HashMap, HashSet)
// Section: Iterator Adapter를 사용해 벡터에 직접 추가


fn main() {
    let mut v = vec![1,2,3];
    let arr = [4,5,6,7];

    arr.into_iter().for_each(|x| v.push(x));
    println!("{:?}",v); 
}
