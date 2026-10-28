// Chapter: chapter 2-15장. 스마트 포인터 (Box, Rc, Arc)
// Section: 📦 Box가 문제를 해결한다
// Source: src/chapter_2_15.md

fn find_runner(
    is_human: bool
) -> Box<dyn Moving>
{
    if is_human {
        Box::new(Human)
    } else {
        Box::new(Dog)
    }
}
