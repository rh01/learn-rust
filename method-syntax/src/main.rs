#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// 方法 与函数类似：它们使用 fn 关键字和名称声明，可以拥有参数和返回值，
// 同时包含在某处调用该方法时会执行的代码。
// 不过方法与函数是不同的，因为它们在结构体的上下文中被定义
// （或者是枚举或 trait 对象的上下文，将分别在第六章和第十七章讲解），
// 并且它们第一个参数总是 self，它代表调用该方法的结构体实例。

// 方法需要定义在结构体的上下文中
impl Rectangle {
    fn area(&mut self) -> u32 {
        self.width = 40;
        return self.height * self.width;
    }
}

fn main() {
    // mut
    let mut rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    // 他是这样工作的：当使用 object.something() 调用方法时，Rust 会自动为 object 添加 &、&mut 或 *
    // 以便使 object 与方法签名匹配。也就是说，这些代码是等价的：
    println!("rect1: {:?}", rect1);
}
