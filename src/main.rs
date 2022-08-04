#![no_std]
#![no_main]
#![feature(panic_info_message)]
mod lang_items;
mod sbi;
#[macro_use]
mod console;


use core::arch::global_asm;
global_asm!(include_str!("entry.asm"));
fn clear_bbs(){
    extern "C"{
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a|{
        unsafe {(a as *mut u8).write_volatile(0)}
    })
}


#[no_mangle]
pub fn rust_main(){
    clear_bbs();
    println!("
 _____       _     _     _ _    ____
|  __ \\     | |   | |   (_) |  / __ \\
| |__) |__ _| |__ | |__  _| |_| |  | |___
|  _  // _` | '_ \\| '_ \\| | __| |  | / __|
| | \\ \\ (_| | |_) | |_) | | |_| |__| \\__
|_|  \\_\\__,_|_.__/|_.__/|_|\\__|\\____/|___/");
    panic!("Shutdown machine!");
}