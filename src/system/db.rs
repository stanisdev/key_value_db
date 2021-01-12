use super::file::{File, Compose};

pub struct Db<'a> {
  result: Option<&'a str>,
}

impl<'a> Db<'a> {
  /**
   * Create a new instance of the struct
   */
  pub fn new() -> Self {
    Db {
      result: None,
    }
  }

  /**
   * Execute a query
   */
  pub fn execute(&mut self, query: String) {
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
          _ => (),
        };
      },
      None => panic!("unrecognizable query"),
    };
  }
}

impl<'a> Db<'a> {
  /**
   * Find value by a key
   */
  fn find(&mut self, value: &str) {
    let reader = File::new().get_reader();

    for line in reader {
      let data = line.unwrap();

      if let Some(index) = data.find(':') {
        let key = &data[..index];

        if key == value {
          let result = &data[index + 1..];
          println!("{}", result);
          // self.result = Some(result);
        }
      }
    }
  }

  /**
   * Save or replace a value
   */
  fn save(&mut self, key: &str, value: &str) {
    let (mut file, fo) = Compose::new().get();
    let buffer = &mut String::new();

    file.read_to_string(buffer);
    let mut lines = fo.get_as_collection(buffer);

    lines[3] = "occupation:Dietitian";
    fo.save_collection(lines);
    self.result = Some("ok");
  }

  /**
   * Delete line by a key
   */
  fn delete(&self, key: &str) -> String {
    "ok".to_string()
  }
}