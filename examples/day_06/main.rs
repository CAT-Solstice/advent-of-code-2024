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
  todo!()
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

#[cfg(test)]
mod tests {
  use super::*;

  const TEST_INPUT: &str = "....#.....
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
