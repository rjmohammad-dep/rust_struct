fn main() {
    let rect1 = (30, 50);

    println!(
        " The Area of rectangle is {} square pixels",
        area(rect1)
    )
}

fn area(dimen: (u32, u32)) -> u32 {
    let (w, h) = dimen;
    w * h
}