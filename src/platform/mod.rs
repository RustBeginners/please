mod rust;
use self::rust::Rust;

pub trait Platform {
    fn probe(&self) -> bool;

    fn build(&self) -> bool;

    fn run(&self) -> bool;
}

pub fn probe() -> Option<Box<Platform>> {
    let rust = Rust {};
    if rust.probe() {
        Some(Box::new(rust))
    } else {
        None
    }
}
