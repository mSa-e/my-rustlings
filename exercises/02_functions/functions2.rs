// TODO: Add the missing type of the argument `num` after the colon `:`.
fn call_me(num: i32) {
    // Specifying the data type of the arguments
    for i in 0..num {
        println!("Ring! Call number {}", i + 1); // Not a returning function
    }
}

fn main() {
    call_me(3);
}
