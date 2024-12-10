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
  let answer = part_two::compute_answer( input );
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

fn checksum( disk: &[Option<usize>] ) -> usize {
  disk.iter().enumerate()
    .filter_map( |(pos, elt)| elt
      .map( |file_id| pos * file_id ))
    .sum()
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
  use itertools::Itertools;
  use itertools::FoldWhile::{Continue, Done};
  use super::*;

  pub(super) fn compute_answer( input: &str ) -> usize {
    let mut disk = parse_input( input );
    defrag( &mut disk );
    checksum( &disk )
  }

  fn defrag( disk: &mut [Option<usize>] ) {
    // maybe not the best way to do it, will look for a better/simpler solution
    let last_file_id = *disk.iter().rev().flatten().next().unwrap();
    for src_file_id in (0..=last_file_id).rev() {
      let (src_pos, src_len) = disk.iter().enumerate().rev()
        .flat_map( |(idx, elt)| elt.map( |file_id| (idx, file_id) ) )
        .skip_while( |(_, file_id)| *file_id != src_file_id )
        .take_while( |(_, file_id)| *file_id == src_file_id )
        .fold( None, |span, (idx,_)| span
          .map( |(_, len)| (idx, len+1) )
          .or( Some((idx, 1)) ))
        .unwrap();

      let (dst_pos, dst_len) = &disk[0..src_pos].iter().enumerate()
        .filter_map( |(idx, elt)| elt.is_none().then_some(idx) )
        .fold_while( (0, 0), |span, idx| match span {
          (start, len) if len == src_len => Done((start, len)),
          (start, len) if idx > start+len => Continue((idx, 1)),
          (start, len) => Continue((start, len+1)),})
        .into_inner();

      if *dst_len == src_len {
        for offset in 0..src_len {
          disk.swap( src_pos+offset, dst_pos+offset );
        }
      }
    }
  }

  #[cfg(test)]
  mod tests {
    use super::*;
    use super::super::tests::TEST_INPUT_2;

    #[test]
    fn test_defrag() {
      // 00992111777.44.333....5555.6666.....8888..
      let expected = [
        Some(0), Some(0),
        Some(9), Some(9),
        Some(2),
        Some(1), Some(1), Some(1),
        Some(7), Some(7), Some(7),
        None,
        Some(4), Some(4),
        None,
        Some(3), Some(3), Some(3),
        None, None, None, None,
        Some(5), Some(5), Some(5), Some(5),
        None,
        Some(6), Some(6), Some(6), Some(6),
        None, None, None, None, None,
        Some(8), Some(8), Some(8), Some(8),
        None, None,
      ];
      let mut disk = parse_input( TEST_INPUT_2 );
      defrag( &mut disk );
      println!( "{disk:?}" );
      assert_eq!( &expected[..], &disk[..] );
    }

    #[test]
    fn test_compute_answer() {
      let expected = 2858;
      let actual = compute_answer( TEST_INPUT_2 );
      assert_eq!( expected, actual );
    }
  }
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
