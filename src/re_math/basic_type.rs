// 2024 (c) MaoHuPi
// RuNeX/src/re_math/basic_type.rs

use std::char::from_digit;
use std::cmp::{max, min, Ordering};
use std::usize;

#[derive(Clone)]
pub struct Integer {
    sign: bool,
    digits: Vec<u8>,
}

pub const BASIC_INT_RADIX: usize = 10;

#[allow(dead_code)]
impl Integer {
    pub fn new(sign: bool, digits: Vec<u8>) -> Self {
        Self {
            sign: sign,     // true: (-), false: (+)
            digits: digits, // [n*1, n*1e1, n*1e2, n*1e3, ...]
        }
    }
    pub fn zero() -> Self {
        Self::new(false, vec![0])
    }

    pub fn from_isize(number: isize) -> Self {
        let mut number: isize = number;
        const RADIX: isize = BASIC_INT_RADIX as isize;
        let mut digits: Vec<u8> = Vec::new();
        while number > RADIX {
            let digit: isize = number % RADIX;
            number -= digit;
            number /= RADIX;
            digits.push(digit as u8);
        }
        digits.push(number as u8);
        Self::new(number < 0, digits)
    }
    pub fn from_string(string: String) -> Self {
        let mut sign: bool = false;
        let mut chars: Vec<char> = string.chars().collect();
        if chars[0] == '-' {
            chars = chars[1..].to_vec();
            sign = true;
        }
        let mut digits: Vec<u8> = chars
            .iter()
            .map(|c| c.to_digit(BASIC_INT_RADIX as u32).unwrap() as u8)
            .collect();
        digits.reverse();
        Self::new(sign, digits)
    }
    pub fn to_isize(self: Self) -> isize {
        let digits: Vec<u8> = self.digits.clone();
        digits
            .iter()
            .enumerate()
            .map(|(i, &n)| (n as usize) * BASIC_INT_RADIX.pow(i as u32))
            .sum::<usize>() as isize
            * (if self.sign { -1 } else { 1 })
    }
    pub fn to_string(self: Self) -> String {
        let mut digits: Vec<u8> = self.digits.clone();
        digits.reverse();
        (if self.sign { "-" } else { "" }).to_string()
            + digits
                .iter()
                .map(|&n| from_digit(n as u32, 16).unwrap())
                .collect::<String>()
                .as_str()
    }

    pub fn opposite(n: Self) -> Self {
        Self::new(!n.sign, n.digits)
    }

    fn digits_cmp(a: Self, b: Self) -> Ordering {
        let a_len: usize = a.digits.len();
        let b_len: usize = b.digits.len();
        if a_len > b_len {
            return Ordering::Greater;
        } else if a_len < b_len {
            return Ordering::Less;
        } else {
            for i in (0..a_len).rev() {
                if a.digits[i] > b.digits[i] {
                    return Ordering::Greater;
                } else if a.digits[i] < b.digits[i] {
                    return Ordering::Less;
                }
            }
            return Ordering::Equal;
        }
    }
    pub fn cmp(a: Self, b: Self) -> Ordering {
        if !a.sign && b.sign {
            // a: (+), b: (-)
            return Ordering::Greater;
        } else if a.sign && !b.sign {
            // a: (-), b: (+)
            return Ordering::Less;
        } else if !a.sign {
            // a: (+), b: (+)
            return Self::digits_cmp(a, b);
        } else {
            // a: (-), b: (-)
            return match Self::digits_cmp(a, b) {
                Ordering::Greater => Ordering::Less,
                Ordering::Less => Ordering::Greater,
                Ordering::Equal => Ordering::Equal,
            };
        }
    }

