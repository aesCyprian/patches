pub struct Entry {
  address: u64,
  value: u64,
}

// Locates the entry point of binary bin.
// Returns an Entry containing its address and it's value.
pub fn point(bin: &[u8]) -> Entry {
  Entry { 0, 0 }
}