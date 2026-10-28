// Chapter: chapter 2-12장. 열거형(Enum)과 패턴 매칭(match)
// Section: 타입으로 구분한다는 것은 무슨 의미일까?
// Source: src/chapter_2_12.md

enum Member {
    Normal,
    Vip,
}

fn get_member(point: u32) -> Member {

    if point >= 1000 {
        Member::Vip
    } else {
        Member::Normal
    }
}
