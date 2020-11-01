use clap::Clap;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const AUTHOR: &str = env!("CARGO_PKG_AUTHORS");

#[derive(Clap)]
#[clap(version = VERSION, about = "Streamlined binary patcher.", author = AUTHOR)]
pub struct Arguments {
  #[clap(about = "a binary to patch", long_about = "\
A binary to apply a patch to. See patch's help for format
information.")]
  pub binary: String,
  #[clap(about = "a patch to apply to a binary", long_about = "\
A patch to apply to a binary. This patch should contain a function
called 'main'. After patching, this function will replace the
binary's entry point, then normal functioning will resume. This
patch should be in the binary's target platform's associated
shared object format. For example:
| Plaform | Binary | Patch |
|---------|--------|-------|
| macOS   | mach-o | dylib |
| Windows | PE     | DLL   |
| Linux   | ELF    | object|
Any platform may patch another platform's binaries.
")]
  pub patch: String,
}

pub fn parse() -> Arguments {
  Arguments::parse()
}