
pub fn main() {
  let input = include_str!( "day_03.input" );
  let answer = part_one::compute_answer( input );
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
    use super::super::tests::TEST_INPUT;

    #[test]
    fn test_compute_answer() {
      let expected = 161;
      let actual = super::compute_answer( TEST_INPUT );
      assert_eq!( expected, actual );
    }
  }
}

mod part_two {

  #[cfg(test)]
  mod tests {
    use super::*;

  }
}


#[cfg(test)]
mod tests {
  use super::*;

  pub(super) const TEST_INPUT: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
}
