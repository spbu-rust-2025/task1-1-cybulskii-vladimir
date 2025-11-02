use std::io;

fn main() {
    let mut twonumbers = String::new();
    io::stdin()
        .read_line(&mut twonumbers)
        .expect("Failed to read line");
    let mut x = 0;
    for i in twonumbers.split(' ') {
        let inum : i32 = match i.trim().parse::<i32>() {
            Ok(num) => num,
            Err(_) => continue,
        };
        x = x+inum;
    }
    println!("{x}");
}
