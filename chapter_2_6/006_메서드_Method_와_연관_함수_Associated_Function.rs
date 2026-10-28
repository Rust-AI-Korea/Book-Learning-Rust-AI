// Chapter: chapter 2-6장. 함수(Functions)와 반환값(Return Value)
// Section: 메서드(Method)와 연관 함수(Associated Function)


struct Car {
    brand: String,
    speed: u32,
}

impl Car {
    // 1. 연관 함수 (Associated Function)
    fn new(brand: &str) -> Car {
        Car {
            brand: brand.to_string(),
            speed: 0,
        }
    }

    // 2. 메서드 (Method)
    fn accelerate(&mut self, amount: u32) {
        self.speed += amount;
        println!("{}의 속도가 {}km/h가 되었습니다.", self.brand, self.speed);
    }
}
