use itertools::Itertools;

use four_equals_ten::equations::{Equation, Op, Parens};

fn main() {
    let nums = [3, 3, 5, 6];
    let ops_base = [Op::Add, Op::Sub, Op::Mul, Op::Div, Op::Mod, Op::Exp];
    let target = 10;

    let paren_spans = (0..nums.len()).collect::<Vec<usize>>();

    for nums_perm in nums.iter().permutations(nums.len()).unique() {
        for paren_spans_perm in paren_spans.iter().combinations(2).unique() {
            let parens = if paren_spans_perm[0] == &0 && paren_spans_perm[1] == &(nums.len() - 1) {
                Parens::None
            } else {
                Parens::Span(*paren_spans_perm[0], *paren_spans_perm[1])
            };
            for ops_perm in ops_base
                .iter()
                .combinations_with_replacement(nums.len() - 1)
            {
                let nums = nums_perm.iter().map(|n| f64::from(**n)).collect_vec();
                let ops = ops_perm.iter().cloned().cloned().collect_vec();
                let eq = Equation::Expression {
                    nums,
                    ops,
                    parens: parens.clone(),
                };
                let solved = eq.solve();
                if solved == f64::from(target) {
                    println!("{} = {}", eq.format(), solved);
                }
            }
        }
    }
}
