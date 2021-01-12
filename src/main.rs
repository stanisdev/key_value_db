mod system;
use system::db::Db;

fn main() {
  let query = "FIND occupation".to_string(); // Simulate a query
  let path = "./src/storage/data.txt";
  let mut db = Db::new(path);
  let result = db.execute(query);

  // println!("{}", result);
}