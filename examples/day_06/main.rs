use advent_of_code_2024::Mat2D;

pub fn main() {
  todo!()
}

fn parse_input( input: &str ) -> Mat2D<char> {
  input.lines().map( str::chars ).collect()
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
}
