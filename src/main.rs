mod db;

fn main() {
  let mut x: db::Db<String> = db::Db::new();
  x.set("name", String::from("Stas"));

  match x.get("name") {
    Some(v) => println!("{}", v),
    None => (),
  }

  x.delete("name");
}
