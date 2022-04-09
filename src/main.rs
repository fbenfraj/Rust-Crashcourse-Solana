use std::collections::HashMap;

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
    // _print_struct();
    // _print_enum();
    // _print_vector();
    // _print_hashmap();
    // _print_options();
    // _print_results();
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

struct _Bird {
    _name: String,
    _attack: u64,
}

impl _Bird {
    fn _print_name(&self) {
        println!("{}", self._name)
    }
}

fn _print_struct() {
    let name = String::from("Bird");
    let bird = _Bird {
        _name: name,
        _attack: 5,
    };

    bird._print_name();
    println!("{} {}", bird.can_fly(), bird.is_animal())
}

// trait = interface
trait Animal {
    fn can_fly(&self) -> bool;
    fn is_animal(&self) -> bool {
        true
    }
}

impl Animal for _Bird {
    fn can_fly(&self) -> bool {
        true
    }
    fn is_animal(&self) -> bool {
        false
    }
}

#[derive(Debug)]
enum MyEnum {
    _A,
    _B(i32),
    _C { x: i32, y: i32 },
}

fn _print_enum() {
    let a: MyEnum = MyEnum::_A;
    let b: MyEnum = MyEnum::_B(5);
    let c: MyEnum = MyEnum::_C { x: 10, y: 20 };

    println!("{:?}", a);
    println!("{:?}", b);
    println!("{:?}", c);

    if let MyEnum::_B(val) = b {
        println!("{}", val);
    }

    if let MyEnum::_C { x, y } = c {
        println!("{} {}", x, y);
    }
}

fn _print_vector() {
    // Vector's size can be dynamically altered
    let mut vec: Vec<i64> = vec![1, 2, 3, 4, 5];

    vec.len();
    vec[0];
    vec.push(6);
    vec.remove(0);
    println!("{:?}", vec);
}

fn _print_hashmap() {
    let mut map = HashMap::new();

    map.insert(0, "Hi");
    map.insert(1, "Hi again");
    println!("{:?}", map);

    // map.get returns an Option: the fields are Some and Null (basically success and failiure)
    match map.get(&0) {
        Some(value) => println!("{}", value),
        _ => println!("Doesn't exist in the map"),
    }

    match map.get(&2) {
        Some(value) => println!("{}", value),
        _ => println!("Doesn't exist in the map"),
    }

    map.remove(&0);
    println!("{:?}", map);
}

// An Option is an enum that has two types: None (failiure, throws an exception)
// or Some(value), a tuple struct that wraps a value with type T
fn _divide_options(dividend: i32, divisor: i32) -> Option<i32> {
    if dividend % divisor != 0 {
        None
    } else {
        Some(dividend / divisor)
    }
}

fn _print_options() {
    let divide1: Option<i32> = _divide_options(4, 2);
    let _divide2: Option<i32> = _divide_options(2, 3);

    // Unwrapping a "Some" variant will extract the value wrapped
    println!("{:?} unwraps to {}", divide1, divide1.unwrap());

    // Unwrapping a "None" variant will throw an exception, called "panic"
    //println!("{:?} unwraps to {}", _divide2, _divide2.unwrap());
}

#[derive(Debug)]
enum MyError {
    _Error1,
}

// A Result returns an Err (enum with an error code) ok an Ok(value) (wrapper with a value)
fn _divide_results(dividend: i32, divisor: i32) -> Result<i32, MyError> {
    if dividend % divisor != 0 {
        Err(MyError::_Error1)
    } else {
        Ok(dividend / divisor)
    }
}

fn _print_results() {
    let _divide = _divide_results(4, 2);

    //We can use a match but it might get really nested
    match _divide {
        Ok(v) => println!("{}", v),
        Err(v) => println!("{:?}", v),
    }

    // We can also use an if statement
    // if _divide.is_ok() {
    //     println!("{}", _divide.unwrap());
    // }

    // Or we could just unwrap it
    // println!("{}", _divide.unwrap());

    // This one returns 100 instead of Error
    // println!("{}", _divide.unwrap_or(100));

    // How to expect/ignore an error and log it
    // let res = _divide.expect("We crashed.");
    // println!("{}", res);
}
