#[derive(Debug, Clone, Copy)]
pub struct ZeroPoint {
  address: u64,
  size: u64,
}

// Locates a code cave of at least size min in data bin.
// Returns a ZeroPoint containing the cave's address and size.
pub fn point(bin: &[u8], min: u64) -> ZeroPoint {
  let mut cave_size: u64 = 0;

  ZeroPoint { address: 0, size: 0 }
}