// Chapter: chapter 2-15장. 스마트 포인터 (Box, Rc, Arc)
// Section: 어떤 객체를 반환할지 실행 중에 결정하고 싶다
// Source: src/chapter_2_15.md

fn find_runner(is_human: bool) -> Moving {

    if is_human {
        Human
    } else {
        Dog
    }

}
