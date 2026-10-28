// Chapter: chapter 2-15장. 스마트 포인터 (Box, Rc, Arc)
// Section: 🔍 돋보기: Box 없이는 재귀 구조를 만들 수 없는 이유
// Source: src/chapter_2_15.md

// ⭕ 컴파일 성공!
struct TreeNode {
    value: f32,
    left: Option<Box<TreeNode>>,  // 힙 메모리를 가리키는 고정된 주소값 상자(8바이트)
    right: Option<Box<TreeNode>>, // 고정된 주소값 상자(8바이트)
}
