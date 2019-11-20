use rand::Rng;
use std::cmp::Ordering; // 用来比较的，也是一个枚举
use std::io; // io 来自标准库

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is: {}", secret_number);
    loop {
        println!("Please input your guess");

        let mut guess = String::new(); // mut 表示可变变量
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // 类型转换
        let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };
            // 添加处理无效输入的处理
            // .expect("Please type a number!"); // 这里的guess 隐藏了之前的guess
        println!("Your guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!!"),
            Ordering::Greater => println!("Too big!!"),
            Ordering::Equal => {
                println!("You win!!");
                break;
            }
        };
    }
}
