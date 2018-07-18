/*****************************************************
             PROJECT  : hpc_allocator_rust
             VERSION  : 0.1.0-dev
             DATE     : 05/2018
             AUTHOR   : Valat Sébastien
             LICENSE  : CeCILL-C
*****************************************************/

//we want to avoid to take special language things inside the allocator
<<<<<<< HEAD
<<<<<<< HEAD
#![feature(lang_items,core,libc)]
#![feature(panic_implementation)]
#![feature(core_intrinsics)]
#![no_std]
#![allow(dead_code)]
#![feature(asm)]

//load modules
mod common;
mod registry;
mod portability;
mod chunk;
mod mmsource;
=======
=======
>>>>>>> d43d130... First commit
#![feature(lang_items, start,core)]
#![no_std]

//load modules
mod common;
<<<<<<< HEAD
>>>>>>> d43d130... First commit
=======
>>>>>>> d43d130... First commit

#[cfg(not(test))]
pub mod export;
