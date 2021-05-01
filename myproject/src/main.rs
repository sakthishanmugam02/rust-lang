// tuple struct - useful when defining unit structs to avoid acciendental use of invalid data type
struct Meter (i8);

// unit struct - useful when defining methods. struct without any members

struct Temp;

enum Time {
    HHMMSS {
        hh: i8,
        mm: i8, 
        ss: i8
    }
}

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

    loop {
        println!("test loop");
        break;
    }

    let mut cond = true;
    while cond {
        println!("{}", cond);
        cond = false;
    }
    for value in array.iter() {
        println!("array values - {}", value);
    }

    enum TestEnum {
        Test1,
        Test2
    }

    let _value1 = TestEnum::Test2;
    let value = TestEnum::Test1;
    match value {
        TestEnum::Test1 => println!("Test1"),
        TestEnum::Test2 => println!("Test2")
    }

    enum DateFormat {
        DDMMYYYY(i16, i16, i16),
        DDMM(i16, i16)
    }

    let mut date : DateFormat;
    date = DateFormat::DDMMYYYY(01, 12, 2000);
    match date {
        DateFormat::DDMMYYYY(dd, mm, yyyy) => println!("{}, {}, {}", dd, mm, yyyy),
        DateFormat::DDMM(dd, mm) => println!("{}, {}", dd, mm)
    }
    date = DateFormat::DDMM(01, 12);
    match date {
        DateFormat::DDMMYYYY(dd, mm, yyyy) => println!("{}, {}, {}", dd, mm, yyyy),
        DateFormat::DDMM(dd, mm) => println!("{}, {}", dd, mm)
    }

    struct Student {
        name: String,
        age: i8
    }

    let sakthi = Student {
        name: "sakthi".to_string(),
        age: 30
    };

    println!("my name is {}, and am {} years old", sakthi.name, sakthi.age);

    let side = Meter(2);

    area_of_square(side);

    enum Time {
        HHMMSS {
            hh: i8,
            mm: i8, 
            ss: i8
        }
    }
    
    let now = Time::HHMMSS{hh: 10,mm: 48,ss: 44,};
    match now {
        Time::HHMMSS{hh, mm, ss} => println!("Time is {}:{}:{}", hh, mm, ss)
    }

    struct ConcreteType {
        name: String
    }

    impl ConcreteType {
        fn new() -> ConcreteType {
            return ConcreteType {
                name: "".to_string(),
            }
        }
        fn modify(&mut self, name: String) {
            self.name = name;
        }
        fn hi(&self) {
            println!("hello {}", self.name);
        }
    }

    let mut method_test = ConcreteType::new();
    method_test.modify("sakthi".to_string());
    method_test.hi();
}

fn area_of_square (side : Meter) {
    println!("area of square: {}", side.0 * side.0);
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

    //println!("{:?}", Temp);
}