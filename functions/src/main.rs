fn main() {
    let x = plus_one(5);

    println!("The value of x is: {x}");

    let (_a, _b) = addsub(1, 2);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn addsub(x: isize, y: isize) -> (isize, isize) {
    (x + y, x - y) // use tuple to simulate it
}

