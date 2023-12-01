pub mod test;

pub fn recover_calibration(dirty_data: &str) -> u32 {
    dirty_data.split("\n").fold(0, |acc, line| {

        let digits = line.chars().fold((None, None), |mut acc, char| {
            if let Some(digit) = char.to_digit(10) {
                if acc.0.is_none() {
                    acc.0 = Some(digit);
                }
                acc.1 = Some(digit);
            }
            acc
        });
        acc + digits.0.unwrap()*10 + digits.1.unwrap()
    })
}
