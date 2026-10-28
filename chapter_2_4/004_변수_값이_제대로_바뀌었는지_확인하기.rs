// Chapter: chapter 2-4장. 변수(가변성)와 상수
// Section: 변수 값이 제대로 바뀌었는지 확인하기

fn main() {
    let mut count = 0;

    count += 1;
    count += 1;

    dbg!(count);
}
