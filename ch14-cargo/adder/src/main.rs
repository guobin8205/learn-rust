// adder：调用 add_one 库
use add_one;

fn main() {
    let num = 10;
    println!(
        "Hello, world! {} + 1 = {}",
        num,
        add_one::add_one(num)
    );
}
