fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        " The Area of rectangle is {} square pixels",
        area(width1, height1)
    )
}

fn area(w: u32, h :u32) -> u32 {
    w * h
}