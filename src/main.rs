mod system;
use system::db::Db;

fn main() {
  let query = "FIND occupation".to_string(); // Simulate a query
  let mut db = Db::new();
  let result = db.execute(query);

  // println!("{}", result);
}