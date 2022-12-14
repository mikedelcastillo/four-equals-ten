pub type N = f64;

#[derive(Debug, Clone, PartialEq)]
pub enum Op {
    Add,
    Mul,
    Sub,
    Div,

    Mod,
    Exp,
}

#[derive(Debug, Clone)]
pub enum Parens {
    Span(usize, usize),
    None,
}

#[derive(Debug, Clone)]
pub enum Equation {
    Solved(N),
    Expression {
        nums: Vec<N>,
        ops: Vec<Op>,
        parens: Parens,
    },
}

impl Equation {
    pub fn new(nums: Vec<N>, ops: Vec<Op>, parens: Parens) -> Self {
        Self::Expression { nums, ops, parens }
    }

    pub fn from_comp(nums: Vec<N>, ops: Vec<Op>) -> Self {
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

            // Exponent
            let index = ops.iter().position(|op| op == &Op::Exp);
            if let Some(index) = index {
                let result = nums[index].powf(nums[index + 1]);

                mod_nums.splice(index..=index + 1, vec![result]);
                mod_ops.splice(index..=index, []);

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

            // Mod
            let index = ops.iter().position(|op| op == &Op::Mod);
            if let Some(index) = index {
                let result = nums[index] % nums[index + 1];

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

    pub fn solve(&self) -> N {
        let mut comp = self.clone();
        let mut count = 0;
        loop {
            count += 1;
            // println!("({}) (step{}) = {}", self.format(), count, comp.format());
            if count > 100 {
                panic!("took too long");
            }
            if let Equation::Solved(n) = comp {
                return n;
            } else {
                comp = comp.step()
            }
        }
    }

    pub fn format(&self) -> String {
        match self {
            Equation::Solved(n) => n.to_string(),
            Equation::Expression { nums, ops, parens } => {
                let mut output = String::new();

                for (i, n) in nums.iter().enumerate() {
                    let part = n.to_string();
                    let part = match parens {
                        Parens::Span(start, end) => {
                            if &i == start {
                                format!("({}", part)
                            } else if &i == end {
                                format!("{})", part)
                            } else {
                                part
                            }
                        }
                        Parens::None => part,
                    };

                    let op = if i < ops.len() {
                        match ops[i] {
                            Op::Add => "+",
                            Op::Mul => "*",
                            Op::Sub => "-",
                            Op::Div => "/",
                            Op::Mod => "%",
                            Op::Exp => "^",
                        }
                    } else {
                        ""
                    };

                    output = format!("{} {} {}", output, part, op)
                }

                output.trim().to_string()
            }
        }
    }
}
