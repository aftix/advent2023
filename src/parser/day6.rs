use super::parse_int;
use nom::{bytes::complete::tag, character::complete::space0, multi::many1, sequence::preceded};

pub fn parse_line(input: &[&str]) -> (Vec<i64>, Vec<i64>) {
    let (rest, _) = tag::<_, _, nom::error::Error<&str>>("Time:")(input[0]).unwrap();
    let (_, times) =
        many1::<_, _, nom::error::Error<&str>, _>(preceded(space0, parse_int))(rest).unwrap();

    let (rest, _) = tag::<_, _, nom::error::Error<&str>>("Distance:")(input[1]).unwrap();
    let (_, records) =
        many1::<_, _, nom::error::Error<&str>, _>(preceded(space0, parse_int))(rest).unwrap();

    (times, records)
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    #[test_case(&["Time:      7  15   30", "Distance:  9  40  200"] => (vec![7, 15, 30], vec![9, 40, 200]))]
    fn parse_line(input: &[&str]) -> (Vec<i64>, Vec<i64>) {
        super::parse_line(input)
    }
}
