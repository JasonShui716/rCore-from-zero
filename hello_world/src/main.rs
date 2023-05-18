#![no_std]
#![no_main]

mod lang_items;
mod console;

use core::arch::global_asm;
global_asm!(include_str!("entry.asm"));