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

pub struct Mat2D<T> {
  data: Vec<Vec<T>>,
}

impl<T> Mat2D<T> {
  pub fn from_iter_of_iter<II, III>( src: III ) -> Self
  where II: IntoIterator<Item=T>,
        III: IntoIterator<Item=II> {
    let data = src.into_iter()
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

    let mat = Mat2D::from_iter_of_iter( data.lines().map( str::chars ) );

    assert_eq!( Some(&'a'), mat.get((0,0)) );
    assert_eq!( Some(&'a'), mat.get((0,7)) );
    assert_eq!( Some(&'h'), mat.get((7,0)) );
    assert_eq!( Some(&'h'), mat.get((7,7)) );
  }

}
