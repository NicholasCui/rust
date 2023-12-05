fn main() {
    let s1: &'static str = "I am a superman.";
    let s2: String = s1.to_string();
    let s3: &String = &s2;
    let s4: &str = &s2[..];
    let s5: &str = &s2[..6];

    println!("s1: {}", s1);
    println!("s2: {}", s2);
    println!("s3: {}", s3);
    println!("s4: {}", s4);
    println!("s5: {}", s5);
}
