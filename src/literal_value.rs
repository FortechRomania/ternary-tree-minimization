use std::hash::{Hash, Hasher};

#[derive(Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum LiteralValue {
  True = 1,
  False = 0,
  DontCare = 2,
}

impl Hash for LiteralValue {
  /**
   * Hash function for the literal value
   */
  fn hash<H: Hasher>(&self, state: &mut H) {
    if *self == LiteralValue::True {
      1.hash(state);
    } else if *self == LiteralValue::False {
      0.hash(state);
    } else {
      2.hash(state);
    }
  }
}
