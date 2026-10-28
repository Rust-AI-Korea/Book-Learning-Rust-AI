// Chapter: chapter 2-13장. 제네릭(Generic)과 라이프타임과 트레잇(Trait)
// Section: ① 제네릭이 없는 경우의 고충 (코드 중복)
// Source: src/chapter_2_13.md



fn print_i32_point(x: i32, y: i32) { println!("Point: {}, {}", x, y); }
fn print_f64_point(x: f64, y: f64) { println!("Point: {}, {}", x, y); }
