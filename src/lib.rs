extern crate libloading;
pub extern crate serde;
pub extern crate serde_json;


// Problem Number One
// The default DLL format produces an error if it contains too many entries.
// There is an option, /bigobj, that fixes it.
// 1. The GNU linker doesn't support this.
// 2. MSVC theoretically supports it, but its linker, ld.exe, does not actually, only the compiler
//    cl.exe does. It might be possible to talk the compiler into acting as a linker the way you
//    can do with gcc, but I wasn't able to figure it out.
// 3. This leaves lldb as the only option for the linker. It is said it supports /hugeobj.
pub mod shit_ton;

// Problem Number Two
// Compiling with lldb.exe seemed incapable of handling monomorphized types.
#[derive(Debug, serde::Serialize)]
pub struct AToughNut<T>(pub T);

#[cfg(not(windows))]
static PATH: &str = "./libtemplate.so";
#[cfg(windows)]
static PATH: &str = "./template.dll";

pub fn xp_main() {
    let mut me = std::env::current_exe()
        .expect("whoami?");
    me.pop();
    std::env::set_current_dir(me)
        .expect("cd failed");

    println!("Hello from the application crate!");
    let lib = libloading::Library::new(PATH).unwrap();
    unsafe {
        let func: Result<libloading::Symbol<unsafe extern fn()>, _> = lib.get(b"in_dylib");
        println!("{:?}", func);
        let func = func.unwrap();
        func();
    }
}
