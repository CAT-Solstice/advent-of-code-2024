use std::{fs, io, path};

// to run tests for examples : cargo test --example day_01 -- --nocapture
// cargo test --example day_01 part_one -- --nocapture
// cargo test --example '*'

// ---------------------------------------------------------------------------------------------------------------------------------

pub fn read_file_lines<P>( path: P ) -> io::Result<impl Iterator<Item=io::Result<String>>> where P: AsRef<path::Path> {

  fs::File::open( path )
    .map( io::BufReader::new )
    .map( io::BufRead::lines )
}

// ---------------------------------------------------------------------------------------------------------------------------------

use std::{sync::OnceLock, ops::Deref};

pub struct Lazy<T> {
  value: OnceLock<T>,
  supplier: fn() -> T,
}

impl<T> Lazy<T> {
  pub const fn from( supplier: fn() -> T ) -> Self {
    Self {
      value: OnceLock::new(),
      supplier,
    }
  }
}

impl<T> Deref for Lazy<T> {
  type Target = T;

  fn deref( &self ) -> &Self::Target {
    self.value.get_or_init( self.supplier )
  }
}

// ---------------------------------------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct Mat2D<T> {
  data: Vec<Vec<T>>,
}

impl<T, II> FromIterator<II> for Mat2D<T> where II: IntoIterator<Item=T> {
  fn from_iter<III>( iter: III ) -> Self where III: IntoIterator<Item=II> {
    let data = iter.into_iter()
      .map( |line| line.into_iter().collect() )
      .collect();
    Self { data }
  }
}

impl<T> Mat2D<T> {
  pub fn get( &self, (row, col): (usize, usize) ) -> Option<&T> {
    self.data.get( row )
      .and_then( |row| row.get(col) )
  }

  pub fn iter( &self ) -> impl Iterator<Item=((usize,usize), &T)> {
    self.data.iter().enumerate()
      .flat_map( |(row_idx, row)|
        row.iter().enumerate()
          .map( move |(col_idx, value)|
            ((row_idx,col_idx), value)
      ))
  }
}


// =================================================================================================================================

#[cfg(test)]
mod tests {

  use super::*;

  #[test]
  fn test_mat2d() {
    let data =
     "aaaaaaaa
bbbbbbbb
cccccccc
dddddddd
eeeeeeee
ffffffff
gggggggg
hhhhhhhh";

    let mat = data.lines()
      .map( str::chars )
      .collect::<Mat2D<_>>();

    assert_eq!( Some(&'a'), mat.get((0,0)) );
    assert_eq!( Some(&'a'), mat.get((0,7)) );
    assert_eq!( Some(&'h'), mat.get((7,0)) );
    assert_eq!( Some(&'h'), mat.get((7,7)) );
  }
}
