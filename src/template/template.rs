extern crate xp_lib;
extern crate serde;

#[derive(Debug, Default, serde::Serialize)]
struct ToCrack {
    x: i32,
}

#[no_mangle]
pub fn in_dylib() {
    println!("Hello from the dynamic library!");
    let val = &xp_lib::AToughNut(ToCrack::default());
    println!("{:?}", val);
    println!("{}", xp_lib::serde_json::to_string(val).unwrap());
}
