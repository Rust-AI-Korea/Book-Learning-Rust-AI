// Chapter: chapter 2-11장. 구조체(Struct)와 메서드 구현(impl)
// Section: 구조체도 소유권(Ownership)을 가진다
// Source: src/chapter_2_11.md


struct AiModel {
    name: String,
    version: f64,
}

fn main() {
    let name = String::from("Qwen");
    let version = 3.0;

    let model = AiModel {
        name,
        version,
    };

    println!("{}", model.name);
}
