use std::io::prelude::*;
use std::fs::{File as FileSystem, write};
use std::io::{BufReader, Lines, Read};

pub struct File {
  pub descriptor: FileSystem,
  pub buffer: String,
}

impl File {
  pub fn new(path: &str) -> Self {
    File {
      descriptor: FileSystem::open(path).expect("??"),
      buffer: String::new(),
    }
  }

  pub fn read_to_string(&mut self, buffer: &mut String) {
    self.descriptor.read_to_string(buffer).expect("?");
  }
}

/**
 * Methods that are used to operate with the data
 */
pub struct FileOperations<'a> {
  path: &'a str,
}

impl<'a> FileOperations<'a> {
  pub fn new(path: &'a str) -> Self {
    FileOperations { path }
  }

  pub fn get_as_collection<'b>(&'a self, buffer: &'b String) -> Vec<&'b str> {
    buffer.as_str().split("\n").collect()
  }

  pub fn save_collection(&self, lines: Vec<&str>) {
    let result = lines.join("\n");
    write(self.path, result.as_bytes()).expect("@@");
  }
}

/**
 * A struct to create instances of
 * File and FileOperations structures
 */
pub struct Compose<'a> {
  path: &'a str,
}

impl<'a> Compose<'a > {
  pub fn new(path: &'a str) -> Self {
    Compose { path }
  }

  pub fn get(self) -> (File, FileOperations<'a>) {
    (
      File::new(self.path),
      FileOperations::new(self.path),
    )
  }
}
