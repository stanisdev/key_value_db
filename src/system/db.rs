use super::query::Query;

pub struct Db {}

impl Db {
  /**
   * Execute a query
   */
  pub fn execute(&self, input: String) -> Option<String> {
    match input.find(char::is_whitespace) {
      Some(index) => {
        let command = &input[..index];
        let data = &input[index + 1..];
        let mut query = Query::new();

        match command.to_lowercase().as_str() {
          "find" => query.find(data),
          "save" => {
            let params: Vec<&str> = data.split(" ").collect();
            query.save(params[0], params[1])
          },
          _ => (),
        };
        query.result
      },
      None => panic!("unrecognizable query"),
    }
  }
}