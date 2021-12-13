use std::fmt;

pub struct List(pub Vec<u32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) ->fmt::Result {
      let vec = &self.0;

      write!(f, "[")?;
      // Iterate over `v` in `vec` while enumerating the iteration
      // count in `count`.
      for (count, v) in vec.iter().enumerate() {
          // For every element except the first, add a comma.
          // Use the ? operator to return on errors.
          if count != 0 { write!(f, ", ")?; }
          write!(f, "{}", v)?;
      }

      // Close the opened bracket and return a fmt::Result value.
      write!(f, "]")
    }
}
