fn main() {
    // naive method
    // problem: width1 & height1 aren't clearly related
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area1(width1, height1)
    );

    // tuple method
    // problem: elements are unnamed
    let rect2 = (30, 50);
    println!(
        "The area of the rectangle is {} square pixels.",
        area2(rect2)
    );

    // struct method
    let rect3 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        area3(&rect3)
    );

    // print Rectangle struct instance
    println!("rect3 is {rect3:#?}");
    dbg!(&rect3);
}

fn area1(width: u32, height: u32) -> u32 {
    width * height
}

fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
