// run-pass
// compile-flags: -Ccodegen-units=1 -Cinline-threshold=0 -Clink-dead-code -Copt-level=0 -Cdebuginfo=2
// ignore-e2k64 NIY

// Make sure LLVM does not miscompile this.

fn make_string(ch: char) -> String {
    let mut bytes = [0u8; 4];
    ch.encode_utf8(&mut bytes).into()
}

fn main() {
    let ch = '😃';
    dbg!(ch);
    let string = make_string(ch);
    dbg!(string);
}
