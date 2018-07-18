
/*****************************************************
             PROJECT  : hpc_allocator_rust
             VERSION  : 0.1.0-dev
             DATE     : 05/2018
             AUTHOR   : Valat Sébastien
             LICENSE  : CeCILL-C
*****************************************************/

///Export the C API of the allocator

// Pull in the system libc library for what crt0.o likely requires
extern crate libc;

<<<<<<< HEAD
<<<<<<< HEAD
use registry::registry::RegionRegistry;
use mmsource::cached::CachedMMSource;
use common::traits::{MemorySourcePtr};
use core::panic::PanicInfo;
use core::intrinsics;
use chunk::huge::HugeChunkManager;
use chunk::medium::manager::MediumChunkManager;
use common::consts::*;
use common::shared::SharedPtrBox;

// Entry point for this program
#[no_mangle]
pub extern fn malloc(size: libc::size_t) -> *mut libc::c_void {
	let mut registry = RegionRegistry::new();
	let mut mmsource = CachedMMSource::new_default(Some(SharedPtrBox::new_ref_mut(&mut registry)));
    let pmmsource = MemorySourcePtr::new_ref_mut(&mut mmsource);
	let mut huge_manager = HugeChunkManager::new(pmmsource.clone());
    let mut medium_manager = MediumChunkManager::new(true, Some(pmmsource.clone()));

	if size < 1024 {
        medium_manager.malloc(size,BASIC_ALIGN,false).0 as *mut libc::c_void
    } else {
        huge_manager.malloc(size,BASIC_ALIGN,false).0 as *mut libc::c_void
    }
=======
=======
>>>>>>> d43d130... First commit
// Entry point for this program
#[no_mangle]
pub extern fn malloc(size: libc::size_t) -> *mut libc::c_void {
	let ret;
	unsafe {
		ret = libc::mmap(0 as *mut libc::c_void, size*4096,libc::PROT_READ | libc::PROT_WRITE, libc::MAP_ANON | libc::MAP_PRIVATE, 0,0);
		panic!("Hello");
	}
	ret
<<<<<<< HEAD
>>>>>>> d43d130... First commit
=======
>>>>>>> d43d130... First commit
}

// These functions and traits are used by the compiler, but not
// for a bare-bones hello world. These are normally
// provided by libstd.
#[lang = "eh_personality"] 
<<<<<<< HEAD
<<<<<<< HEAD
#[no_mangle]
pub extern fn eh_personality() {}

#[panic_implementation]
fn panic(_info: &PanicInfo) -> ! {
    unsafe { intrinsics::abort() }
}
=======
=======
>>>>>>> d43d130... First commit
extern fn eh_personality() {}

#[lang = "panic_fmt"] 
fn panic_fmt() -> ! { loop {} }
<<<<<<< HEAD
>>>>>>> d43d130... First commit
=======
>>>>>>> d43d130... First commit

#[lang = "eh_unwind_resume"]
#[no_mangle]
pub extern fn rust_eh_unwind_resume() {
}
<<<<<<< HEAD
<<<<<<< HEAD
=======
=======
>>>>>>> d43d130... First commit

#[no_mangle]
pub extern fn rust_begin_unwind() {

<<<<<<< HEAD
}
>>>>>>> d43d130... First commit
=======
}
>>>>>>> d43d130... First commit
