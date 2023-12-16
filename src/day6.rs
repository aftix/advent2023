pub(crate) fn ways_to_win(time: i64, record: i64) -> i64 {
    // let x be the time the button is held down
    // then, x in [0, time]
    // time_left = time - x
    // distance = time_left * speed = time_left * x
    // distance = (time - x) * x = time * x - x^2
    // To win, distance > record
    // time * x - x*2 > record
    // x*2 - time * x + record < 0
    // det of this is (-time)^2 - 4(1)(record) = time^2 - 4*record
    // x1 = (time - sqrt(time^2 - 4*record))/2
    // x2 = (time + sqrt(time^2 - 4* record))/2
    // Then we win for x in [ceil(x1), floor(x2)]

    let time_f = time as f64;
    let record_f = record as f64;
    let det: f64 = time_f.powi(2) - 4.0 * record_f;
    let root = det.sqrt();

    if !root.is_nan() {
        let lower = (time_f - root) * 0.5;
        let upper = (time_f + root) * 0.5;

        // Check the boundaries (if det is a perfect square, then lower or upper could be as well)
        if (root as i64).pow(2) == det as i64 {
            // lower and upper are integers iff time +/- root is even
            // So lower and upper are both integers or neither are
            if (time + root as i64) % 2 == 0 {
                upper.floor() as i64 - lower.ceil() as i64 - 1
            } else {
                upper.floor() as i64 - lower.ceil() as i64 + 1
            }
        } else {
            upper.floor() as i64 - lower.ceil() as i64 + 1
        }
    } else {
        0
    }
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    #[test_case(7, 9 => 4)]
    #[test_case(30, 200 => 9)]
    #[test_case(71530, 940200 => 71503)]
    fn ways_to_win(t: i64, d: i64) -> i64 {
        super::ways_to_win(t, d)
    }
}
