#![no_std]
#![no_main]

mod lang_items;
mod console;

#[no_mangle]
fn main() {
    println!("Hello, world!");
}
