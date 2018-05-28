macro_rules! five_times {
    ($x:expr) => {
        5 * $x
    };
}

fn main() {
    println!(five_times(5));
}

#[cfg(test)]
mod tests {
    #[#[test]
    fn it_works() {
        unimplemented!()
    }]
}
