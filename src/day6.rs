pub(crate) fn ways_to_win(time: i64, record: i64) -> i64 {
    // let x be the time the button is held down
    // then, x in [0, time]
    // time_left = time - x
    // distance = time_left * speed = time_left * x
    // distance = (time - x) * x = time * x - x^2
    // To win, distance > record
    // time * x - x*2 > record
    // x*2 - time * x + record < 0
    // x1 = (time - sqrt(time^2 - 4*record))/2
    // x2 = (time + sqrt(time^2 - 4* record))/2
    // Then we win for x in [ceil(x1), floor(x2)]

    // for proofs, x real, m, n are integers

    // floor((time + root) / 2)
    // => floor((time + floor(root)) / 2) { floor((x + m)/n) = floor((floor(x) + m)/n if n is positive}

    // ceil((time - root) / 2)
    // => ceil((time + ceil(-root))/2) { ceil((x+m)/n) = ceil((ceil(x)+m)/n) }
    // => ceil((time - floor(root))/2) { ceil(-x) = -floor(x) }
    // => (time - floor(root)) - floor((time - floor(root))/2) { n = floor(n/2) + ceil(n/2) }

    let discriminant = time.pow(2) - 4 * record;
    if let Some(root) = discriminant.checked_isqrt() {
        // Since isqrt rounds down, root is floor(root) (root is positive)
        // integer division by 2 is floor for positive numerator (rounds down)
        let lower = time - root - (time - root) / 2;
        let upper = (time + root) / 2;

        let range = upper - lower + 1;

        // If the quadratic roots are integers, we don't include the endpoints
        // That occurs when 1) discriminant is a perfect square
        // and 2) time +/- root is even (due to 2 in denominator)
        // (time + root and time - root have the same parity)
        if root.pow(2) == discriminant && (time + root) % 2 == 0 {
            range - 2
        } else {
            range
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
