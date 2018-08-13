fn main() {
    else_ifs();
    let_if();
    while_loop_intro();
    looping_through_collection();
}

fn else_ifs() {
    println!("----In else_ifs");

    let number = 6;

    if number % 4 == 0 {
        println!("Number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

fn let_if() {
    println!("----In let_if");

    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };

    println!("The value of number is: {}", number)
}

// fn loop_forever() {
//     println!("----In loop_forever");
//     loop {
//        println!("again!")
//     }
// }

fn while_loop_intro() {
    println!("----In repetition_with_loops");

    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number = number - 1;
    }

    // for loop alt
    // for number in (1..4).rev() {
    //      println!("{}!", number)
    // }

    println!("LIFTOFF!!!");
}

fn looping_through_collection() {
    let a = [10, 20, 30, 40, 50];
    // let mut index = 0;
    //
    // while index < 5 {
    //     println!("The value is: {}", a[index]);
    //
    //     index = index + 1
    // }

    for element in a.iter() {
        println!("The value is: {}", element);
    }
}
