// Chapter: chapter 2-11장. 구조체(Struct)와 메서드 구현(impl)
// Section: 구조체도 소유권(Ownership)을 가진다
// Source: src/chapter_2_11.md

struct Student {
    name: String,
}

fn main() {
    let s1 = Student {
        name: String::from("홍길동"),
    };

    let s2 = s1;

    // println!("{}", s1.name);
}
