#![feature(type_alias_impl_trait)]
#![allow(dead_code)]

// check-pass
// known-bug #53092 #90409

type Bug<T, U> = impl Fn(T) -> U + Copy;

union Moo {
    x: Bug<u8, ()>,
    y: (),
}

const CONST_BUG: Bug<u8, ()> = unsafe { Moo { y: () }.x };

fn make_bug<T, U: From<T>>() -> Bug<T, U> {
    |x| x.into()
}

fn main() {
    CONST_BUG(0);
}
