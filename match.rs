fn main() {
    let x = (1, false);

    match x {
        (51, false) => println!("nope"),
        (51, true) => println!("success"),
        (_, true) => println!("well, it's true I guess"),
        (_, _) => println!("no match")
    }
}
