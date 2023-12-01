#[cfg(test)]
mod tests {
    use crate::recover_calibration;

    #[test]
    fn simple() {
        let dirty_calibration_data = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        let ans = recover_calibration(dirty_calibration_data);
        assert_eq!(ans, 142);
    }
}
