fn main() {
    proconio::input! {
        n: usize,
    }

    println!("{}", if n % 2 == 0 {"White"} else {"Black"});
}
