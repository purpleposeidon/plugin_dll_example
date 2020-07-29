extern crate xp_lib;

#[derive(Debug)]
struct ToCrack;

#[no_mangle]
pub fn in_dylib() {
    println!("Hello from the dynamic library!");
    println!("{:?}", xp_lib::AToughNut(ToCrack));
}
