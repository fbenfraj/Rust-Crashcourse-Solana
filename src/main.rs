fn main() {
    // _say_hello();
    // _print_types();
    // _print_arrays();
    // _print_tuples();
    // _test_functions();
    // _test_mutability();
    // _print_slices();
    // _print_strings();
    // _test_conditions();
    // _test_loops();
    // _print_match();
}

fn _say_hello() {
    let a = 10;
    let b = 15;

    println!("Hello world, {} {}", a, b);
}

fn _print_types() {
    // u8 = 8 bits, u16, u32, u64, u128
    let unsigned: u8 = 10;
    let signed: i8 = -10;
    let float: f32 = 1.2;
    let letter = "c";
    let is_true: bool = true;

    println!("unsign: {}, sign: {}, float: {}", unsigned, signed, float);
    println!("letter: {}", letter);
    println!("isTrue: {}", is_true);
}

fn _print_arrays() {
    let arr: [u8; 3] = [1, 2, 3];
    // [100, 100, 100, 100, 100]
    let other_arr: [u8; 5] = [100; 5];

    println!("index: {}, length: {}", arr[0], other_arr.len());
    // Special syntax to print array
    println!("{:?}", other_arr);
}

fn _print_tuples() {
    let tuple: (u8, bool, f32) = (5, true, 2.1);
    let tuple2 = (3, 5);
    // destructuring
    let (a, b, c) = tuple;

    println!("first {}, second {}, third {}", tuple.0, tuple.1, tuple.2);
    println!("{:?}", tuple2);
    println!("first {}, second {}, third {}", a, b, c);
}

fn _test_functions() {
    println!("{}", is_even(2))
}

pub fn is_even(num: u8) -> bool {
    let digit: u8 = num % 2;
    digit == 0 // return bool
}

fn _test_mutability() {
    let mut num = 5;

    println!("Before change: {}", num);
    num = 3;
    println!("After change: {}", num);
}

fn _print_slices() {
    // array: we know the length of the array at compile time, slice we don't
    let arr = [0, 1, 2, 3];
    let slice = &arr[1..3]; // [1, 2], index 3 is not included
    borrowing_slice(arr, slice);
}

pub fn borrowing_slice(arr: [u8; 4], slice: &[u8]) {
    println!("Array: {:?}", arr);
    println!("Slice: {:?}", slice);
    println!("Slice length: {:?}", slice.len());
    println!("Slice[0]: {}, Slice[1]: {}", slice[0], slice[1]);
}

fn _print_strings() {
    let _str: &str = "Hello World!";
    let mut string: String = String::from("Bye World!");
    let slice = &string[..6]; // Again index 6 is excluded
    let _slice_length = slice.len();

    string.push('!');
    string.push_str(" D:");
    string = string.replace("Bye", "Hello");
    string = string.replace("D:", ":D");
    println!("{}", string);
}

fn _test_conditions() {
    let n = 3;

    if n > 0 {
        println!("Greater than 0");
    } else if n < 0 {
        println!("Less than 0");
    } else {
        println!("Is 0")
    }
}

fn _test_loops() {
    let mut i = 0;

    // For loops (6 is excluded here)
    for i in 0..6 {
        print!("{}", i);
    }
    println!("");

    while i < 4 {
        print!("{}", i);
        i += 1;
        if i == 3 {
            println!(" Exiting instead of printing 3");
            break; // or continue
        }
    }
    println!("");
}

fn _print_match() {
    let i = 3;

    match i {
        0 => println!("0"),
        1 | 2 => println!("1 or 2"),
        3..=4 => println!("3, 4"), // this is how you include 4
        _ => println!("No matching value found, printing default."),
    }
}
