// TODO: Fix the function body without changing the signature.
fn square(num: i32) -> i32 {
    num * num

    /*
        The return from num*num; to num*num only

        and there is other commonly used way by adding the return keyword that becomes like return num*num;

        But we used the First way because it's a rusty way ;>
    */
}

fn main() {
    let answer = square(3);
    println!("The square of 3 is {answer}");
}
