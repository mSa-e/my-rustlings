#![allow(clippy::ptr_arg)]

// TODO: Fix the compiler errors without changing anything except adding or
// removing references (the character `&`).

// Shouldn't take ownership
fn get_char(data: &String) -> char {
    // We just want a refrence because we do not want to change the data it self but to take a copy of a modified version of it.
    data.chars().last().unwrap() // returning the last char of a string
}

// Should take ownership
fn string_uppercase(mut data: String) {
    // We change the data it self ,so no need to pass a refrence just pass it and change it
    data = data.to_uppercase();

    println!("{data}");
}

fn main() {
    let data = "Rust is great!".to_string();

    get_char(&data);

    string_uppercase(data);
}
