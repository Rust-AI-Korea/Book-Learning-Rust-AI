// Chapter: chapter 2-9장. 컬렉션 타입 (Vector, HashMap, HashSet)
// Section: 지금까지는 벡터 내 원소 1개에만 접근하는 방법이었다면, 이제는 벡터 내 모든 원소를 읽는 방법을 알아보겠습니다.


fn main() {
    let v = vec![1,2,3,4,5];

    //레퍼런스로 접근하기
    for a in &v{ print!("{} ",*a); }  //1 2 3 4 5
    println!("");

    for a in &v{ print!("{} ",a); }  //1 2 3 4 5  print! 매크로는 레퍼런스 변수에 대해서도 안의 내용을 출력함
    println!("");

    //소유권 이동
    for a in v{ print!("{} ",a); }  //1 2 3 4 5
    println!("");
}
