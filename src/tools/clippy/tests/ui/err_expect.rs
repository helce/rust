// run-rustfix

struct MyTypeNonDebug;

#[derive(Debug)]
struct MyTypeDebug;

fn main() {
    let test_debug: Result<MyTypeDebug, u32> = Ok(MyTypeDebug);
    test_debug.err().expect("Testing debug type");

    let test_non_debug: Result<MyTypeNonDebug, u32> = Ok(MyTypeNonDebug);
    test_non_debug.err().expect("Testing non debug type");
}
