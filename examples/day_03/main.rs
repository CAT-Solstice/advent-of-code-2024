
pub fn main() {
  let input = include_str!( "day_03.input" );
  let answer = part_one::compute_answer( input );
  println!( "{answer}" );
  let answer = part_two::compute_answer( input );
  println!( "{answer}" );
}

mod part_one {
  pub(super) fn compute_answer( input: &str ) -> usize {
    let regex = regex::Regex::new( r"mul\((\d{1,3}),(\d{1,3})\)" ).unwrap();
    regex.captures_iter( input )
      .map( |capture| capture.extract() )
      .map( |(_, [x,y])| x.parse::<usize>().unwrap() * y.parse::<usize>().unwrap() )
      .sum()
  }

  #[cfg(test)]
  mod tests {
    use super::*;

    const TEST_INPUT: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    #[test]
    fn test_compute_answer() {
      let expected = 161;
      let actual = compute_answer( TEST_INPUT );
      assert_eq!( expected, actual );
    }
  }
}

mod part_two {
  enum State {
    Enabled,
    Disabled,
  }

  pub(super) fn compute_answer( input: &str ) -> usize {
    let regex = regex::Regex::new( r"(do\(\))|(don't\(\))|mul\((\d{1,3}),(\d{1,3})\)" ).unwrap();
    let mut state = State::Enabled;
    let mut sum = 0;
    for c in regex.captures_iter( input ) {
      match (c.get(1), c.get(2), c.get(3), c.get(4) ) {
        (Some(_do), None, None, None) => state = State::Enabled,
        (None, Some(_dont), None, None) => state = State::Disabled,
        (None, None, Some(x), Some(y)) if matches!(state,State::Enabled) =>
          sum += x.as_str().parse::<usize>().unwrap() * y.as_str().parse::<usize>().unwrap(),
        _ => {},
      }
    }
    sum
  }

  #[cfg(test)]
  mod tests {
    use super::*;

    const TEST_INPUT: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_compute_answer() {
      let expected = 48;
      let actual = compute_answer( TEST_INPUT );
      assert_eq!( expected, actual );
    }
  }
}
