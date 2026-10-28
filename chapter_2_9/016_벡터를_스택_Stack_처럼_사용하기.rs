// Chapter: chapter 2-9장. 컬렉션 타입 (Vector, HashMap, HashSet)
// Section: 벡터를 스택(Stack)처럼 사용하기


fn main() {

    let mut stack = Vec::new();

    stack.push(10);
    stack.push(20);
    stack.push(30);

    while let Some(value) = stack.pop() {
        println!("{}", value);
    }
}
