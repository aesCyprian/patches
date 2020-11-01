use std::fmt;
use std::error;

// For when the binary format is unknowm.
// The String it holds may contain the best guess at its format.
#[derive(Debug, Clone)]
pub struct BinaryFormatError(Option<String>);

impl fmt::Display for BinaryFormatError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let mut guess = String::new();
    if let Some(v) = &self.0 {
      guess = format!("(maybe {})", v);
    }
    write!(f, "unknown binary format {}", guess)
  }
}

impl error::Error for BinaryFormatError { }

// For when binary format 0 cannot contain patch format 1
#[derive(Debug, Clone)]
pub struct IncompatibleFormats(pub Format, pub Format);

impl fmt::Display for IncompatibleFormats {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "binary format {:?} is incompatible with patch format {:?}", self.0, self.1)
  }
}

impl error::Error for IncompatibleFormats { }

// Parsable formats.
#[derive(Debug, Clone, Copy)]
pub enum Format {
  // Binary
  ELF,
  MachO32,
  MachO64,
  MachOUniversal,
  PE,

  // Patch
  Object,
  Dylib,
  DLL,
}

// Determines the format of a binary.
pub fn determine(bin: &[u8]) -> Result<Format, BinaryFormatError> {
  match &bin[0..4] {
    [0xCE, 0xFA, 0xED, 0xFE] => Ok(Format::MachO32),
    [0xCF, 0xFA, 0xED, 0xFE] => Ok(Format::MachO64),
    [0xBE, 0xBA, 0xFE, 0xCA] => Ok(Format::MachOUniversal),
    [0x7F, 0x45, 0x4C, 0x46] => Ok(Format::ELF),
    [0x4D, 0x5A, _   , _   ] =>
      // 16 bit MZ and NE binaries use the same two magic bytes, however
      // only PE has a PE header following a fixed-size MZ binary stub.
      if &bin[64..69] == [] {
        // https://stackoverflow.com/questions/1153090/how-to-detect-that-a-given-pe-file-exe-or-dll-is-64-bit-or-32-bit
        Err(BinaryFormatError(Some(String::from("DOS MZ .EXE"))))
      } else {
        Ok(Format::PE)
      }
    _ => Err(BinaryFormatError(None))
  }
}

// Determines if two given formats are "compatible", e.g. mach-o and dylib, exe and dll.
pub fn compatible(a: Format, b: Format) -> bool {
  use Format::*;

  match (a, b) {
    (ELF, Object) | (MachO32, Dylib) | (MachO64, Dylib) | (MachOUniversal, Dylib) | (PE, DLL)
      => true,
    _ => false,
  }
}