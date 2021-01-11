mod system;
use system::db::Db;

fn main() {
  let query = "SAVE occupation Dietitian".to_string(); // Simulate a query
  let path = "./src/storage/data.txt";
  let db = Db::new(path);
  let result = db.execute(query);

  // println!("{}", result);
}