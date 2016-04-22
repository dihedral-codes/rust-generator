struct Bracelets {
  curr: u64
	, mask: u64
}

impl Bracelets {
	
	fn new(k: u8) -> Bracelets  {
		
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
		
		// Let the initial value be 10000..0 with k zeroes.
		let curr: u64 = 1u64 << k - 1;
		
		Bracelets { curr: curr, mask: mask }
	}

}

impl Iterator for Bracelets {
  type Item = u64;

  fn next(&mut self) -> Option<u64> {
	
    self.curr += 1u64;
		
    if self.curr <= self.mask {
      Some(self.curr)
    } else {
      None
    }
  }
}

fn main() {
  for i in Bracelets::new(4) {
    println!("{:b}", i);
  }
}
