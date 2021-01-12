use std::io::prelude::*;
use std::fs::{File as FileSystem, write};
use std::io::{BufReader, Lines, Read};
use super::config::{Config};

pub struct File {
  pub descriptor: FileSystem,
  pub buffer: String,
}

impl File {
  pub fn new() -> Self {
    File {
      descriptor: FileSystem::open(Config::new().path).expect("??"),
      buffer: String::new(),
    }
  }

  pub fn read_to_string(&mut self, buffer: &mut String) {
    self.descriptor.read_to_string(buffer).expect("?");
  }

  pub fn get_reader(self) -> Lines<BufReader<FileSystem>> {
    BufReader::new(self.descriptor).lines()
  }
}

/**
 * Methods that are used to operate with the data
 */
pub struct FileOperations<'a> {
  config: Config<'a>,
}

impl<'a> FileOperations<'a> {
  pub fn new() -> Self {
    FileOperations {
      config: Config::new(),
    }
  }

  pub fn get_as_collection<'b>(&'a self, buffer: &'b String) -> Vec<&'b str> {
    buffer.as_str().split("\n").collect()
  }

  pub fn save_collection(&self, lines: &Vec<&str>) {
    let result = lines.join("\n");
    write(self.config.path, result.as_bytes()).expect("@");
  }
}

/**
 * A struct to create instances of
 * File and FileOperations structures
 */
pub struct Compose {}

impl Compose {
  pub fn new() -> Self {
    Compose {}
  }

  pub fn get(self) -> (File, FileOperations<'static>) {
    (
      File::new(),
      FileOperations::new(),
    )
  }
}
