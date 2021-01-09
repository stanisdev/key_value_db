use super::file;
use file::File;

pub struct Db {
  query: String,
  file: File,
}

impl Db {
  /**
   * Create a new instance of the struct
   */
  pub fn new(query: String) -> Self {
    Db {
      query,
      file: File::new(),
    }
  }

  /**
   * Execute a query
   */
  pub fn execute(&self) {
    match self.query.find(char::is_whitespace) {
      Some(index) => {
        let command = &self.query[..index];
        let data = &self.query[index + 1..];

        match command.to_lowercase().as_str() {
          "find" => self.find(data),
          "save" => self.save(),
          _ => (),
        }
      },
      None => panic!("unrecognizable query"),
    }
  }
}

impl Db {
  /**
   * Find value by a key
   */
  fn find(&self, value: &str) {
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