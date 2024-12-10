#[derive(Debug)]
enum Marker {
    File,
    Empty,
}

const MARKERS: &[Marker] = &[Marker::File, Marker::Empty];

pub fn main() {
  let input = include_str!( "day_09.input" );
  let answer = part_one::compute_answer( input );
  println!( "{answer}" );
}

fn parse_input( input: &str ) -> Vec<Option<usize>> {
    let mut disk = Vec::new();
    let mut file_id = 0;
    for (len, marker) in std::iter::zip( input.chars(), MARKERS.iter().cycle() ) {
      let len = len.to_digit(10).unwrap() as usize;
      match marker {
        Marker::File => {
          disk.extend( std::iter::repeat_n(Some(file_id),len) );
          file_id += 1;
        },
        Marker::Empty => {
          disk.extend( std::iter::repeat_n(None,len) );
        },
      }
    }
    disk
}

mod part_one {
  use super::*;

  pub(super) fn compute_answer( input: &str ) -> usize {
    let mut disk = parse_input( input );
    defrag( &mut disk );
    checksum( &disk )
  }

  fn defrag( disk: &mut Vec<Option<usize>> ) {
    while let Some( empty ) = disk.iter().position( Option::is_none ) {
      disk.swap_remove( empty );
    }
  }

  fn checksum( disk: &[Option<usize>] ) -> usize {
    disk.iter().flatten().enumerate()
      .map( |(pos, &file_id)| pos * file_id )
      .sum()
  }

  #[cfg(test)]
  mod tests {
    use super::*;
    use super::super::tests::{TEST_INPUT_1, TEST_INPUT_2};

    #[test]
    fn test_defrag() {
      // 022111222
      let expected = [0, 2, 2, 1, 1, 1, 2, 2, 2].map( Option::Some );
      let mut actual = parse_input( TEST_INPUT_1 );
      defrag( &mut actual );
      assert_eq!( &expected[..], &actual[..] );

      // 0099811188827773336446555566
      let expected = [0, 0, 9, 9, 8, 1, 1, 1, 8, 8, 8, 2, 7, 7, 7, 3, 3, 3, 6, 4, 4, 6, 5, 5, 5, 5, 6, 6].map( Option::Some );
      let mut actual = parse_input( TEST_INPUT_2 );
      defrag( &mut actual );
      assert_eq!( &expected[..], &actual[..] );
    }

    #[test]
    fn test_compute_answer() {
      let expected = 1928;
      let actual = compute_answer( TEST_INPUT_2 );
      assert_eq!( expected, actual );
    }
  }
}

mod part_two {

}

#[cfg(test)]
mod tests {
  use super::*;

  pub(super) const TEST_INPUT_1: &str = "12345";
  pub(super) const TEST_INPUT_2: &str = "2333133121414131402";

  #[test]
  fn test_parse_input() {
    let expected = &[
      Some(0),
      None, None,
      Some(1), Some(1), Some(1),
      None, None, None, None,
      Some(2), Some(2), Some(2), Some(2), Some(2)
    ];
    let actual = parse_input( TEST_INPUT_1 );
    assert_eq!( expected, &actual[..] );

    let actual = parse_input( TEST_INPUT_2 );
    println!( "{actual:?}" );
  }
}
