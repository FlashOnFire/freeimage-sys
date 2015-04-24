#![feature(convert)]
use std::process::Command;
use std::env;
use std::fs;
use std::path::Path;
use std::ffi::OsString;

#[cfg(macos)]
fn main() {
	let freeimage_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
	let freeimage_native_dir = Path::new(&freeimage_dir).join("FreeImage");
	let xcode_select_out = OsString::from_bytes(
            Command::new("xcode-select")
                .arg("-print-path")
                .output().unwrap()
		        .stdout
        ).unwrap().into_string().unwrap();
    let xcode_path = xcode_select_out.lines().next().unwrap().to_string();
    let sdks_path = Path::new(&xcode_path).join("Platforms/MacOSX.platform/Developer/SDKs");
    let last_sdk_entry = match fs::read_dir(&sdks_path){
        Ok(sdks) => sdks.last().unwrap().unwrap(),
        Err(_) => panic!("Couldn't find SDK at {}, probably xcode is not installed",sdks_path.to_str().unwrap())
    };
    
    let sdk = last_sdk_entry.path().as_path().file_stem().unwrap().to_str().unwrap().to_string();
    if sdk.contains("MacOSX"){
        let version = &sdk[6..];
        Command::new("make")
		    .current_dir(&freeimage_native_dir)
		    .env("MACOSX_SDK",version)
		    .arg("-j4")
		    .status().unwrap();
	    let out_dir = env::var("OUT_DIR").unwrap();
	    let dest_path = Path::new(&out_dir).join("libfreeimage.a");
	    fs::copy(freeimage_native_dir.join("Dist/libfreeimage.a"),dest_path).unwrap();
	    println!("cargo:rustc-flags= -L native={}",out_dir);
        
    }else{
        panic!("Couldn't find SDK at {}, probably xcode is not installed",sdks_path.to_str().unwrap())
    }
}

#[cfg(not(macos))]
fn main() {
	let freeimage_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
	let freeimage_native_dir = Path::new(&freeimage_dir).join("FreeImage");
    Command::new("make")
	    .current_dir(&freeimage_native_dir)
	    .arg("-j4")
	    .status().unwrap();
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("libfreeimage.a");
    fs::copy(freeimage_native_dir.join("Dist/libfreeimage.a"),dest_path).unwrap();
    println!("cargo:rustc-flags= -L native={}",out_dir);
}
