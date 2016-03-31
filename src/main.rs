struct Squares {
  curr: u8
}

impl Iterator for Squares {
  type Item = u8;

  fn next(&mut self) -> Option<u8> {

    self.curr += 1;

    if self.curr <= 2u8.pow(4) - 1 {
      Some(self.curr.pow(2))
    } else {
      None
    }
  }
}

fn squares() -> Squares {
  Squares { curr: 0 }
}


fn main() {
  for i in squares() {
    println!("{}", i);
  }
}
