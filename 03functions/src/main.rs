fn main() {
    another_function(five());
}

fn another_function(x: i32) {
    println!("Im in a fn {x}");
}

fn five() -> i32 {
    5
}