extern crate plugin_dll_example_lib;
extern crate serde;

#[derive(Debug, Default, serde::Serialize)]
struct ToCrack {
    x: i32,
}

#[no_mangle]
pub fn in_dylib() {
    println!("Hello from the dynamic library!");
    let val = &plugin_dll_example_lib::AToughNut(ToCrack::default());
    println!("{:?}", val);
    println!("{}", plugin_dll_example_lib::serde_json::to_string(val).unwrap());
}
