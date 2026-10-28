// Chapter: chapter 2-15장. 스마트 포인터 (Box, Rc, Arc)
// Section: 🔍 돋보기: Box 없이는 재귀 구조를 만들 수 없는 이유
// Source: src/chapter_2_15.md

// ❌ 컴파일 에러 발생! (recursive type has infinite size)
struct TreeNode {
    value: f32,
    left: TreeNode,  // 내 안에 내가 또 들어감 (무한대 크기)
    right: TreeNode, // 내 안에 내가 또 들어감
}
