#[derive(Debug)]
struct Ages<T, O> {
    a: T,
    b: O,
    c: O,
    d: T,
}

fn main() {
    let compiles = Ages {
        a: 32, b: 33, c: 90, d: 24,
    };

    println!("{:?}", compiles);

    let now_it_compiles = Ages {
        a: 32, b: "16", c: "99", d: 1,
    };

    println!("{:?}", now_it_compiles);

    let this_to_will_compile = Ages {
        a: 32, b: 11, c: 9, d: 1,
    };

    println!("{:?}", this_to_will_compile);
}