use std::str;

#[derive(Debug)]
struct Equation {
  result: isize,
  values: Vec<isize>,
}

impl str::FromStr for Equation {
  type Err = &'static str;

  fn from_str( input: &str ) -> Result<Self, Self::Err> {
    let mut split = input.split( ':' );
    let (Some(result), Some(values)) = (split.next(), split.next()) else {
      return Err( "failed to parse equation" );
    };
    let Ok(result) = result.parse() else {
      return Err( "failed to parse equation's result" );
    };
    let Ok(values) = values.trim().split(' ').map( str::parse ).collect::<Result<_,_>>() else {
      return Err( "failed to parse equation's values" );
    };

    let this = Self { result, values };
    Ok( this )
  }
}

// ---------------------------------------------------------------------------------------------------------------------------------

pub fn main() {
  let input = include_str!( "day_07.input" );
  let answer = part_one::compute_answer( input );
  println!( "{answer}" );
  let answer = part_two::compute_answer( input );
  println!( "{answer}" );
}

fn parse_input( input: &str ) -> Vec<Equation> {
  input.lines()
    .map( |line| line.parse() )
    .collect::<Result<Vec<_>,_>>()
    .expect( "input should parse as Equations" )
}

// ---------------------------------------------------------------------------------------------------------------------------------

mod part_one {
  use itertools::{self, Itertools};
  use super::*;

  #[derive(Debug, Clone, Copy)]
  enum Operator {
    Add,
    Mul,
  }

  const OPERATORS: &[Operator; 2] = &[
    Operator::Add,
    Operator::Mul,
  ];

  pub(super) fn compute_answer( input: &str ) -> isize {
    parse_input( input ).into_iter()
      .filter( try_solve )
      .map( |equation| equation.result )
      .sum()
  }

  fn try_solve( equation: &Equation ) -> bool {
    let operators_permutations = itertools::repeat_n( OPERATORS, equation.values.len()-1 )
      .multi_cartesian_product();
    for operators in operators_permutations {
      let mut values = equation.values.iter();
      let mut result = *values.next().unwrap();
      for (operand, operator) in std::iter::zip( values, operators ) {
        match operator {
          Operator::Add => result += operand,
          Operator::Mul => result *= operand,
        }
      }
      if result == equation.result {
        return true;
      }
    }

    false
  }

  #[cfg(test)]
  mod tests {
    use super::*;
    use super::super::tests::TEST_INPUT;

    #[test]
    fn test_try_solve() {
      let equation = Equation{ result: 190, values: vec![10,19] };
      let success = try_solve( &equation );
      assert!( success );
    }

    #[test]
    fn test_compute_answer() {
      let expected = 3749;
      let actual = compute_answer( TEST_INPUT );
      assert_eq!( expected, actual );
    }
  }
}

// ---------------------------------------------------------------------------------------------------------------------------------

mod part_two {
  use itertools::{self, Itertools};
  use super::*;

  #[derive(Debug, Clone, Copy)]
  enum Operator {
    Add,
    Mul,
    Concat,
  }

  const OPERATORS: &[Operator; 3] = &[
    Operator::Add,
    Operator::Mul,
    Operator::Concat,
  ];

  pub(super) fn compute_answer( input: &str ) -> isize {
    parse_input( input ).into_iter()
      .filter( try_solve )
      .map( |equation| equation.result )
      .sum()
  }

  fn try_solve( equation: &Equation ) -> bool {
    let operators_permutations = itertools::repeat_n( OPERATORS, equation.values.len()-1 )
      .multi_cartesian_product();
    for operators in operators_permutations {
      let mut values = equation.values.iter();
      let mut result = *values.next().unwrap();
      for (operand, operator) in std::iter::zip( values, operators ) {
        match operator {
          Operator::Add => result += operand,
          Operator::Mul => result *= operand,
          Operator::Concat => result = result * 10isize.pow(operand.ilog10()+1) + operand,
        }
      }
      if result == equation.result {
        return true;
      }
    }

    false
  }

  #[cfg(test)]
  mod tests {
    use super::*;
    use super::super::tests::TEST_INPUT;

    #[test]
    fn test_try_solve() {
      let equation = Equation{ result: 7290, values: vec![6,8,6,15] };
      let success = try_solve( &equation );
      assert!( success );
    }

    #[test]
    fn test_compute_answer() {
      let expected = 11387;
      let actual = compute_answer( TEST_INPUT );
      assert_eq!( expected, actual );
    }
  }
}

// ---------------------------------------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
  use super::*;

  pub(super) const TEST_INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

  #[test]
  fn test_parse_input() {
    let equations = parse_input( TEST_INPUT );
    for equation in equations {
      println!( "{equation:?}" );
    }
  }
}