    fn tidy(self: &mut Self) {
        let digits_len: usize = self.digits.len();
        let mut zero_count: usize = 0;
        const RADIX: u8 = BASIC_INT_RADIX as u8;
        for i in 0..digits_len {
            if self.digits[i] == 0 {
                zero_count += 1;
            } else {
                if self.digits[i] >= RADIX {
                    let digit: u8 = self.digits[i];
                    self.digits[i] = digit % RADIX;
                    self.digits[i + 1] += (digit - self.digits[i]) / RADIX;
                }
                zero_count = 0;
            }
        }
        if zero_count > 0 {
            self.digits = self.digits[..(self.digits.len() - zero_count)].to_vec();
        }
        if self.digits.len() == 0 {
            self.sign = false;
            self.digits = vec![0];
        }
    }
    fn digits_add(a: Self, b: Self) -> Self {
        let mut _min_digits_number: usize = a.digits.len();
        let mut _max_digits_number: usize = b.digits.len();
        let mut max_digits_name: char = 'b';
        if _min_digits_number > _max_digits_number {
            let temp: usize = _min_digits_number;
            _min_digits_number = _max_digits_number;
            _max_digits_number = temp;
            max_digits_name = 'a';
        }
        let mut carry: u8 = 0;
        let mut new_digits: Vec<u8> = Vec::new();
        for i in 0.._max_digits_number {
            let mut digit: u8;
            if i < _min_digits_number {
                digit = a.digits[i] + b.digits[i] + carry;
            } else {
                digit = (if max_digits_name == 'a' {
                    a.digits[i]
                } else {
                    b.digits[i]
                }) + carry;
            }
            if digit >= BASIC_INT_RADIX as u8 {
                carry = 1;
                digit = digit - BASIC_INT_RADIX as u8;
            } else {
                carry = 0;
            }
            new_digits.push(digit);
        }
        if carry > 0 {
            new_digits.push(carry);
        }
        Self::new(false, new_digits)
    }
    fn digits_sub(big: Self, small: Self) -> Self {
        let _max_digits_number: usize = big.digits.len();
        let _min_digits_number: usize = small.digits.len();

        let mut carry: u8 = 0;
        let mut new_digits: Vec<u8> = Vec::new();
        let mut zero_count: usize = 0;
        for i in 0.._max_digits_number {
            let mut digit: i8;
            if i < _min_digits_number {
                carry += small.digits[i]
            }
            digit = big.digits[i] as i8;
            digit -= carry as i8;
            if digit < 0 {
                carry = 1;
                digit += BASIC_INT_RADIX as i8;
            } else {
                carry = 0;
            }
            if digit == 0 {
                zero_count += 1;
            } else {
                zero_count = 0;
            }
            new_digits.push(digit as u8);
        }
        if zero_count > 0 {
            new_digits = new_digits[..(new_digits.len() - zero_count)].to_vec();
        }
        Self::new(false, new_digits)
    }
    pub fn add(a: Self, b: Self) -> Self {
        let a_sign: bool = a.sign;
        let b_sign: bool = b.sign;
        if a_sign == b_sign {
            let mut digits_result: Self = Self::digits_add(a, b);
            digits_result.sign = a_sign;
            return digits_result;
        } else {
            let new_sign: bool;
            let big: Self;
            let small: Self;
            match Self::digits_cmp(a.clone(), b.clone()) {
                Ordering::Greater => {
                    new_sign = a_sign;
                    big = a;
                    small = b;
                }
                Ordering::Less => {
                    new_sign = b_sign;
                    big = b;
                    small = a;
                }
                Ordering::Equal => {
                    return Self::zero();
                }
            }
            let mut digits_result: Self = Self::digits_sub(big, small);
            digits_result.sign = new_sign;
            return digits_result;
        }
    }
    pub fn sub(a: Self, b: Self) -> Self {
        return Self::add(a, Self::opposite(b));
    }

    pub fn high_shift(self: &mut Self, times: u32) {
        let mut new_digits = vec![0; times as usize];
        new_digits.append(&mut self.digits.clone());
        self.digits = new_digits;
    }
    pub fn low_shift(self: &mut Self, times: u32) {
        self.digits = self.digits.clone()[(times as usize)..].to_vec();
    }

