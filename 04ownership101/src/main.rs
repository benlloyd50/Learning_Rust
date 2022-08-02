// ~~~~~~~~~~~~~~~~~~~~~~~~Slicing example
fn main() {
    let test_word = String::from("Hello world and all who inhabit it");
    let ans = first_word(&test_word);
    println!("{} <== {}", ans, test_word);

    let ans = first_word("Some very literal string");
    println!("{}", ans);

    let a = [99, 231, 2312, 423, 543];
    let some_length = a.len() / 2;  //Decimals are truncated here
    let some_of_a = &a[..some_length];
    println!("{some_of_a:#?}, which is {some_length}");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// ~~~~~~~~~~~~~~~~~~~~~~Borrowing example
// fn main() {
//     let s1 = gives_ownership();         // gives_ownership moves its return
//                                         // value into s1

//     let s2 = String::from("hello");     // s2 comes into scope

//     let s3 = takes_and_gives_back(s2);  // s2 is moved into
//     println!("{}", s1);                  // takes_and_gives_back, which also;'
//                                         // moves its return value into s3
// } // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
//   // happens. s1 goes out of scope and is dropped.

// fn gives_ownership() -> String {             // gives_ownership will move its
//                                              // return value into the function
//                                              // that calls it

//     let some_string = String::from("yours"); // some_string comes into scope

//     some_string                              // some_string is returned and
//                                              // moves out to the calling
//                                              // function
// }

// // This function takes a String and returns one
// fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
//                                                       // scope

//     a_string  // a_string is returned and moves out to the calling function
// }

//~~~~~~~~~~~~~~~~~~~~~~~~~~~ Copying example
// fn main() {
//     copy_example2();
// }

// fn copy_example1() {
//     let mut s = String::from("hello");
//     println!("{s}");

//     s.push_str(", babers!");
//     println!("{}", s);

//     let s2 = s; //s is freed since s2 takes it
//     println!("{s2}");
// }

// fn copy_example2() {
//     let s1 = String::from("Hello");
//     let s2 = s1.clone();

//     println!("{s1} <-> {s2}: 2 values with 2 pointers");
// }