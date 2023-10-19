fn test_panic() {
    let val: Option<u8> = None;
    val.expect("you mad?");

    // this will never happen
    unreachable!();
}

fn debug_macro() {
    let value = 5;
    let another = 10;
    dbg!(value);
    dbg!(value, another);
}

fn test_result() {

}

#[derive(Debug)]
struct TestError;

impl Display for TestError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "hello")
    }
}

// Error: Debug + Display
impl Error for TestError {

}
