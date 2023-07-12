// build-fail
// dont-check-compiler-stderr
// ignore-emscripten
// ignore-e2k64 we have another message from as

#![feature(llvm_asm)]
#![allow(deprecated)] // llvm_asm!

fn main() {
    unsafe {
        llvm_asm!("nowayisthisavalidinstruction"); //~ ERROR instruction
    }
}
