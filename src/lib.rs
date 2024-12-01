use std::{fs, io, path};

// to run tests for examples : cargo test --example day_01 -- --nocapture
// cargo test --example day_01 part_one -- --nocapture
// cargo test --example '*'

pub fn read_file_lines<P>( path: P ) -> io::Result<impl Iterator<Item=io::Result<String>>> where P: AsRef<path::Path> {

  fs::File::open( path )
    .map( io::BufReader::new )
    .map( io::BufRead::lines )
}
