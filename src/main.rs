use std::fs::{self, File};
use std::error;
use std::path::Path;
use std::io::prelude::*;
use goblin::*;

//mod formats;
mod zero_point;
mod arguments;

/*fn main() -> Result<(), Box<dyn error::Error>> {
    let args = arguments::parse();

    // Binary parsing
    println!("Loading binary {}", args.binary);
    let mut binary_file = File::open(args.binary)?;
    let mut binary_data = Vec::new();
    binary_file.read_to_end(&mut binary_data)?;

    let binary_format = formats::determine(&binary_data)?;
    println!("Determined binary format {:?}", &binary_format);

    // Patch parsing
    println!("Loading patch {}", args.patch);

    let mut patch_file = File::open(args.patch)?;
    let mut patch_data = Vec::new();
    patch_file.read_to_end(&mut patch_data)?;

    let patch_format = formats::determine(&patch_data)?;
    println!("Determined binary format {:?}", &patch_format);

    if !formats::compatible(binary_format, patch_format) {
      println!("Formats {:?} and {:?} are incompatible.", binary_format, patch_format);
      return Err(Box::new(formats::IncompatibleFormats(binary_format, patch_format)));
    }

    Ok(())
}
*/

fn main() -> Result<(), Box<dyn error::Error>> {
  let args = arguments::parse();
  let binary_file = Path::new(&args.binary);
  let binary_data = fs::read(binary_file)?;

  let out = match Object::parse(&binary_data)? {
    Object::Mach(mach::Mach::Binary(mut obj)) => {
      obj.entry = 0;
      obj
    },
    _ => unimplemented!("mach-o only"),
  };

  let mut out_file = File::create("output")?;
  out_file.write_all(out)?;

  println!("{:?}", zero_point::point(&binary_data, 500));

  Ok(())
}