use std::convert::From;

#[derive(Debug)]
#[allow(dead_code)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(v: i32) -> Self {
        Number{ value: v}
    }
}

// 6.1
#[allow(dead_code)]
fn from_and_into() {
    let n1 = Number::from(24);
    println!("From number: {:?}", n1);

    let n2: Number = 48.into();
    println!("Into number: {:?}", n2);
}

// 8.5.1.3
#[allow(dead_code)]
fn pointers_ref() {
    let reference = &4;
    match reference {
        &val => println!("Got a value via deconstructing: {:?}", val),
    }
    match *reference {
        val => println!("Got a value via dereferencing: {:?}", val),
    }

    let value = 5;
    let mut mut_value = 6;
    match value {
        ref r => println!("Got a reference to a value: {:?}", r),
    }
    match mut_value {
        ref mut r => {
            *r += 10;
            println!("mut_value=6 plus 10 is {:?}", r);
        },
    }
}

// 8.6
#[allow(dead_code)]
fn if_let() {
    let opt = Some(7);
    match opt {
        Some(i) => println!("Got value i: {}", i),
        _ => {},
    }
    if let Some(i) = opt {
        println!("Got same value {}, but cleaner", i);
    }
}

// 9.2
#[allow(dead_code)]
fn closure() {
    fn function(i: i32) -> i32 {i + 1}
    let closure_annotated = |i: i32| -> i32 {i + 1};
    let closure_inferred = | i | i + 1;

    let i = 1;
    println!("Fuction: {}", function(i));
    println!("closure_annotated: {}", closure_annotated(i));
    println!("closure_inferred: {}", closure_inferred(i));

    let one = || 1;
    println!("closure return one: {}", one());
}

fn main() {
    // from_and_into();
    // pointers_ref();
    if_let();
    // closure();
}
