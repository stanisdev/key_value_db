pub struct Config<'a> {
  pub path: &'a str,
}

impl<'a> Config<'a> {
  pub fn new() -> Self {
    Config {
      path: "./src/storage/data.txt",
    }
  }
}