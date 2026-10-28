// Chapter: chapter 2-12장. 열거형(Enum)과 패턴 매칭(match)
// Section: 실습 예제: 게임 캐릭터의 체력(HP)과 마나(MP) 상태 동시 판별
// Source: src/chapter_2_12.md

fn main() {
    let hp = 0;
    let mp = 50;

    // (HP가 0인지 여부, MP가 0인지 여부)를 튜플로 만들어 던집니다.
    match (hp == 0, mp == 0) {
        (true, true)   => println!("💀 캐릭터가 완전히 사망하여 유령이 되었습니다."),
        (true, false)  => println!("🚨 체력이 다해 기절했습니다. 부활이 필요합니다."), // 출력됨
        (false, true)  => println!("🔮 마나가 고갈되어 일반 공격만 가능합니다."),
        (false, false) => println!("⚔️ 전투 준비 완료! 상태가 양호합니다."),
    }
}
