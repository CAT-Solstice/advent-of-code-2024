pub fn main() {
  let input = include_str!( "day_11.input" );
  let answer = part_one::compute_answer( input );
  println!( "{answer}" );
  let answer = part_two::compute_answer( input );
  println!( "{answer}" );
}

fn parse_input( input: &str ) -> Vec<usize> {
  input.split_whitespace()
    .map( |stone| stone.parse() )
    .collect::<Result<Vec<_>,_>>()
    .unwrap()
}

mod part_one {
  use super::*;
  use itertools::Either;

  pub(super) fn compute_answer( input: &str ) -> usize {
    let mut stones = parse_input( input );
    for _ in 0..25 {
      stones = stones.into_iter().flat_map( blink ).collect();
    }
    stones.len()
  }

  fn blink( stone: usize ) -> impl Iterator<Item=usize> {
    if stone == 0 {
      return Either::Left( std::iter::once(1_usize) );
    }
    let num_digits = stone.ilog10() + 1;
    if num_digits % 2 == 0 {
      let split_at = 10_usize.pow( num_digits / 2 );
      let left = stone / split_at;
      let right = stone - left*split_at;
      return Either::Right( [left, right].into_iter() );
    }
    Either::Left( std::iter::once(stone*2024) )
  }

  #[cfg(test)]
  mod tests {
    use super::*;
    use super::super::tests::TEST_INPUT;

    #[test]
    fn test_blink() {
      let input = parse_input( TEST_INPUT );
      let expected: &[usize] = &[ 253000, 1, 7 ];
      let actual = input.into_iter().flat_map( blink ).collect::<Vec<_>>();
      assert_eq!( expected, actual );

      let expected: &[usize] = &[ 253, 0, 2024, 14168 ];
      let actual = actual.into_iter().flat_map( blink ).collect::<Vec<_>>();
      assert_eq!( expected, actual );

      let expected: &[usize] = &[ 512072, 1, 20, 24, 28676032 ];
      let actual = actual.into_iter().flat_map( blink ).collect::<Vec<_>>();
      assert_eq!( expected, actual );

      let expected: &[usize] = &[ 512, 72, 2024, 2, 0, 2, 4, 2867, 6032 ];
      let actual = actual.into_iter().flat_map( blink ).collect::<Vec<_>>();
      assert_eq!( expected, actual );

      let expected: &[usize] = &[ 1036288, 7, 2, 20, 24, 4048, 1, 4048, 8096, 28, 67, 60, 32 ];
      let actual = actual.into_iter().flat_map( blink ).collect::<Vec<_>>();
      assert_eq!( expected, actual );

      let expected: &[usize] = &[ 2097446912, 14168, 4048, 2, 0, 2, 4, 40, 48, 2024, 40, 48, 80, 96, 2, 8, 6, 7, 6, 0, 3, 2 ];
      let actual = actual.into_iter().flat_map( blink ).collect::<Vec<_>>();
      assert_eq!( expected, actual );
    }

    #[test]
    fn test_compute_answer() {
      let expected = 55312;
      let actual = compute_answer( TEST_INPUT );
      assert_eq!( expected, actual );
    }
  }
}

mod part_two {
  use std::collections::HashMap;
  use itertools::Either;
  use super::*;

  const MAX_DEPTH: usize = 75;
  type Cache = HashMap<(usize,usize), usize>;

  pub(super) fn compute_answer( input: &str ) -> usize {
    fn inner( stone: usize, depth: usize, cache: &mut Cache ) -> usize {
      if depth == MAX_DEPTH { return 1; }
      if let Some( count ) = cache.get( &(stone, MAX_DEPTH-depth) ) { return *count; }
      let count = match blink( stone ) {
        Either::Left( stone ) => inner( stone, depth+1, cache ),
        Either::Right( [left, right] ) => inner( left, depth+1, cache ) + inner( right, depth+1, cache ),
      };
      cache.insert( (stone, MAX_DEPTH-depth), count );
      count
    }

    let mut cache = Cache::new();
    let stones = parse_input( input );
    let now = std::time::Instant::now();
    let count = stones.into_iter()
      .map( |stone| inner( stone, 0, &mut cache ) )
      .sum();
    println!( "total: {count} in {}ms", now.elapsed().as_millis() );
    println!( "cache size: {}", cache.len() );
    count
  }

  fn blink( stone: usize ) -> Either<usize, [usize; 2]> {
    if stone == 0 {
      return Either::Left( 1_usize );
    }
    let num_digits = stone.ilog10() + 1;
    if num_digits % 2 == 0 {
      let split_at = 10_usize.pow( num_digits / 2 );
      let left = stone / split_at;
      let right = stone - left*split_at;
      return Either::Right( [left, right] );
    }
    Either::Left( stone * 2024 )
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  pub(super) const TEST_INPUT: &str = "125 17";

  #[test]
  fn test_parse_input() {
    let expected: &[usize] = &[ 125, 17 ];
    let actual = parse_input( TEST_INPUT );
    assert_eq!( expected, actual );
  }
}
