pub struct Db<'a, V> {
  data: Vec<(&'a str, V)>,
}

impl<'a, V> Db<'a, V> {
  pub fn new() -> Self {
    Db {
      data: vec![],
    }
  }

  pub fn set(&mut self, key: &'a str, value: V) {
    let result = self.data.iter().position(|x| x.0 == key);
    match result {
      Some(index) => self.data[index].1 = value,
      None => {
        let element = (key, value);
        self.data.push(element);
      },
    }
  }

  pub fn get(&self, key: &str) -> Option<&V> {
    for item in &self.data {
      if key == item.0 {
        return Some(&item.1);
      }
    }
    None
  }

  pub fn delete(&mut self, key: &str) {
    let mut i = 0;
    while i != self.data.len() {
      if self.data[i].0 == key {
        self.data.remove(i);
        break;
      }
      i += 1;
    }
  }
}
