
fn main() {
    // Example: Calculate the 10th Fibonacci number
    let mut s = String::from("hello");
    println!("{s}");
    {
        let r1 = &mut s;
        r1.push_str("sdss");
    } // r1 goes out of scope here, so we can make a new reference with no problems.
    println!("{s}");
    
    let r2 = &mut s;
    println!("{s}");
}

fn meow(x:&String,y:&String){
    println!("{x},{y}");
}