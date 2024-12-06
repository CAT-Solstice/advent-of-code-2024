use advent_of_code_2024::Mat2D;

pub fn main() {
  let input = include_str!( "day_04.input" );
  let answer = part_one::compute_answer( input, (140,140) );
  println!( "{answer}" );
  let answer = part_two::compute_answer( input, (140,140) );
  println!( "{answer}" );
}

fn parse_input( input: &str ) -> Mat2D<char> {
    input.lines().map( str::chars ).collect()
}

mod part_one {
  use super::*;

  fn count_at( mat: &Mat2D<char>, row_offset: usize, col_offset: usize ) -> usize {
    const XMAS: [Option<&char>; 4] = [Some(&'X'), Some(&'M'), Some(&'A'), Some(&'S')];

    let get = move |row: usize, col: usize| -> Option<&char> {
      mat.get(( row+row_offset, col+col_offset ))
    };

    let mut count = 0;
    [get(0,0), get(0,1), get(0,2), get(0,3)].eq( &XMAS ).then( || count += 1 );
    [get(0,3), get(0,2), get(0,1), get(0,0)].eq( &XMAS ).then( || count += 1 );

    [get(0,0), get(1,0), get(2,0), get(3,0)].eq( &XMAS ).then( || count += 1 );
    [get(3,0), get(2,0), get(1,0), get(0,0)].eq( &XMAS ).then( || count += 1 );

    [get(0,0), get(1,1), get(2,2), get(3,3)].eq( &XMAS ).then( || count += 1 );
    [get(3,3), get(2,2), get(1,1), get(0,0)].eq( &XMAS ).then( || count += 1 );

    [get(0,3), get(1,2), get(2,1), get(3,0)].eq( &XMAS ).then( || count += 1 );
    [get(3,0), get(2,1), get(1,2), get(0,3)].eq( &XMAS ).then( || count += 1 );

    count
  }

  pub(super) fn compute_answer( input: &str, dim: (usize, usize) ) -> usize {
    let (rows, cols) = dim;
    let mat = parse_input( input );
    let mut count = 0;
    for row in 0..rows {
      for col in 0..cols {
        count += count_at( &mat, row, col );
      }
    }
    count
  }

  #[cfg(test)]
  mod tests {
    use super::*;
    use super::super::tests::TEST_INPUT;

    #[test]
    fn test_count_at() {
      let mat = parse_input( TEST_INPUT );
      let count = count_at( &mat, 0, 5 );
      assert_eq!( 1, count );
      let count = count_at( &mat, 1, 6 );
      assert_eq!( 1, count );
    }

    #[test]
    fn test_compute_answer() {
      let count = compute_answer( TEST_INPUT, (10,10) );
      assert_eq!( 18, count );
    }
  }
}

mod part_two {
  use super::*;

  fn count_at( mat: &Mat2D<char>, row_offset: usize, col_offset: usize ) -> usize {
    const MAS: [Option<&char>; 3] = [Some(&'M'), Some(&'A'), Some(&'S')];

    let get = move |row: usize, col: usize| -> Option<&char> {
      mat.get(( row+row_offset, col+col_offset ))
    };

    let one = [get(0,0), get(1,1), get(2,2)].eq( &MAS ) || [get(2,2), get(1,1), get(0,0)].eq( &MAS );
    let two = [get(2,0), get(1,1), get(0,2)].eq( &MAS ) || [get(0,2), get(1,1), get(2,0)].eq( &MAS );

    if one & two { 1 } else { 0 }
  }

  pub(super) fn compute_answer( input: &str, dim: (usize, usize) ) -> usize {
    let (rows, cols) = dim;
    let mat = parse_input( input );
    let mut count = 0;
    for row in 0..rows {
      for col in 0..cols {
        count += count_at( &mat, row, col );
      }
    }
    count
  }

  #[cfg(test)]
  mod tests {
    use super::*;
    use super::super::tests::TEST_INPUT;

    #[test]
    fn test_count_at() {
      let mat = parse_input( TEST_INPUT );
      let count = count_at( &mat, 0, 1 );
      assert_eq!( 1, count );
      let count = count_at( &mat, 1, 6 );
      assert_eq!( 1, count );
    }

    #[test]
    fn test_compute_answer() {
      let count = compute_answer( TEST_INPUT, (10,10) );
      assert_eq!( 9, count );
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  pub(super) const TEST_INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

  #[test]
  fn test_input() {
    let mat = parse_input( TEST_INPUT );
    assert_eq!( Some(&'M'), mat.get((4, 1)) );
  }
}
