// Chapter: chapter 2-7장. 제어문 (if 조건문과 loop/while/for 반복문)
// Section: ① 무한 루프의 정석: loop


fn main() {
    let p = max_factor(10);
    println!("가장 큰 소인수: {}", p); // 결과: 5
}

fn max_factor(mut n: u64) -> u64 {   
    let mut p = 2; // 2부터 나누기 시작
    
    loop {                         
        // 1. 일단 나누어보고 몫(q)과 나머지(r)를 구합니다. (우선 실행)
        let (q, r) = (n / p, n % p);        

        // 2. 연산 결과를 바탕으로 탈출 조건을 검사합니다.
        if q == 1 { break; } // 더 이상 나눌 수 없으면 탈출 (p가 곧 n인 상태)

        // 3. 나머지에 따라 값을 갱신합니다.
        if r == 0 { 
            n = q; // 나누어떨어지면 n을 몫으로 갱신
        } else { 
            p += 1; // 나누어떨어지지 않으면 나누는 수를 증가
        }      
    }
    n
}
