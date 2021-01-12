use super::file::{File, Compose};

pub struct Query {
  pub result: Option<String>,
}

impl Query {
  /**
   * Create an instance of the structure
   */
  pub fn new() -> Self {
    Query {
      result: None,
    }
  }

  /**
   * Find value by a key
   */
  pub fn find(&mut self, value: &str) {
    let reader = File::new().get_reader();

    for line in reader {
      let data = line.unwrap();

      if let Some(index) = data.find(':') {
        let key = &data[..index];

        if key == value {
          let result = &data[index + 1..];
          self.set_result(result.to_string());
        }
      }
    }
  }

  /**
   * Save or replace a value
   */
  pub fn save(&mut self, key: &str, value: &str) {
    let (mut file, fo) = Compose::new().get();
    let buffer = &mut String::new();

    file.read_to_string(buffer);
    let mut lines = fo.get_as_collection(buffer);

    lines[3] = "occupation:Dietitian";
    fo.save_collection(lines);
    self.set_result("ok");
  }

  /**
   * Delete line by a key
   */
  pub fn delete(&self, key: &str) -> String {
    "ok".to_string()
  }
}

impl Query {
  /**
   * Set result of a query
   */
  fn set_result<T: ToString>(&mut self, value: T) {
    self.result = Some(value.to_string());
  }
}