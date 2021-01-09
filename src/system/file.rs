use std::io::prelude::*;
use std::fs::File as FileSystem;
use std::io::{BufReader, Lines};

pub struct File {
  pub lines: Lines<BufReader<FileSystem>>,
}

impl File {
  pub fn new() -> Self {
    let path = "./src/storage/data.txt"; // @todo: move to config
    let input = FileSystem::open(path).expect("file not found");
    File {
      lines: BufReader::new(input).lines(),
    }
  }
}

impl Iterator for File {
  type Item = String;

  fn next(&mut self) -> Option<Self::Item> {
    match self.lines.next() {
      Some(v) => Some(v.unwrap()),
      None => None,
    }
  }
}