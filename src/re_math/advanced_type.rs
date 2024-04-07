use crate::re_math::basic_type::Float;
use crate::re_math::basic_type::Integer;

#[derive(Clone)]
pub enum FormulaTypes {
    Integer(Integer),
    Float(Float),
    Addition(Addition),
    Negative(Negative),
}
impl Formula for FormulaTypes {
    fn simplify(self) -> Box<FormulaTypes> {
        match self {
            Self::Integer(n) => Box::new(Self::Integer(n)),
            Self::Float(n) => Box::new(Self::Float(n)),
            Self::Addition(f) => f.simplify(),
            Self::Negative(f) => f.simplify(),
        }
    }
    fn calculate(self) -> Float {
        match self {
            Self::Integer(n) => Float::from_integer(n),
            Self::Float(n) => n,
            Self::Addition(f) => f.calculate(),
            Self::Negative(f) => f.calculate(),
        }
    }
}
pub trait Formula {
    fn simplify(self: Self) -> Box<FormulaTypes>;
    fn calculate(self: Self) -> Float;
}

#[derive(Clone)]
pub struct Addition {
    elements: Vec<Box<FormulaTypes>>,
}
impl Addition {
    pub fn new(elements: Vec<Box<FormulaTypes>>) -> Self {
        Self { elements: elements }
    }
}
impl Formula for Addition {
    fn simplify(mut self: Self) -> Box<FormulaTypes> {
        let mut delta_position: isize = 0;
        let mut position_update_list: Vec<usize> = Vec::new();
        let mut item_update_list: Vec<Vec<Box<FormulaTypes>>> = Vec::new();
        let mut pure_integer: Integer = Integer::zero();
        let mut pure_float: Float = Float::zero();
        for i in 0..(self.elements.len()) {
            let element: Box<FormulaTypes> = self.elements[i].clone();
            match (*element).clone() {
                FormulaTypes::Integer(n) => {
                    if !n.clone().is_zero() {
                        pure_integer = Integer::add(pure_integer, n);
                    }
                    position_update_list.push((i as isize + delta_position) as usize);
                    item_update_list.push(vec![]);
                    delta_position -= 1;
                }
                FormulaTypes::Float(n) => {
                    if !n.clone().is_zero() {
                        pure_float = Float::add(pure_float, n);
                    }
                    position_update_list.push((i as isize + delta_position) as usize);
                    item_update_list.push(vec![]);
                    delta_position -= 1;
                }
                FormulaTypes::Addition(f) => {
                    position_update_list.push((i as isize + delta_position) as usize);
                    delta_position += f.elements.len() as isize - 1;
                    item_update_list.push(f.elements);
                }
                _ => {}
            }
        }
        for i in 0..(position_update_list.len()) {
            self.elements.splice(
                position_update_list[i]..(position_update_list[i] + 1),
                item_update_list[i].iter().cloned(),
            );
        }
        let pure_iis0 = pure_integer.clone().is_zero();
        let pure_fis0 = pure_float.clone().is_zero();
        if !pure_iis0 && !pure_fis0 {
            self.elements.push(Box::new(FormulaTypes::Float(Float::add(
                Float::from_integer(pure_integer),
                pure_float,
            ))));
        } else if !pure_iis0 {
            self.elements
                .push(Box::new(FormulaTypes::Integer(pure_integer)));
        } else if !pure_fis0 {
            self.elements
                .push(Box::new(FormulaTypes::Float(pure_float)));
        }
        return Box::new(FormulaTypes::Addition(self.clone()));
    }
    fn calculate(self: Self) -> Float {
        let mut sum: Float = Float::zero();
        for i in 0..(self.elements.len()) {
            let element: Box<FormulaTypes> = self.elements[i].clone();
            sum = Float::add(sum, (*element).clone().calculate());
        }
        sum
    }
}

#[derive(Clone)]
pub struct Negative {
    element: Box<FormulaTypes>,
}
impl Negative {
    pub fn new(element: Box<FormulaTypes>) -> Self {
        Self { element: element }
    }
}
impl Formula for Negative {
    fn simplify(self: Self) -> Box<FormulaTypes> {
        match (*self.element).clone() {
            FormulaTypes::Integer(n) => {
                return Box::new(FormulaTypes::Integer(Integer::opposite(n)));
            }
            FormulaTypes::Float(n) => {
                return Box::new(FormulaTypes::Float(Float::opposite(n)));
            }
            FormulaTypes::Negative(f) => {
                return f.element.clone();
            }
            FormulaTypes::Addition(f) => {
                return Box::new(FormulaTypes::Addition(Addition::new(
                    f.elements
                        .iter()
                        .map(|element| {
                            let element = Negative::new(element.clone());
                            return element.simplify();
                        })
                        .collect::<Vec<Box<FormulaTypes>>>(),
                )));
            }
            _ => return Box::new(FormulaTypes::Negative(self.clone())),
        }
    }
    fn calculate(self: Self) -> Float {
        Float::opposite((*self.element).calculate())
    }
}
