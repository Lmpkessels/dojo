#[derive(Debug)]
enum Option<T> {
    Some(T),
    None,
}

fn main() {
    let integer = Option::Some(34);
    let float = Option::Some(33.1);

    println!("Integer = {:?}", integer);
    println!("Float = {:?}", float);
}