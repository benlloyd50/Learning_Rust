use std::io;

fn main() {
    //Tuple example
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {y}");

    //Array example
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("a is {a:#?}") //:#? is pretty print modifier


}

fn index_array_example() {
    let a = [1, 2, 3, 4, 5];
    println!("Please enter an array index");
    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index is not a number");

    let element = a[index];
    println!("{element:#?}")
}