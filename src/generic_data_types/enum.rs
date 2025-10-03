enum Option<T> {
    Some(T),
    None,
}

enum Return<T, E> {
    Ok(T),
    Err(E),
}

fn value_or_no_value(inp: Option<u32>) {
    match inp {
        Option::Some(t) => println!("Some {}", t),
        Option::None => println!("None"),
    }
}

fn handle_error(inp: Return<&str, &str>) -> String {
    match inp {
        Return::Ok(a) => a.to_string(),
        Return::Err(b) => b.to_string(),
    }
}

fn main() {
    let some = Option::Some(22);
    let returns_some = value_or_no_value(some);

    let none = Option::None;
    let returns_none = value_or_no_value(none);

    let ok = Return::Ok("This is OKKK");
    let handled = handle_error(ok);
    println!("{}", handled);

    let err = Return::Err("This is an ERRORRR");
    let ihandled = handle_error(err);
    println!("{}", ihandled);
}