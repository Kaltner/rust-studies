fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the retangle is {} square pixels",
        area(width1, height1)
    );

    // Tuple version
    let rect1 = (30, 50); 

    println!(
        "The area of the retangle is {} square pixels",
        area_from_tuple(rect1)
    );

    // Struct version
    let rect = Rectangle{
        width: 30,
        height: 50,
    };

    println!(
        "The area of the retangle is {} square pixels",
        area_from_struct(rect)
    );
}

struct Rectangle {
    width: u32,
    height: u32,
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_from_tuple(rect: (u32, u32)) -> u32 {
    rect.0 * rect.1
}

fn area_from_struct(rect: Rectangle) -> u32 {
    rect.width * rect.height
}