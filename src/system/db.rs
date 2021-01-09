use super::file;
use file::File;

pub struct Db {
  query: String,
}

impl Db {
  /**
   * Create a new instance of the struct
   */
  pub fn new(query: String) -> Self {
    Db { query }
  }

  /**
   * Execute a query
   */
  pub fn execute(&self) -> String {
    match self.query.find(char::is_whitespace) {
      Some(index) => {
        let command = &self.query[..index];
        let data = &self.query[index + 1..];

        match command.to_lowercase().as_str() {
          "find" => self.find(data),
          "save" => self.save(),
          _ => "nothing found".to_string(),
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
  fn find(&self, value: &str) -> String {
    for line in File::new() {
      if let Some(index) = line.find(':') {
        let key = &line[..index];

        if key == value {
          let result = &line[index + 1..];
          return result.to_string();
        }
      }
    }
    "not found".to_string()
  }

  fn save(&self) -> String {
    "ok".to_string()
  }

  fn delete(&self) {

  }
}