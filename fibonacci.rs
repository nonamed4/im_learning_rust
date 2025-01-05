fn main() {
    let mut n = String::new();

    println!("Please enter a number!");
    std::io::stdin()
        .read_line(&mut n)
        .expect("?");

    let mut n : i32 = n.trim().parse().expect("Not a number!");

    let (mut a, mut b): (i64, i64) = (1, 0);
    let a: i64 = loop {
        n -= 1;
        (a, b) = (a+b, a);
        if n <= 0 {break a;}
    };
    println!("{a}, {b}");
}