    fn digits_mul(a: Self, b: Self, level: u32) -> Self {
        // ref: PanSci 泛科學 - [地表最速乘法傳說！碰到大得要命的數字，這是最快的乘法方式](https://pansci.asia/archives/162365)
        let mut _max_digits_number: usize = a.digits.len();
        let mut _min_digits_number: usize = b.digits.len();
        if _max_digits_number == 0
            || _min_digits_number == 0
            || (_max_digits_number == 1 && a.digits[0] == 0)
            || (_min_digits_number == 1 && b.digits[0] == 0)
        {
            return Self::zero();
        }
        if _min_digits_number > _max_digits_number {
            let temp: usize = _min_digits_number;
            _min_digits_number = _max_digits_number;
            _max_digits_number = temp;
        }
        let mut level: u32 = level;
        if level == 0 {
            level = ((_max_digits_number as f64).log10() / 2_f64.log10()).ceil() as u32;
            if level > 0 {
                level -= 1;
            }
        }
        let group_width = 2_u32.pow(level as u32);
        if level == 0 {
            let a0: u8 = a.digits[0];
            let a1: u8 = if a.digits.len() > 1 { a.digits[1] } else { 0 };
            let b0: u8 = b.digits[0];
            let b1: u8 = if b.digits.len() > 1 { b.digits[1] } else { 0 };

            let r1: u8 = a0 * b0;
            let r4: u8 = a1 * b1;
            let r23: u16 = ((a0 + a1) as u16) * ((b0 + b1) as u16) - r1 as u16 - r4 as u16;

            let radix: u8 = BASIC_INT_RADIX as u8;
            let mut messy_result: Self = Self::new(
                false,
                vec![
                    r1 % radix,
                    r1 / radix + (r23 % (radix as u16)) as u8,
                    (r23 / (radix as u16)) as u8 + r4 % radix,
                    r4 / radix,
                ],
            );
            messy_result.tidy();
            return messy_result;
        }
        // n11 n12
        // n21 n22
        let a_group_width: usize = min(group_width as usize, a.digits.len());
        let b_group_width: usize = min(group_width as usize, b.digits.len());
        let n12: Self = Self::new(false, a.digits[..a_group_width].to_vec());
        let n22: Self = Self::new(false, b.digits[..b_group_width].to_vec());
        let n11: Self = Self::new(false, a.digits[a_group_width..].to_vec());
        let n21: Self = Self::new(false, b.digits[b_group_width..].to_vec());
        let r1 = Self::digits_mul(n12.clone(), n22.clone(), level - 1);
        let mut r4 = Self::digits_mul(n11.clone(), n21.clone(), level - 1);
        let mut r23 = Self::digits_sub(
            Self::digits_mul(
                Self::digits_add(n11, n12),
                Self::digits_add(n21, n22),
                level - 1,
            ),
            Self::digits_add(r1.clone(), r4.clone()),
        );
        r23.high_shift(group_width);
        r4.high_shift(group_width * 2);
        Self::digits_add(Self::digits_add(r1, r23), r4)
    }
    pub fn mul(a: Self, b: Self) -> Self {
        let new_sign: bool = a.sign != b.sign; // ++ => +, -- => +, +- => -, -+ => -
        let mut digits_result: Self = Self::digits_mul(a, b, 0);
        digits_result.sign = new_sign;
        return digits_result;
    }
}

/* Float */

#[derive(Clone)]
pub struct Float {
    point: Integer,
    value: Integer,
}

#[allow(dead_code)]
impl Float {
    pub fn new(point: Integer, value: Integer) -> Self {
        Self {
            point: point,
            value: value,
        }
    }
    pub fn zero() -> Self {
        Self::new(Integer::zero(), Integer::zero())
    }

