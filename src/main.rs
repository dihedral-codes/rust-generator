struct Bracelets {
  curr: u64
  , mask: u64
}

impl Bracelets {
  
  fn new(k: u8, w: u8) -> Bracelets  {
    
    // Set the bit mask for k bits.
    let mut mask:u64 = 0;
    let mut i: u8 = k;
    while i > 4 {
      mask = (mask << 4) | 0b1111u64;
      i -= 4;
    }
    while i > 0 {
      mask = (mask << 1) | 0b1u64;
      i -= 1;
    }

    // Start with 0111..11000....0
    let curr: u64 = ((1u64 << w) - 1) << (k - w - 1);

    Bracelets { curr: curr, mask: mask }
  }

}

impl Iterator for Bracelets {
  type Item = u64;

  fn next(&mut self) -> Option<u64> {

    // Adapted from: http://graphics.stanford.edu/~seander/bithacks.html
    let t:u64 = self.curr | (self.curr - 1);
    self.curr = (t + 1) | (((!t & (t + 1)) - 1) >> (self.curr.trailing_zeros() + 1));  
    
    if self.curr <= self.mask {
      Some(self.curr)
    } else {
      None
    }
  }
}

fn main() {
  for i in 3..15 {
    for j in Bracelets::new(48, i) {
      println!("{:b}", j);
    }
  }
}
