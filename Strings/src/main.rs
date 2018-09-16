fn main() {
    let mut s = String::new();
    let data = "initial contents";
    s = "initial contents".to_string();
    s.push_str("bar");//append "bar" into s
    println!("s is {}", s);
    let s2 = "bar";
    s.push_str(s2);
    println!("s2 is {}", s2);
    println!("s is {}", s);
    s.push('*');
    s.push('-');
    s.push('l');
    println!("s is {}", s);
    let s3 = s + &s2;// Note s has been moved here and can no longer be used
    println!("s is {}", s3);
    println!("s2 is {}", s2);
    //otra forma de concatenation is
    let s4 = format!("palabras {} + otra palabra {} y {}", s2, s2, s3);
    println!("s4 is {}", s4);
}
