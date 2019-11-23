fn main() {
    let x = 4;
    if x < 3 {
        println!("condition is true");
    }else {
        println!("condition is false")
    }
    

    // 在let语句中使用if
    let condition = false;
    let number = if condition {
        3
    } else {
        // error[E0308]: if and else have incompatible types
        // "32"
        2
    };

    println!("the number is  {}", number);


    // 循环
    // rust提供了三种循环 loop、for和while
    let mut i = 1;
    let x = loop {
        i+=1;
        if i == 3 {
            // 普通的中断循环的方式
            // break;
            // 从中断中返回
            break i*2;
        };
    };
    println!("The x is {}", x);

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index = index + 1;
    };

    // 我们增强了代码安全性，并消除了可能由于超出数组的结尾
    // 或遍历长度不够而缺少一些元素而导致的 bug。
    for elem in a.iter() {
        println!("The value is {}", elem);
    };

    // Range类型
    for i in (1..4) {
        println!("The value is {}", i);
    };

    // rev，用来反转 range：
    for i in (1..4).rev() {
        println!("The value is {}", i);
    };
}
