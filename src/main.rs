mod re_math;
use crate::re_math::advanced_type::{Addition, Formula, FormulaTypes, Negative};
#[allow(unused_imports)]
use crate::re_math::basic_type::{Float, Integer};

fn main() {
    // println!("Hello, world!");
    // let mut t: Float = Float::from_f64(0.0);
    // for _ in 1..=10 {
    //     t = Float::add(t, Float::from_f64(0.1));
    // }
    // println!("{}", t.clone().to_f64());

    // println!(
    //     "{}",
    //     Float::add(Float::from_f64(0.1), Float::from_f64(0.2)).to_f64()
    // );
    // println!("{}", Float::from_f64(0.1).to_f64());
    // println!("{}", Float::from_f64(0.2).to_f64());
    // println!(
    //     "{}",
    //     Float::mul(Float::from_f64(0.3), Float::from_f64(2.0)).to_f64()
    // );

    let mut formula: FormulaTypes = FormulaTypes::Addition(Addition::new(vec![
        Box::new(FormulaTypes::Integer(Integer::from_isize(10))),
        Box::new(FormulaTypes::Negative(Negative::new(Box::new(
            FormulaTypes::Integer(Integer::from_isize(15)),
        )))),
    ]));
    formula = *formula.simplify();
    println!("{}", formula.calculate().to_f64());
}