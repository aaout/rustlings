// primitive_types3.rs
//
// Create an array with at least 100 elements in it where the ??? is.
//
// Execute `rustlings hint primitive_types3` or use the `hint` watch subcommand
// for a hint.

fn main() {
    // let a = "Hello world!! Hello world!! Hello world!! Hello world!! Hello world!! Hello world!! Hello world!! Hello world!!";
    let a = vec![0; 100];

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
        println!("a len is {}", a.len());
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed")
    }
}
