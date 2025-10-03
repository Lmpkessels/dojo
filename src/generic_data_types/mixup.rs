struct Option<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Option<X1, Y1> {
    fn mix_up<X2, Y2>(self, mix_option: Option<X2, Y2>) -> Option<X1, Y2> {
        Option {
            x: self.x,
            y: mix_option.y,
        }
    }
}

fn main() {
    let i1 = Option { x: 3223, y: 7179, };
    let i2 = Option { x: "Hello", y: "World", };
    let i3 = i1.mix_up(i2);
    println!("x = {}, y = {}", i3.x, i3.y);
}