fn main() {
    // 创建一个不可变的变量，默认不可变
    // let x = 5;
    // println!("The value of x is: {}", x);

    // cannot assign twice to immutable variable x
    // x = 6; // compile error
    // println!("The value of x is: {}", x);

    // 创建一个可变的变量
    let mut x = 6;
    println!("The value of x is {}", x);

    x = 5;
    println!("The value of x is {}", x);

    // const 是完全不可变，不可以使用mut
    // 并且const的关键字不是let，而是const，
    // 且必须使用类型名
    const MAX_POINTS: u32 = 100_000;
    println!("The value of const value is {}", MAX_POINTS);

    // shadowing
    let x = 5;

    //gilet mut x = x + 1;  // suggest:

    // 编译错误
    // x = "34";

    println!("The value of x is {}", x);
}
