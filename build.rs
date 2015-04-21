use std::process::Command;
use std::env;
use std::fs;
use std::path::Path;

#[cfg(unix)]
fn main() {
	let freeimage_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
	let freeimage_native_dir = Path::new(&freeimage_dir).join("FreeImage");
	Command::new("make")
		.current_dir(&freeimage_native_dir)
		.status().unwrap();
	let out_dir = env::var("OUT_DIR").unwrap();
	let dest_path = Path::new(&out_dir).join("libfreeimage.a");
	fs::copy(freeimage_native_dir.join("Dist/libfreeimage.a"),dest_path).unwrap();
	println!("cargo:rustc-flags= -L native={}",out_dir);
}
