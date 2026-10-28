// Chapter: chapter 4-1장. Rust로 알아보는 자동미분(AutoGrad)
// Section: 4. 실무 검증: Value 객체에 미분값 저장해 보기
// Source: src/chapter_4_1.md



use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub struct Value(
    Rc<RefCell<ValueData>>
);

struct ValueData {
    data: f64,
    grad: f64,
}

impl Value {

    pub fn new(data: f64) -> Self {

        Value(
            Rc::new(
                RefCell::new(
                    ValueData {
                        data,
                        grad: 0.0,
                    }
                )
            )
        )
    }

    pub fn data(&self) -> f64 {
        self.0.borrow().data
    }

    pub fn grad(&self) -> f64 {
        self.0.borrow().grad
    }

    pub fn set_grad(
        &self,
        grad: f64
    ) {
        self.0.borrow_mut().grad = grad;
    }
}

fn main() {

    let x = Value::new(2.0);
    let w = Value::new(-3.0);
    let b = Value::new(10.0);

    // 순전파
    let f =
        (x.data() * w.data())
        + b.data();

    println!("=== 순전파 ===");
    println!("f = {}", f);

    // 수동 역전파
    x.set_grad(w.data());
    w.set_grad(x.data());
    b.set_grad(1.0);

    println!("\n=== 역전파 결과 ===");

    println!(
        "df/dx = {}",
        x.grad()
    );

    println!(
        "df/dw = {}",
        w.grad()
    );

    println!(
        "df/db = {}",
        b.grad()
    );
}
