pub fn parse_line(input: &str) -> i64 {
    let zero = 48;
    let nine = 57;

    let mut number = 0;

    for ch in input.chars() {
        let digit: u8 = unsafe { ch.try_into().unwrap_unchecked() };
        if digit < zero || digit > nine {
            continue;
        }
        number = (digit - zero) as i64;
        break;
    }

    for ch in input.chars().rev() {
        let digit: u8 = unsafe { ch.try_into().unwrap_unchecked() };
        if digit < zero || digit > nine {
            continue;
        }
        number *= 10;
        number += (digit - zero) as i64;
        break;
    }

    number
}
