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
  fn turn_right( self ) -> Self {
    match self {
      Direction::North => Direction::East,
      Direction::East => Direction::South,
      Direction::South => Direction::West,
      Direction::West => Direction::North,
    }
  }
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

pub fn main() {
  let input = include_str!( "day_06.input" );
  let answer = part_one::compute_answer( input );
  println!( "{answer}" );
}

fn parse_input( input: &str ) -> Mat2D<char> {
  input.lines().map( str::chars ).collect()
}

fn find_start( mat: &Mat2D<char> ) -> Option<(usize, usize)> {
  mat.iter()
    .find_map( |(position, cell)|
      (*cell == '^').then_some( position )
    )
}

mod part_one {
  use super::*;
  use std::collections::HashSet;

  pub(super) fn compute_answer( input: &str ) -> usize {
    let mat = parse_input( input );
    let start = find_start( &mat )
      .expect( "should find a start position" );
    patrol( &mat, start, Direction::North )
}

  fn patrol( mat: &Mat2D<char>, mut position: (usize,usize), mut direction: Direction ) -> usize {
    let go_next = |position: (usize,usize), direction: Direction| {
      let position = position.go( direction )?;
      let cell = mat.get( position )?;
      Some((position, cell))
    };

    let mut visited = HashSet::new();
    visited.insert( position );

    while let Some(( next, cell )) = go_next( position, direction ) {
      match *cell {
        '#' => direction = direction.turn_right(),
        _ => {
          position = next;
          visited.insert( position );
        },
      }
    }

    visited.len()
  }

  #[cfg(test)]
  mod tests {
    use super::*;
    use super::super::tests::TEST_INPUT;

    #[test]
    fn test_compute_answer() {
      let expected = 41;
      let actual = compute_answer( TEST_INPUT );
      assert_eq!( expected, actual );
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  pub(super) const TEST_INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

  #[test]
  fn test_parse_input() {
    let mat = parse_input( TEST_INPUT );
    println!( "{mat:?}" );
  }

  #[test]
  fn test_find_start() {
    let mat = parse_input( TEST_INPUT );
    let expected = Some(( 6, 4 ));
    let actual = find_start( &mat );
    assert_eq!( expected, actual );
  }
}
