#![feature(intrinsics)]
#![feature(no_core)]
#![feature(rustc_attrs)]

#![no_core]
#![crate_name = "foo"]

//@ has 'foo/fn.abort.html'
//@ has - '//pre[@class="rust item-decl"]' 'pub fn abort() -> !'
#[rustc_intrinsic]
pub fn abort() -> !;
//@ has 'foo/fn.unreachable.html'
//@ has - '//pre[@class="rust item-decl"]' 'pub unsafe fn unreachable() -> !'
#[rustc_intrinsic]
pub unsafe fn unreachable() -> !;
