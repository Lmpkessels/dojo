use std::fmt::Display;

struct Pair<T> {
    a: T,
    b: T,
}

impl<T> Pair<T> {
    fn new(a: T, b: T) -> Self {
        Self { a, b }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.a > self.b {
            println!("The biggest number is a = {}", self.a);
        } else if self.a < self.b {
            println!("The biggest number is b = {}", self.b);
        } else {
            println!("They're equal");
        }
    }
}

fn main() {
    let p = Pair {
        a: 18,
        b: 19,
    };

    p.cmp_display();
}