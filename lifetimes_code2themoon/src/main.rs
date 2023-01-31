#[derive(Debug)]
struct SimpleStruct<'a> {
    num: &'a i32,
}

fn print_some_struct(the_struct: &SimpleStruct) {
    println!("{:?}", the_struct);
}

// fn mutate_struct(the_struct: &mut SimpleStruct) {
//     the_struct.num += 5;
// }

fn biggest<'a>(a : &'a SimpleStruct, b: &'a SimpleStruct) -> &'a SimpleStruct {
    if a.num > b.num {
        a
    } else {
        b
    }
}

fn main() {
    let b: SimpleStruct;
    {
        let num = 3;
        b = SimpleStruct { num: &num };
    }

    let mut some_struct = SimpleStruct { num : &3 };
    let bigger: &SimpleStruct;
    let other_struct = SimpleStruct{ num: &10};
    bigger = biggest(&some_struct, &other_struct);

    print_some_struct(bigger);

    print_some_struct(&some_struct);
    print_some_struct(&some_struct);
}
