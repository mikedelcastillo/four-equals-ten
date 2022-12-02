#[derive(Debug)]
enum NumOp {
    Add(Box<NumOp>, Box<NumOp>),
    Sub(Box<NumOp>, Box<NumOp>),
    Mul(Box<NumOp>, Box<NumOp>),
    Div(Box<NumOp>, Box<NumOp>),
    N(i32),
}

impl NumOp {
    fn compute(&self) -> i32 {
        match self {
            NumOp::Add(a, b) => a.compute() + b.compute(),
            NumOp::Sub(a, b) => a.compute() - b.compute(),
            NumOp::Mul(a, b) => a.compute() * b.compute(),
            NumOp::Div(a, b) => a.compute() / b.compute(),
            NumOp::N(a) => *a,
        }
    }
}

// _ n _ ~ _ n _ ~ _ n _ ~ _ n _

// (1+2)+3+4
// (1+2+3)+4
// 1+(2+3)+4
// 1+(2+3+4)
// 1+2+(3+4)

fn main() {
    let add = NumOp::Mul(
        Box::new(NumOp::N(5)), 
        Box::new(NumOp::Add(
            Box::new(NumOp::N(55)),
            Box::new(NumOp::Add(
                Box::new(NumOp::N(55)),
                Box::new(NumOp::Add(
                    Box::new(NumOp::N(55)),
                    Box::new(NumOp::N(2)),
                )),
            )),
        )));

    println!("{:?} = {}", add, add.compute())
}
