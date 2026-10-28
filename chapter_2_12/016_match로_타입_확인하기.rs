// Chapter: chapter 2-12장. 열거형(Enum)과 패턴 매칭(match)
// Section: match로 타입 확인하기
// Source: src/chapter_2_12.md

fn main() {

    let member = get_member(1500);

    match member {

        Member::Vip => {
            println!("VIP 회원입니다.");
        }

        Member::Normal => {
            println!("일반 회원입니다.");
        }
    }
}
