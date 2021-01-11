use super::file;
use file::{File, Compose};

pub struct Db<'a> {
  path: &'a str,
}

impl<'a> Db<'a> {
  /**
   * Create a new instance of the struct
   */
  pub fn new(path: &'a str) -> Self {
    Db { path }
  }

  /**
   * Execute a query
   */
  pub fn execute(&self, query: String) -> String {
    match query.find(char::is_whitespace) {
      Some(index) => {
        let command = &query[..index];
        let data = &query[index + 1..];

        match command.to_lowercase().as_str() {
          "find" => self.find(data),
          "save" => {
            let params: Vec<&str> = data.split(" ").collect();
            self.save(params[0], params[1])
          },
          _ => "nothing found".to_string(),
        }
      },
      None => panic!("unrecognizable query"),
    }
  }
}

impl<'a> Db<'a> {
  /**
   * Find value by a key
   */
  fn find(&self, value: &str) -> String { 
    "not found".to_string()
  }

  /**
   * Save or replace a value
   */
  fn save(&self, key: &str, value: &str) -> String {
    let (mut file, fo) = Compose::new(self.path).get();
    let buffer = &mut String::new();

    file.read_to_string(buffer);
    let mut lines = fo.get_as_collection(buffer);

    lines[3] = "occupation:Dietitian";
    fo.save_collection(lines);

    "ok".to_string()
  }

  /**
   * Delete line by a key
   */
  fn delete(&self, key: &str) -> String {
    "ok".to_string()
  }
}