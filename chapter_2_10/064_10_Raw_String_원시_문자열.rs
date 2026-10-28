// Chapter: chapter 2-10장. 문자열의 비밀 (char, &str, String의 삼각관계)
// Section: 10. Raw String (원시 문자열)
// Source: src/chapter_2_10.md

fn main() {
    let html = r#"
<div>
    <h1>Hello Rust</h1>
</div>
"#;

    println!("{}", html);
}
