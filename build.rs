// see http://doc.crates.io/build-script.html
#![feature(vec_push_all)]
#![feature(fs_walk)]

use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

trait ExpectSuccess {
    fn expect_success(&mut self);
}
impl ExpectSuccess for Command {
    fn expect_success(&mut self){
        let result = self.status().unwrap();
        if !result.success() {panic!("command executed unsuccessfully");}
    }
}

fn assemble_file(in_path: &Path, out_dir: &str) -> String {
    let base_name = in_path.file_stem().unwrap().to_str().unwrap();
    let out_file = format!("{}/{}.o", out_dir, base_name);
    Command::new("arm-none-eabi-as")
        .arg(in_path.as_os_str())
        .arg("-Isrc")
        .arg("-o").arg(&out_file)
        .expect_success();
    
    out_file
}

fn compile_c_file(in_path: &Path, out_dir: &str) -> String {
    let base_name = in_path.file_stem().unwrap().to_str().unwrap();
    let out_file = format!("{}/{}.o", out_dir, base_name);
    Command::new("arm-none-eabi-gcc")
        .arg(in_path.as_os_str())
        .arg("-Isrc")
        .arg("-c").arg("-Os")
        .arg("-mthumb").arg("-mcpu=cortex-m4")
        .arg("-ffunction-sections")
        .arg("-o").arg(&out_file)
        .expect_success();
    
    out_file
}

fn archive_objects(objects: &Vec<String>, out_dir: &str, lib_name: &str) {
    let out_file = format!("{}/lib{}.a", out_dir, lib_name);
    
    let mut args : Vec<String> = Vec::new();
    args.push("crs".to_string());
    args.push(out_file.to_string());
    args.push_all(&objects);
    
    Command::new("arm-none-eabi-ar")
        .args(&args)
        .expect_success();
}

// run src/*.s through the assembler, build it into a static lib,
// and tell rust to link against it.
fn assemble_sources(src_dir: &str, out_dir: &str, lib_name: &str) {
    let mut objects : Vec<String> = Vec::new();
    
    for entry in fs::walk_dir(&Path::new(src_dir)).unwrap() {
        let path = entry.unwrap().path();
        if path.extension() == Some("s".as_ref())
        || path.extension() == Some("S".as_ref()) {
            objects.push(assemble_file(path.as_ref(), out_dir));
        } 
        if path.extension() == Some("c".as_ref()) {
            objects.push(compile_c_file(path.as_ref(), out_dir));
        }
    }
    
    archive_objects(&objects, out_dir, lib_name);

    println!("cargo:rustc-link-lib=static={}", lib_name);
}

fn copy_linker_scipts(in_dir: &str, out_dir: &str) {
    let out_path = Path::new(out_dir);
    
    for entry in fs::read_dir(Path::new(in_dir)).unwrap() {
        let ld_script = entry.unwrap();
        fs::copy(ld_script.path(), out_path.join(ld_script.file_name())).unwrap();
    }
}

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    
    assemble_sources("src", &out_dir, "arm_embedded_rt");
    copy_linker_scipts("ld", &out_dir);
    
    println!("cargo:rustc-link-search=native={}", out_dir);
}