#![feature(alloc_error_handler)]
#![feature(const_fn)]
#![feature(decl_macro)]
#![feature(asm)]
#![feature(global_asm)]
#![feature(optin_builtin_traits)]
#![feature(raw_vec_internals)]
#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

#[cfg(not(test))]
mod init;

extern crate alloc;

pub mod allocator;
pub mod console;
pub mod fs;
pub mod mutex;
pub mod shell;
use pi::timer::spin_sleep;
use console::kprintln;
use allocator::Allocator;
use fs::FileSystem;
use core::time::Duration;
use pi::atags;
#[cfg_attr(not(test), global_allocator)]
pub static ALLOCATOR: Allocator = Allocator::uninitialized();
pub static FILESYSTEM: FileSystem = FileSystem::uninitialized();

fn kmain() -> ! {

    // unsafe {
    //     ALLOCATOR.initialize();
    //     FILESYSTEM.initialize();
    // }
    spin_sleep(Duration::new(1,0));
    let atags = atags::Atags::get();
    for atag in atags{
        kprintln!("{:#?}",atag);
    }
    shell::shell("> ");
}
