fn main() {
    moving();
    shadowing();
    types();
    control();
    expressions();
}

fn moving() {
    let some_str = String::from("Hi");
    println!("{}", some_str);

    let some_other_str = some_str;
    println!("{}", some_other_str);

    // Illegal because move occurred. some_str is no longer valid.
    // println!("{}", some_str);
}

fn shadowing() {
    let some_int = 5;

    println!("{}", some_int);

    // Illegal because a wasn't declared as mutable
    // some_int = 6;

    // However, this is allowed because of "shadowing"
    let some_int = 6;
    println!("New value of some_int: {}", some_int);
}

fn types() {
    let some_int: u32 = 5;
    let some_signed_int: i32 = -5;
    let some_float: f32 = 5.5;
    let some_bool: bool = false;
    let some_char: char = 'g';
    let some_str_literal: &str = "hello";
    let some_str: String = String::from("hello");
    let some_tuple: (u32, i32, f64) = (5, -5, -5.5);
    let some_array:[u32; 3] = [1, 2, 3];

    println!("Types: {} {} {} {} {} {} {}", some_int, some_signed_int, some_float,
             some_bool, some_char, some_str_literal, some_str);
    println!("More types: {} {}", some_tuple.1, some_array[2]);
}

fn control() {


    let mut i: u64 = 0;
    let goal = 99;

    // infinite loop. Equivalent to "while True:"
    loop {
        if i == goal {
            break;
        }
        i += 1
    }

    // Loop from [0, 99]
    for _j in 0..100 {
        // println!("{}", _j);
    }

    // Loop from [99, 0]
    for _j in (0..100).rev() {
        // println!("{}", _j);
    }

    // Loop from [0, 100]
    for _j in 0..=100 {
        // println!("{}", _j);
    }

    i = 0;
    while i < goal {
        i += 1
    }

    if i % 2 == 0 {
        println!("Even!");
    } else if i % 2 == 1 {
        println!("Odd!");
    } else {
        println!("Impossible!");
    }

    match i % 2 {
        0 => println!("Even!"),
        1 => println!("Odd!"),
        _ => println!("Unmöglich!")
    }

    // For-each looping
    for _num in vec![1, 2, 3, 4, 5].iter() {
        // println!("{}", _num);
    }

    for _num in [1, 2, 3].iter() {
        // println!("{}", _num)
    }
}

fn expressions() {
        let mut i = 0;

        let cool_assignment = if (i % 2) == 0 { "even" } else { "odd" };
        println!("The number is {}", cool_assignment);

        // You can also use break to return stuff!
        let result = loop {
            if i == 20 {
                break i + 1
            } else {
                i += 1
            }
        };
        assert_eq!(result, 21);
}

