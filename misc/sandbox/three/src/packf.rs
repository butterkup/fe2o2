pub mod inner {
  pub fn three() {
    println!("Hello from three::packf::inner::three()");
    super::super::lib();
  }

  pub struct Range {
    pub start: isize,
    pub stop: isize,
  }

  impl Range {
    pub fn new(start: isize, stop: isize) -> Range { Range { start, stop } }

    pub fn to(stop: isize) -> Range { Range { start: 0, stop } }
    pub fn rev(&self) -> Range {
      Range {
        start: self.start,
        stop: self.stop,
      }
    }
    pub fn iter(&self) -> RangeIterator {
      RangeIterator {
        current: self.start,
        end: self.stop,
        step: if self.start < self.stop { 1 } else { -1 },
      }
    }
  }

  pub struct RangeIterator {
    current: isize,
    end: isize,
    step: isize,
  }

  impl Iterator for RangeIterator {
    type Item = isize;

    fn next(&mut self) -> Option<Self::Item> {
      if self.current == self.end {
        None
      } else {
        let current = self.current;
        self.current += self.step;
        Some(current)
      }
    }
  }
}
