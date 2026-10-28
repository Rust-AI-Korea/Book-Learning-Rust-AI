// Chapter: chapter 2-5장. 기본 출력과 형식 지정자 (Formatting)
// Section: ③ 이름을 지정하여 출력 (Named Arguments)


fn main() {
    println!(
        "모델명: {name}, 버전: {version}",
        name = "Model-name",
        version = 2.5
    );
}
