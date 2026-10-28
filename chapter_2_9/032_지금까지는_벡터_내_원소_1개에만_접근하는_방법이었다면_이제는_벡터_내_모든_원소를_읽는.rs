// Chapter: chapter 2-9장. 컬렉션 타입 (Vector, HashMap, HashSet)
// Section: 지금까지는 벡터 내 원소 1개에만 접근하는 방법이었다면, 이제는 벡터 내 모든 원소를 읽는 방법을 알아보겠습니다.


fn main() {

    let scores = vec![90, 80, 100];

    for score in scores.into_iter() {
        println!("{}", score);
    }

    // println!("{:?}", scores); 해당 코드는  borrow of moved value: `scores` 에러를 발생시킴
}
