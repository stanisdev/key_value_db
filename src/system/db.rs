use super::file;
use file::File;

pub struct Db {
  query: String,
  // file: File,
}

impl Db {
  pub fn new(query: String) -> Self {
    Db {
      query,
      // file: File::new(),
    }
  }

  pub fn execute(&self) {
    // @todo: split by regexp
    let params: Vec<&str> = self.query.as_str().split(" ").collect();
    let data = params[1].to_string();

    match params[0].to_lowercase().as_str() {
      "find" => self.find(data),
      _ => (),
    }
  }
}

impl Db {
  fn find(&self, value: String) {
    let file = File::new();
    for line in file {
      println!("{}", line);
    }
  }

  fn save(&self) {
    
  }

  fn delete(&self) {

  }
}