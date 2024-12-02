#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct LocationID(usize);

pub fn main() {
  let input = include_str!( "day_01.input" );
  let answer = part_one::compute_answer( input );
  println!( "{answer}" );
  let answer = part_two::compute_answer( input );
  println!( "{answer}" );
}

fn parse_input( input: &str ) -> (Vec<LocationID>, Vec<LocationID>) {
  input.lines()
    .map( |line| line.split_whitespace() )
    .map( |mut split| (split.next().unwrap(), split.next().unwrap()) )
    .map( |(left, right)| (left.parse::<usize>().unwrap(), right.parse::<usize>().unwrap()) )
    .map( |(left, right)| (LocationID(left), LocationID(right)) )
    .unzip()
}

// =================================================================================================================================

mod part_one {
  use super::*;

  pub(super) fn compute_answer( input: &str ) -> usize {
    let (left, right) = get_sorted_data( input );
    std::iter::zip( left, right )
      .map( |(left, right)| left.0.abs_diff(right.0) )
      .sum()
  }

  fn get_sorted_data( input: &str ) -> (Vec<LocationID>, Vec<LocationID>) {
    let (mut left, mut right) = parse_input( input );
    left.sort_unstable();
    right.sort_unstable();
    (left, right)
  }

  #[cfg(test)]
  mod tests {
    use super::*;
    use super::super::tests::TEST_INPUT;

    #[test]
    fn test_get_sorted_data() {
      let (left, right) = get_sorted_data( TEST_INPUT );
      println!( "{left:?}" );
      println!( "{right:?}" );
    }

    #[test]
    fn test_compute_answer() {
      let expected = 11;
      let actual = compute_answer( TEST_INPUT );
      assert_eq!( expected, actual );
    }
  }
}

// =================================================================================================================================

mod part_two {
  use super::*;

  pub(super) fn compute_answer( input: &str ) -> usize {
    let (left, right) = parse_input( input );

    let count = |id: &LocationID| {
      right.iter()
        .filter( |other| *other == id )
        .count()
    };

    left.iter()
      .map( |id| id.0 * count(id) )
      .sum()
  }

  #[cfg(test)]
  mod tests {
    use super::*;
    use super::super::tests::TEST_INPUT;

    #[test]
    fn test_compute_answer() {
      let expected = 31;
      let actual = compute_answer( TEST_INPUT );
      assert_eq!( expected, actual );
    }
  }
}

// =================================================================================================================================

#[cfg(test)]
mod tests {
  use super::*;

  pub(super) const TEST_INPUT: &str =
  "3   4
4   3
2   5
1   3
3   9
3   3";

  #[test]
  fn test_parse_input() {
    let (left, right) = parse_input( TEST_INPUT );
    println!( "{left:?}" );
    println!( "{right:?}" );
  }
}
