use std::time;

use aoc2023d1;
fn main() {
    let data = "oneone
twotwo
threethree
fourfour
fivefive
sixsix
sevenseven
eighteight
ninenine";
    let start = time::Instant::now();
    let ans = aoc2023d1::recover_calibration(data);
    let end = time::Instant::now();
    let diff = end - start;
    println!("{}\n{:?}", ans,diff);

}
