mod system;
use system::db::Db;

fn main() {
  let query = "FIND name".to_string(); // Simulate a query
  let db = Db::new(query);
  let result = db.execute();

  println!("{}", result);
}