fn main() {
    let x = 1;
    let _y = 3.14;
    let _boolean = true;
    let _greet = "Hello";
    println!("Hello, world!");
    println!("Value of x is: {}", x);
    say_hello("John");
    add(10, 20);
}

fn say_hello(name: &str) {
  print!("Hello {}!\n", name);
}

fn add(x: i8, y: i8) {
  println!("{}", x + y);
}