// build-fail
// ignore-emscripten no asm! support
// The error message differs slightly between LLVM versions
// min-llvm-version: 13.0
// Regression test for #69092
// ignore-e2k64

#![feature(llvm_asm)]
#![allow(deprecated)] // llvm_asm!

fn main() {
    unsafe { llvm_asm!(".ascii \"Xen\0\""); }
    //~^ ERROR: expected string
}
