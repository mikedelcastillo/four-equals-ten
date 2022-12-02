type N = f64;

#[derive(Debug, Clone, PartialEq)]
enum Op {
    Add,
    Mul,
    Sub,
    Div,
}

#[derive(Debug, Clone)]
enum Parens {
    Span(usize, usize),
    None,
}

#[derive(Debug, Clone)]
enum Computation {
    Solved(N),
    Expression {
        nums: Vec<N>,
        ops: Vec<Op>,
        parens: Parens,
    },
}

impl Computation {
    fn new(nums: Vec<N>, ops: Vec<Op>, parens: Parens) -> Self {
        Self::Expression { nums, ops, parens }
    }

    fn from_comp(nums: Vec<N>, ops: Vec<Op>) -> Self {
        if nums.len() == 1 {
            Self::Solved(nums[0])
        } else {
            Self::Expression {
                nums,
                ops,
                parens: Parens::None,
            }
        }
    }

    fn step(&self) -> Self {
        let comp = self.clone();

        // If computation is already solved
        if let Self::Solved(_) = comp {
            return comp;
        } else if let Self::Expression { nums, ops, parens } = comp {
            let mut mod_nums = nums.clone();
            let mut mod_ops = ops.clone();

            // Parentheses
            if let Parens::Span(start, end) = parens {
                let result = Self::new(
                    nums[start..=end].to_vec(),
                    ops[start..end].to_vec(),
                    Parens::None,
                );

                mod_nums.splice(start..=end, vec![result.solve()]);
                mod_ops.splice(start..end, []);

                return Self::from_comp(mod_nums, mod_ops);
            }

            // Division
            let index = ops.iter().position(|op| op == &Op::Div);
            if let Some(index) = index {
                let result = nums[index] / nums[index + 1];

                mod_nums.splice(index..=index + 1, vec![result]);
                mod_ops.splice(index..=index, []);

                return Self::from_comp(mod_nums, mod_ops);
            }

            // Multiplication
            let index = ops.iter().position(|op| op == &Op::Mul);
            if let Some(index) = index {
                let result = nums[index] * nums[index + 1];

                mod_nums.splice(index..=index + 1, vec![result]);
                mod_ops.splice(index..=index, []);

                return Self::from_comp(mod_nums, mod_ops);
            }

            // Addition
            let index = ops.iter().position(|op| op == &Op::Add);
            if let Some(index) = index {
                let result = nums[index] + nums[index + 1];

                mod_nums.splice(index..=index + 1, vec![result]);
                mod_ops.splice(index..=index, []);

                return Self::from_comp(mod_nums, mod_ops);
            }

            // Subtraction
            let index = ops.iter().position(|op| op == &Op::Sub);
            if let Some(index) = index {
                let result = nums[index] - nums[index + 1];

                mod_nums.splice(index..=index + 1, vec![result]);
                mod_ops.splice(index..=index, []);

                return Self::from_comp(mod_nums, mod_ops);
            }
        }

        unreachable!()
    }

    fn solve(&self) -> N {
        let mut comp = self.clone();
        let mut count = 0;
        loop {
            count += 1;
            println!("STEP #{}: {:?}", count, comp);
            if count > 5 {
                panic!("took too long");
            }
            if let Computation::Solved(n) = comp {
                return n;
            } else {
                comp = comp.step()
            }
        }
    }
}

fn main() {
    let problem = Computation::new(
        vec![1.0, 0.0, 6.0, 3.0],
        vec![Op::Sub, Op::Sub, Op::Sub],
        Parens::Span(1, 3),
    );

    println!("{:?} = {:?}", &problem, &problem.solve());
}
