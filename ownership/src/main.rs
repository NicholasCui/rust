fn variables() {
    let a = 10u32;
    let b = &a;
    let c = &&&&a;
    let d = &b;
    let e = b;

    println!("{a}");
    println!("{b}");
    println!("{c}");
    println!("{d}");
    println!("{e}");
}

fn reference_variables() {
    let s1 = String::from("I am a superman.");
    let s2 = &s1;
    let s3 = &&&&&s1;
    let s4 = &s2;
    let s5 = s2;

    println!("{s1}");
    println!("{s2}");
    println!("{s3}");
    println!("{s4}");
    println!("{s5}");
}


fn foo(s: &String) {
    println!("in fn foo: {s}");
}
fn main() {
    // variables();
    // reference_variables();

    // 1
    // let mut a = 10u32;
    // let b = &mut a;
    // *b = 20;
    // println!("{b}");

    // 2
    // let mut a = 10u32;
    // let b = &mut a;
    // *b = 20;

    // println!("{b}");
    // println!("{a}");

    // 2 error
    // let mut a = 10u32;
    // let b = &mut a;
    // *b = 20;

    // println!("{a}");
    // println!("{b}");

    // 3 multiple mutable references
    let mut a = 10u32;
    let mut b = &mut a;
    let mut c = &mut b;
    let mut d = &mut c;

    println!("{d}");

    // reference error
    // **d = 20

    // reference correct
    ***d = 20;
    println!("{a}");

    let s1 = String::from("I am a superman.");
    foo(&s1);
    println!("{s1}");
}