    pub fn from_f64(number: f64) -> Self {
        let number_string_list: Vec<String> = number
            .abs()
            .to_string()
            .split('.')
            .map(|s| (&s).to_string())
            .collect::<Vec<String>>();
        let integer: usize = number_string_list[0].parse::<usize>().unwrap() as usize;
        let float: f64 = if number_string_list.len() > 1 {
            number_string_list[1].parse::<usize>().unwrap()
        } else {
            0
        } as f64
            / 10.0;

        let radix: usize = BASIC_INT_RADIX as usize;
        let mut integer: usize = integer;
        let mut digits_integer: Vec<u8> = Vec::new();
        while integer > radix {
            let digit: usize = integer % radix;
            integer -= digit;
            integer /= radix;
            digits_integer.push(digit as u8);
        }
        digits_integer.push(integer as u8);

        let radix: f64 = BASIC_INT_RADIX as f64;
        let mut float: f64 = float;
        let mut digits_float: Vec<u8> = Vec::new();
        let mut digit_value_now: f64 = 0.0;
        while float >= digit_value_now {
            if digit_value_now != 0.0 {
                // println!("{}, {}", float, digit_value_now);
                let digit: f64 = (float / digit_value_now).floor();
                float -= digit * digit_value_now;
                digits_float.push(digit as u8);
            }
            digit_value_now = 1.0 / (radix.powf(digits_float.len() as f64 + 1.0));
        }
        digits_float.push(float as u8);
        digits_float.reverse();

        Self::new(
            Integer::from_isize(digits_float.len() as isize),
            Integer::new(number < 0.0, vec![digits_float, digits_integer].concat()),
        )
    }
    pub fn from_string(string: String) -> Self {
        let mut sign: bool = false;
        let mut chars: Vec<char> = string.chars().collect();
        if chars[0] == '-' {
            chars = chars[1..].to_vec();
            sign = true;
        }
        let mut dot_pos: usize = match chars.iter().position(|&c| c == '.') {
            Some(p) => p,
            None => chars.len(),
        };
        if dot_pos < chars.len() && chars[dot_pos] == '.' {
            chars.remove(dot_pos);
        }
        dot_pos = chars.len() - dot_pos;
        let mut digits: Vec<u8> = chars
            .iter()
            .map(|c| c.to_digit(BASIC_INT_RADIX as u32).unwrap() as u8)
            .collect();
        digits.reverse();
        Self::new(
            Integer::from_isize(dot_pos as isize),
            Integer::new(sign, digits),
        )
    }
    pub fn from_integer(n: Integer) -> Self {
        Self::new(Integer::zero(), n)
    }
    pub fn to_f64(self: Self) -> f64 {
        // println!("{:?}", self.value.digits);
        let digits: Vec<u8> = self.value.digits.clone();
        digits
            .iter()
            .enumerate()
            .map(|(i, &n)| {
                (n as f64)
                    * (BASIC_INT_RADIX as f64)
                        .powf(i as f64 - Integer::to_isize(self.point.clone()) as f64)
            })
            .sum::<f64>()
            * (if self.value.sign { -1.0 } else { 1.0 })
    }
    pub fn to_string(self: Self) -> String {
        let mut digits_vec_char: Vec<char> = self
            .value
            .digits
            .clone()
            .iter()
            .map(|&n| from_digit(n as u32, 16).unwrap())
            .collect::<Vec<char>>();
        let mut digits_string: String;
        let point: isize = Integer::to_isize(self.point.clone());
        let digits_len: isize = digits_vec_char.len() as isize;
        let dot_pos: usize;
        if point > digits_len {
            digits_vec_char = vec![
                digits_vec_char,
                vec!['0'; max(point - digits_len, 0) as usize],
            ]
            .concat();
            dot_pos = digits_vec_char.len();
        } else if point < 0 {
            digits_vec_char =
                vec![vec!['0'; min(point, 0).abs() as usize], digits_vec_char].concat();
            dot_pos = 0;
        } else {
            dot_pos = point as usize;
        }
        digits_vec_char.reverse();
        digits_string = digits_vec_char.iter().collect();
        digits_string.insert(digits_vec_char.len() - dot_pos, '.');
        (if self.value.sign { "-" } else { "" }).to_string() + digits_string.as_str()
    }

