// Chapter: chapter 3-1장. serde: AI 데이터를 위한 초고속 데이터 직렬화/역직렬화
// Section: 5. 실무 응용: 카멜 케이스(JSON) vs 스네이크 케이스(러스트) 맵핑
// Source: src/chapter_3_1.md



use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct ClientRequest {
    // 💡 이 필드는 실제 JSON 상에서 "userId"라는 이름으로 매핑하겠다고 선언합니다.
    #[serde(rename = "userId")]
    user_id: String,
    
    // 만약 구조체 안의 모든 필드가 카멜케이스 기반이라면 통째로 전환도 가능합니다.
    // #[serde(rename_all = "camelCase")] <-- 구조체 상단 선언 시 전체 자동 적용
    
    #[serde(rename = "isTarget")]
    is_target: bool,
}
