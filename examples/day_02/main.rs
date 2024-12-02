use std::str::FromStr;
use itertools::Itertools;

type Level = i32;

#[derive(Debug)]
struct Report {
  levels: Vec<Level>,
}

impl FromStr for Report {
  type Err = <Level as FromStr>::Err;

  fn from_str( input: &str ) -> Result<Self, Self::Err> {
    let levels = input.split_whitespace()
      .map( str::parse::<Level> )
      .collect::<Result<_,_>>()?;
    let this = Self { levels };
    Ok( this )
  }
}

impl Report {
  fn is_safe( &self ) -> bool {
    let diff = self.levels.iter()
      .tuple_windows()
      .map( |(prev, next)| next - prev )
      .collect::<Vec<_>>();
    let is_safe = diff.iter().all( |diff| (1..=3).contains(diff) );
    is_safe | diff.iter().all( |diff| (-3..=-1).contains(diff) )
  }

  fn is_safe_dampened( &self ) -> Result<Option<usize>, ()> {
    if self.is_safe() {
      Ok(None)
    }
    else {
      // brute force ¯\_(ツ)_/¯
      for nth in 0..self.levels.len() {
        if self.is_safe_skip( nth ) {
          return Ok(Some(nth));
        }
      }
      Err(())
    }
  }

  fn is_safe_skip( &self, nth: usize ) -> bool {
    let diff = self.levels.iter()
      .enumerate()
      .filter_map( |(pos, level)| (pos != nth).then_some(level) )
      .tuple_windows()
      .map( |(prev, next)| next - prev )
      .collect::<Vec<_>>();
    let is_safe = diff.iter().all( |diff| (1..=3).contains(diff) );
    is_safe | diff.iter().all( |diff| (-3..=-1).contains(diff) )
  }
}

pub fn main() {
  let input = include_str!( "day_02.input" );
  let answer = part_one::compute_answer( input );
  println!( "{answer}" );
  let answer = part_two::compute_answer( input );
  println!( "{answer}" );
}

// =================================================================================================================================

mod part_one {
  use super::*;

  pub(super) fn compute_answer( input: &str ) -> usize {
    input.lines()
      .map( |line| line.parse::<Report>().unwrap() )
      .filter( Report::is_safe )
      .count()
  }

  #[cfg(test)]
  mod tests {
    use super::*;
    use super::super::tests::TEST_INPUT;

    #[test]
    fn test_is_safe() {
      for line in TEST_INPUT.lines() {
        let report = line.parse::<Report>()
          .expect( "line should be a valid Report" );
        println!( "{report:?} -> {is_safe}", is_safe = report.is_safe() );
      }
    }

    #[test]
    fn test_compute_answer() {
      let expected = 2;
      let actual = compute_answer( TEST_INPUT );
      assert_eq!( expected, actual );
    }
  }
}

mod part_two {
  use super::*;

  pub(super) fn compute_answer( input: &str ) -> usize {
    input.lines()
      .map( |line| line.parse::<Report>().unwrap() )
      .filter( |report| report.is_safe_dampened().is_ok() )
      .count()
  }

  #[cfg(test)]
  mod tests {
    use super::*;
    use super::super::tests::TEST_INPUT;

    #[test]
    fn test_is_safe_dampened() {
      for line in TEST_INPUT.lines() {
        let report = line.parse::<Report>()
          .expect( "line should be a valid Report" );
        println!( "{report:?} -> {is_safe:?}", is_safe = report.is_safe_dampened() );
      }
    }

    #[test]
    fn test_compute_answer() {
      let expected = 4;
      let actual = compute_answer( TEST_INPUT );
      assert_eq!( expected, actual );
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  pub(super) const TEST_INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

  #[test]
  fn test_parse_input() {
    for line in TEST_INPUT.lines() {
      let report = line.parse::<Report>()
        .expect( "line should be a valid Report" );
      println!( "{report:?}" );
    }
  }
}
