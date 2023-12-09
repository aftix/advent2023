use super::parse_int;
use crate::types::{Schematic, Span};
use nom::{
    character::complete::{char, none_of},
    error::{Error, ErrorKind},
    multi::many0,
    Err as nErr, IResult,
};
use rayon::prelude::*;

pub fn parse_periods(input: &str) -> IResult<&str, usize> {
    let (rest, periods) = many0(char('.'))(input)?;
    Ok((rest, periods.len()))
}

pub fn parse_number(input: &str) -> IResult<&str, (usize, i64)> {
    let (rest, number) = parse_int(input)?;

    if number == 0 {
        return Err(nErr::Failure(Error::new(rest, ErrorKind::Digit)));
    }

    Ok((rest, (number.ilog10() as usize + 1, number)))
}

pub fn parse_symbol(input: &str) -> IResult<&str, char> {
    none_of(".1234567890")(input)
}

pub fn parse_item(input: &str) -> IResult<&str, Schematic> {
    let (rest, offset) = parse_periods(input)?;

    match parse_symbol(rest) {
        Ok((rest, ch)) => Ok((rest, Schematic::Symbol(ch, Span(offset, offset + 1)))),
        Err(nErr::Error(_)) => parse_number(rest)
            .map(|(rest, (len, num))| (rest, Schematic::Number(num, Span(offset, offset + len)))),
        Err(err) => Err(err),
    }
}

pub fn parse_line(input: &str) -> IResult<&str, Vec<Schematic>> {
    let (rest, mut items) = many0(parse_item)(input)?;

    // Adjust the span on each item to show proper place in line
    let mut accumulator = 0;
    items.iter_mut().for_each(|item| match item {
        Schematic::Number(_, ref mut span) | Schematic::Symbol(_, ref mut span) => {
            // The exclusive end of the item span gives the total number of chars
            // that parse_item processed
            let addon = span.1;
            span.0 += accumulator;
            span.1 += accumulator;
            accumulator += addon;
        }
    });

    Ok((rest, items))
}

pub fn parse_input(lines: &[&str]) -> IResult<(), ((usize, usize), Vec<Schematic>)> {
    let height = lines.len();
    if height == 0 {
        return Ok(((), ((0, 0), vec![])));
    }
    let width = lines[0].chars().count();

    let lines: Vec<_> = lines
        .par_iter()
        .map(|line| parse_line(line).map(|(_, vec)| vec))
        .collect();

    // Return error if any of the lines had an error
    let mut lines: Vec<Vec<_>> = lines
        .into_iter()
        .try_collect()
        .map_err(|err| err.map(|err| Error::new((), err.code)))?;

    // Adjust each span's line to be given the point
    lines.par_iter_mut().enumerate().for_each(|(y, line)| {
        line.iter_mut().for_each(|item| match item {
            Schematic::Number(_, ref mut span) | Schematic::Symbol(_, ref mut span) => {
                span.0 += width * y;
                span.1 += width * y;
            }
        });
    });

    Ok(((), ((width, height), lines.into_iter().flatten().collect())))
}

#[cfg(test)]
mod test {
    use crate::types::{Schematic, Span};
    use test_case::test_case;

    #[test_case("" => 0 ; "when empty")]
    #[test_case(" " => 0 ; "when not periods")]
    #[test_case("." => 1)]
    #[test_case(".." => 2)]
    #[test_case(".........." => 10)]
    fn periods(input: &str) -> usize {
        super::parse_periods(input).unwrap().1
    }

    #[test_case("1" => (1, 1))]
    #[test_case("5" => (1, 5))]
    #[test_case("10" => (2, 10))]
    #[test_case("156" => (3, 156))]
    fn number(input: &str) -> (usize, i64) {
        super::parse_number(input).unwrap().1
    }

    #[test_case("" ; "when empty")]
    #[should_panic]
    fn number_panics(input: &str) {
        super::parse_number(input).unwrap();
    }

    #[test_case("#" => '#' ; "octothorpe")]
    #[test_case("#$" => '#' ; "when trailing symbols")]
    #[test_case("^" => '^' ; "caret")]
    #[test_case("*" => '*' ; "asterisk")]
    #[test_case(" " => ' ' ; "space")]
    fn symbol(input: &str) -> char {
        super::parse_symbol(input).unwrap().1
    }

    #[test_case("" ; "when empty")]
    #[test_case("." ; "when period")]
    #[test_case("2" ; "when digit")]
    #[should_panic]
    fn symbol_panics(input: &str) {
        super::parse_symbol(input).unwrap();
    }

    #[test_case("1" => Schematic::Number(1, Span(0, 1)))]
    #[test_case("10" => Schematic::Number(10, Span(0, 2)))]
    #[test_case(".1" => Schematic::Number(1, Span(1, 2)))]
    #[test_case(".10" => Schematic::Number(10, Span(1, 3)))]
    #[test_case(".....1" => Schematic::Number(1, Span(5, 6)))]
    #[test_case(".....10" => Schematic::Number(10, Span(5, 7)))]
    #[test_case("#" => Schematic::Symbol('#', Span(0, 1)))]
    #[test_case(".#" => Schematic::Symbol('#', Span(1, 2)))]
    #[test_case(".....#" => Schematic::Symbol('#', Span(5, 6)))]
    fn item(input: &str) -> Schematic {
        super::parse_item(input).unwrap().1
    }

    #[test_case("" ; "when empty")]
    #[test_case("." ; "when period")]
    #[should_panic]
    fn item_panics(input: &str) {
        super::parse_item(input).unwrap();
    }

    #[test_case("1" => vec![Schematic::Number(1, Span(0, 1))])]
    #[test_case("#" => vec![Schematic::Symbol('#', Span(0, 1))])]
    #[test_case("#1" => vec![Schematic::Symbol('#', Span(0, 1)), Schematic::Number(1, Span(1, 2))])]
    #[test_case("1#" => vec![Schematic::Number(1, Span(0, 1)), Schematic::Symbol('#', Span(1, 2))])]
    #[test_case("#12" => vec![Schematic::Symbol('#', Span(0, 1)), Schematic::Number(12, Span(1, 3))])]
    #[test_case("12#" => vec![Schematic::Number(12, Span(0, 2)), Schematic::Symbol('#', Span(2, 3))])]
    #[test_case(".1" => vec![Schematic::Number(1, Span(1, 2))])]
    #[test_case(".#" => vec![Schematic::Symbol('#', Span(1, 2))])]
    #[test_case("1.#" => vec![Schematic::Number(1, Span(0, 1)), Schematic::Symbol('#', Span(2, 3))])]
    #[test_case("12.#" => vec![Schematic::Number(12, Span(0, 2)), Schematic::Symbol('#', Span(3, 4))])]
    #[test_case("#.12" => vec![Schematic::Symbol('#', Span(0, 1)), Schematic::Number(12, Span(2, 4))])]
    #[test_case("12.....12" => vec![Schematic::Number(12, Span(0, 2)), Schematic::Number(12, Span(7, 9))])]
    fn line(input: &str) -> Vec<Schematic> {
        super::parse_line(input).unwrap().1
    }
}
