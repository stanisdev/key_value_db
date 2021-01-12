mod system;
use system::db::Db;

fn main() {
  let input = "SAVE hobby Sport".to_string(); // Simulate a query
  let db = Db {};
  let result = db.execute(input);

  match result {
    Some(value) => println!("{}", value),
    None => (),
  }
}