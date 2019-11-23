fn main() {
    println!("Hello, world!");

    another_function(32, 32);

    // 表达式函数
    let _x = 5;
    let y = {
        let x = 4;
        x + 1 // 这里不需要分号
    }; // 如果在表达式的结尾加上分号，它就变成了语句，而语句不会返回值
    println!("The value of y is {}", y);

    let x = plus_one(5);
    println!("The value of x is {}", x);

}


fn another_function(x:i32, _y: i32) {
    println!("Another function.");
    println!("The value of x is {}", x);
    five();
}


fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x+1 // 语句并不会返回值，因此不需要添加分号
}