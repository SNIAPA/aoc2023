#[cfg(test)]
mod tests {
    use crate::recover_calibration;

    #[test]
    fn simple() {
        let dirty_calibration_data = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        let ans = recover_calibration(dirty_calibration_data);
        assert_eq!(ans, 281);
    }
    #[test]
    fn count() {
        let dirty_calibration_data = "oneone
twotwo
threethree
fourfour
fivefive
sixsix
sevenseven
eighteight
ninenine";
        let ans = recover_calibration(dirty_calibration_data);
        assert_eq!(ans, 11 + 22 + 33 + 44 + 55 + 66 + 77 + 88 + 99);
    }
    #[test]
    fn count_bait() {
        let dirty_calibration_data = "one2one
two1two
three2three
four2four
five2five
six2six
seven2seven
eight2eight
nine2nine";
        let ans = recover_calibration(dirty_calibration_data);
        assert_eq!(ans, 11 + 22 + 33 + 44 + 55 + 66 + 77 + 88 + 99);
    }
    #[test]

    fn count_double_bait() {
        let dirty_calibration_data = "one2one1
two1two2
three2three3
four2four4
five2five5
six2six6
seven2seven7
eight2eight8
nine2nine9";
        let ans = recover_calibration(dirty_calibration_data);
        assert_eq!(ans, 11 + 22 + 33 + 44 + 55 + 66 + 77 + 88 + 99);
    }

    #[test]
    fn edge() {
        let dirty_calibration_data = "mneightzzqvdm14";
        let ans = recover_calibration(dirty_calibration_data);
        assert_eq!(ans, 84);
    }
    #[test]
    fn edge_2() {

        let dirty_calibration_data = "mzdttqmfpssevensixeight3twoone4";
        let ans = recover_calibration(dirty_calibration_data);
        assert_eq!(ans, 74);
    }
}
