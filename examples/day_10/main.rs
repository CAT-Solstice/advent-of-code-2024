use std::collections::HashMap;
use advent_of_code_2024::Mat2D;

// ---------------------------------------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
  North,
  East,
  South,
  West,
}

impl Direction {
  const DIRECTIONS: [Self; 4] = [
    Direction::North,
    Direction::East,
    Direction::South,
    Direction::West,
  ];
}

// ---------------------------------------------------------------------------------------------------------------------------------

trait Position {
  fn go( &self, direction: Direction ) -> Option<Self> where Self: Sized;
}

impl Position for (usize, usize) {
  fn go( &self, direction: Direction ) -> Option<Self> {
    let (row, col) = *self;
    match direction {
      Direction::North if row > 0 => Some((row-1, col)),
      Direction::East => Some((row, col+1)),
      Direction::South => Some((row+1, col)),
      Direction::West if col > 0 => Some((row, col-1)),
      _ => None,
    }
  }
}

// ---------------------------------------------------------------------------------------------------------------------------------

type Grid = Mat2D<u8>;
type Map = HashMap<(usize,usize), Vec<(usize,usize)>>;

pub fn main() {
  let input = include_str!( "day_10.input" );
  let answer = part_one::compute_answer( input );
  println!( "{answer}" );
  let answer = part_two::compute_answer( input );
  println!( "{answer}" );
}

fn parse_input( input: &str ) -> (Grid, Map) {
  let mat = input.lines()
    .map( |line| line.chars()
      .map( |char| char.to_digit(10).unwrap() as u8 ))
    .collect::<Mat2D<_>>();
  let map = mat.iter()
    .map( |(position, _)| (position, neighbours(&mat,position)) )
    .collect::<HashMap<_,_>>();
  (mat, map)
}

fn neighbours( mat: &Mat2D<u8>, position: (usize,usize) ) -> Vec<(usize, usize)> {
  let next_height = 1 + *mat.get( position ).unwrap();
  Direction::DIRECTIONS.iter().copied()
    .filter_map( |direction| position.go(direction) )
    .filter_map( |position| mat.get( position )
      .and_then( |height| (*height == next_height).then_some(position) ))
    .collect()
}

// ---------------------------------------------------------------------------------------------------------------------------------

mod part_one {
  use super::*;

  pub(super) fn compute_answer( input: &str ) -> usize {
    let (mat, map) = parse_input( input );
    let starts = mat.iter()
      .filter_map( |(position, height)| (*height == 0).then_some(position) );
    let ends = mat.iter()
      .filter_map( |(position, height)| (*height == 9).then_some(position) )
      .collect::<Vec<_>>();
    let is_trail = |start: &(usize,usize), end: &(usize,usize)| {
      is_trail( start, end, &map )
    };
    starts
      .flat_map( |start| ends.iter()
        .filter_map( move |end| is_trail( &start, end ).then_some(( start, end )) ))
      .count()
  }

  fn is_trail( start: &(usize,usize), end: &(usize,usize), map: &Map ) -> bool {
    fn inner( current: &(usize,usize), end: &(usize,usize), map: &Map ) -> bool {
      if current.eq( end ) { return true; }
      for neighbour in &map[ current ] {
        if inner( neighbour, end, map ) {
          return true;
        }
      }
      false
    }
    inner( start, end, map )
  }

  #[cfg(test)]
  mod tests {
    use super::*;
    use super::super::tests::{TEST_INPUT_1, TEST_INPUT_2};

    #[test]
    fn test_is_trail() {
      let (_, map) = parse_input( TEST_INPUT_1 );
      assert!(is_trail( &(0, 0), &(3, 0), &map ));
    }

    #[test]
    fn test_compute_answer() {
      let expected = 36;
      let actual = compute_answer( TEST_INPUT_2 );
      assert_eq!( expected, actual );
    }
  }
}

// ---------------------------------------------------------------------------------------------------------------------------------

mod part_two {
  use super::*;

  pub(super) fn compute_answer( input: &str ) -> usize {
    let (mat, map) = parse_input( input );
    let starts = mat.iter()
      .filter_map( |(position, height)| (*height == 0).then_some(position) );
    let ends = mat.iter()
      .filter_map( |(position, height)| (*height == 9).then_some(position) )
      .collect::<Vec<_>>();
    let count_trails = |start: &(usize,usize), end: &(usize,usize)| {
      count_trails( start, end, &map )
    };
    starts
      .flat_map( |start| ends.iter()
        .map( move |end| count_trails( &start, end) ) )
      .sum()
  }

  fn count_trails( start: &(usize,usize), end: &(usize,usize), map: &Map ) -> usize {
    fn inner( current: &(usize,usize), end: &(usize,usize), map: &Map ) -> usize {
      if current.eq( end ) { return 1; }
      map[ current ].iter()
        .map( |neighbour| inner( neighbour, end, map ) )
        .sum()
    }
    inner( start, end, map )
  }

  #[cfg(test)]
  mod tests {
    use super::*;
    use super::super::tests::TEST_INPUT_2;

    #[test]
    fn test_compute_answer() {
      let expected = 81;
      let actual = compute_answer( TEST_INPUT_2 );
      assert_eq!( expected, actual );
    }
  }
}

// ---------------------------------------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
  use super::*;

  pub(super) const TEST_INPUT_1: &str = "0123
1234
8765
9876";

  pub(super) const TEST_INPUT_2: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

  #[test]
  fn test_parse_input() {
    let (_, map) = parse_input( TEST_INPUT_1 );
    assert_eq!( &[(0,1), (1,0)][..], map[&(0,0)] );
    assert_eq!( &[(3,2), (2,1)][..], map[&(2,2)] );
  }
}
