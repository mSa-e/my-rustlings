// This store is having a sale where if the price is an even number, you get 10
// Rustbucks off, but if it's an odd number, it's 3 Rustbucks off.
// Don't worry about the function bodies themselves, we are only interested in
// the signatures for now.

fn is_even(num: i64) -> bool {
    // If even it will return true (10 Rustbucks off ;>) ,but if not will return false and only 3 Rustbucks off ;<
    num % 2 == 0
}

// TODO: Fix the function signature.
fn sale_price(price: i64) -> i64 {
    // Inserting the return dataType (which is i64)
    if is_even(price) {
        price - 10
    } else {
        price - 3
    }
}

fn main() {
    let original_price = 51;
    println!("Your sale price is {}", sale_price(original_price));
}
