use std::collections::HashMap;
use itertools::Itertools;

pub fn main() {
  let input = include_str!( "day_08.input" );
  let answer = part_one::compute_answer::<50, 50>( input );
  println!( "{answer}" );
}

fn parse_input( input: &str ) -> HashMap<char, Vec<(usize,usize)>> {
  input.lines().enumerate()
    .flat_map( |(row, line)| line.char_indices()
      .filter_map( move |(col, char)| match char {
        'a'..='z' | 'A'..='Z' | '0'..='9' => Some((char, (row,col))),
        _ => None,}))
    .into_group_map()
}

mod part_one {
  use std::collections::HashSet;
  use super::*;

  pub(super) fn compute_answer<const ROWS: usize, const COLS: usize>( input: &str ) -> usize {
    let antinodes = |one: (usize,usize), other: (usize,usize)| {
      get_antinodes::<ROWS,COLS>( one, other ).into_iter().flatten()
    };

    let antinodes = parse_input( input )
      .into_values()
      .flat_map( |positions| positions.into_iter()
        .tuple_combinations()
        .flat_map( |(one, other)| antinodes( one, other )))
      .collect::<HashSet<_>>();
     antinodes.len()
  }

  fn get_antinodes<const ROWS: usize, const COLS: usize>( one: (usize,usize), other: (usize,usize) ) -> [Option<(usize,usize)>; 2] {
    let inner = |first: (usize,usize), second: (usize,usize)| {
      let rows = second.0 as isize - first.0 as isize;
      let cols = second.1 as isize - first.1 as isize;
      match (second.0.checked_add_signed(rows)?, second.1.checked_add_signed(cols)?) {
        (row, col) if row < ROWS && col < COLS => Some((row, col)),
        _ => None,
      }
    };

    let first = inner( one, other );
    let second = inner( other, one );
    [first, second]
  }

  #[cfg(test)]
  mod tests {
    use super::*;
    use super::super::tests::TEST_INPUT;

    #[test]
    fn test_get_antinodes() {
      let one = (3, 4);
      let other = (5, 5);
      let expected = [Some((7, 6)), Some((1, 3))];
      let actual = get_antinodes::<10, 10>( one, other );
      assert_eq!( expected, actual );
    }

    #[test]
    fn test_compute_answer() {
      let expected = 14;
      let actual = compute_answer::<12, 12>( TEST_INPUT );
      assert_eq!( expected, actual );
    }
  }
}

mod part_two {

}

#[cfg(test)]
mod tests {
  use super::*;

  pub(super) const TEST_INPUT: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

  #[test]
  fn test_parse_input() {
    let network = parse_input( TEST_INPUT );
    println!( "{network:?}" );
  }
}
