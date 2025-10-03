struct Numbers<T> {
    a: T,
    b: T,
    c: T,
    d: T,
}

impl<T> Numbers<T> {
    fn b(&self) -> &T {
        &self.b
    }
}

fn main() {
    let runs = Numbers {
        a: "abc", b: "abcd", c: "abcde", d: "abcdefg",
    };

    println!("b = {}", runs.b());
}