fn main() {
    let input = 100;

    for i in 1..(input + 1) {
        if divisible_by_fifteen(i) {
            println!("Fizzbuzz!");
        } else if divisible_by_five(i) {
            println!("Buzz");
        } else if divisible_by_three(i) {
            println!("Fizz");
        } else {
            println!("{}", i);
        }
    }
}

fn divisible_by_fifteen(x: i16) -> bool {
    x % 15 == 0
}

fn divisible_by_five(x: i16) -> bool {
    x % 5 == 0
}

fn divisible_by_three(x: i16) -> bool {
    x % 3 == 0
}
