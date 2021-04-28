fn main() {
    println!("Hello, world!");

    let a = 10;
    let b = 20;
    let c = a + b;

    println!("c is {}", c);


    let mut x = 10;
    x += 5;
    println!("z is {}", x);

    let flag :bool = true;
    println!("flag value set to {}", flag);

    let tup = (1, "sakthi", 100.0);
    let first = tup.0;
    let second = tup.1;
    let third = tup.2;
    println!("tuple values are: {}, {}, {}", first, second, third);

    let (f, s, t) = tup;
    println!("destruction of tuple: {}, {}, {}", f, s, t);

    let arr1 = [1, 2, 3];
    println!("{:?}", arr1);

    let mut arr2 = ["sakthi", "saravanan", "shanmugam"];
    println!("{:?}", arr2);
    arr2[2] = "s";
    println!("{:?}", arr2);
    println!("{}", arr2[0]);

    let array = [100, 200, 300, 400, 500];
    let slice = &array[0..3];
    println!("slices: {:?}", slice);

    let strings = "sakthi saravanan shanmugam";
    let dad = &strings[16..26];
    println!("dad: {}", dad);

    say_hello(dad);
    let result = sqrt(5);
    println!("SQRT: {}", result);

    learn_control_statements();
}

fn say_hello(name: &str) {
    println!("hello, {} !", name);
}

fn sqrt(num: i32) -> i32 {
    return num*num;
} 

fn learn_control_statements() {
    let flag = true;
    if flag {
        println!("if - block - true");
    } else {
        println!("else - block - false");
    }

    let is_confirmed = true;
    let is_active = true;
    match (is_confirmed, is_active) {
        (true, true) => println!("both true"),
        (false, false) => {
            println!("both false")
        },
        _ => {}
    }
}