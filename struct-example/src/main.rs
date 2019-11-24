fn main() {

    let rect = Rectangle{
        height: 30, 
        width: 50,
    };
    // let rect = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect)
    );

    println!("rect: {:?}", rect);
}

// 增加注解来派生 Debug trait，并使用调试格式打印 Rectangle 实例
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// 利用结构体重构
fn area(rect: &Rectangle) -> u32 {
    return rect.height * rect.width
}


// fn area(rect: (u32, u32)) -> u32 {
//     return rect.0 * rect.1
// }