    pub fn opposite(n: Self) -> Self {
        Self::new(n.point, Integer::opposite(n.value))
    }

    fn digits_cmp(a: Self, b: Self) -> Ordering {
        let a_len: Integer =
            Integer::sub(Integer::from_isize(a.value.digits.len() as isize), a.point);
        let b_len: Integer =
            Integer::sub(Integer::from_isize(b.value.digits.len() as isize), b.point);
        match Integer::cmp(a_len, b_len) {
            Ordering::Greater => Ordering::Greater,
            Ordering::Less => Ordering::Less,
            Ordering::Equal => Integer::digits_cmp(a.value, b.value),
        }
    }
    pub fn cmp(a: Self, b: Self) -> Ordering {
        if !a.value.sign && b.value.sign {
            // a: (+), b: (-)
            return Ordering::Greater;
        } else if a.value.sign && !b.value.sign {
            // a: (-), b: (+)
            return Ordering::Less;
        } else if !a.value.sign {
            // a: (+), b: (+)
            return Self::digits_cmp(a, b);
        } else {
            // a: (-), b: (-)
            return match Self::digits_cmp(a, b) {
                Ordering::Greater => Ordering::Less,
                Ordering::Less => Ordering::Greater,
                Ordering::Equal => Ordering::Equal,
            };
        }
    }

    fn tidy(self: &mut Self) {
        let digits_len: usize = self.value.digits.len();
        let mut zero_count: usize = 0;
        for i in 0..digits_len {
            if self.value.digits[i] == 0 {
                zero_count += 1;
            } else {
                break;
            }
        }
        if zero_count > 0 {
            self.value.low_shift(zero_count as u32);
            self.point = Integer::sub(self.point.clone(), Integer::from_isize(zero_count as isize));
        }
        self.value.tidy();
    }
    pub fn add(a: Self, b: Self) -> Self {
        let mut min_float_len: Integer = a.point;
        let mut max_float_len: Integer = b.point;
        let mut min_float_name: char = 'a';
        if Integer::cmp(min_float_len.clone(), max_float_len.clone()) == Ordering::Greater {
            let temp: Integer = min_float_len;
            min_float_len = max_float_len;
            max_float_len = temp;
            min_float_name = 'b';
        }
        let mut min_float_value = if min_float_name == 'a' {
            a.value.clone()
        } else {
            b.value.clone()
        };
        min_float_value
            .high_shift(
                Integer::to_isize(Integer::digits_sub(max_float_len.clone(), min_float_len)) as u32,
            );
        Self::new(
            max_float_len,
            Integer::add(
                min_float_value,
                if min_float_name == 'a' {
                    b.value.clone()
                } else {
                    a.value.clone()
                },
            ),
        )
    }
    pub fn sub(a: Self, b: Self) -> Self {
        return Self::add(a, Self::opposite(b));
    }

    fn high_shift(self: &mut Self, times: u32) {
        self.point = Integer::sub(self.point.clone(), Integer::from_isize(times as isize));
    }
    fn low_shift(self: &mut Self, times: u32) {
        self.point = Integer::add(self.point.clone(), Integer::from_isize(times as isize));
    }

    pub fn mul(a: Self, b: Self) -> Self {
        Self::new(
            Integer::add(a.point, b.point),
            Integer::mul(a.value, b.value),
        )
    }
}

/* Symbol */

// #[derive(Clone)]
// pub struct Symbol {
//     redefined: bool,
//     symbol: String,
// }

// impl Symbol {
//     pub fn new(redefined: bool, symbol: String) -> Self {
//         Self {
//             redefined: redefined,
//             symbol: symbol,
//         }
//     }
//     fn get_value(self: Self) -> Integer {}
// }
