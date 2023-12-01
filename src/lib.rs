use std::ops::{AddAssign, MulAssign};

pub mod test;

static DIGITS_SPELLED: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

pub fn recover_calibration(dirty_data: &str) -> u32 {
    dirty_data.split("\n").fold(0, |acc, line| {
        let mut streak = [0; 10];
        let digits = line.chars().fold((None, None), |mut acc, char| {
            let str_digit = DIGITS_SPELLED
                .iter()
                .zip(streak.iter_mut())
                .enumerate()
                .fold(None, |acc, digit| {
                    if digit.1 .0.chars().nth(digit.1 .1.clone()).unwrap() == char {
                        digit.1 .1.add_assign(1);
                    } else {
                        digit.1 .1.mul_assign(0);
                        if digit.1 .0.chars().nth(digit.1 .1.clone()).unwrap() == char {
                            digit.1 .1.add_assign(1);
                        } else {
                            digit.1 .1.mul_assign(0);
                        }

                    }
                    if digit.1 .1.clone() == digit.1 .0.len()  {
                        digit.1 .1.mul_assign(0);
                        return Some(digit.0);
                    }
                    acc
                });
            if char.is_digit(10) || str_digit.is_some() {
                let digit = if char.is_digit(10) {
                    char.to_digit(10).unwrap()
                } else {
                    str_digit.unwrap() as u32
                };
                if acc.0.is_none() {
                    acc.0 = Some(digit);
                }
                acc.1 = Some(digit);
            };

            acc
        });
        //println!("{}{}", digits.0.unwrap(), digits.1.unwrap());
        acc + digits.0.unwrap() * 10 + digits.1.unwrap()
    })
}
