// use rand::

fn main() { 
    // let guesss: u32 = rand;
    let _guess: u32 = "43".parse()
        .expect("Not a number!");

    // int 类型
    // 长度	有符号	无符号
    // 8-bit	i8	u8
    // 16-bit	i16	u16
    // 32-bit	i32	u32
    // 64-bit	i64	u64
    // 128-bit	i128	u128
    // arch	isize	usize

    // 数字字面值	     例子
    // Decimal	        98_222
    // Hex	            0xff
    // Octal	        0o77
    // Binary	        0b1111_0000
    // Byte (u8 only)	b'A'

    let x = 2.0; // f64

    let _y: f32 = 3.0; // f32
    
    println!("The value of x is {}", x);

    // 加法
    let sum = 5 + 10;
    println!("The sum is {}", sum);

    // 减法
    let difference = 95.5 - 4.3;
    println!("The difference is {}", difference);

    // 乘法
    let product = 4 * 30;
    println!("THE SUM OF product IS {}", product);

    // 除法
    let quotient = 56.7 / 32.2;
    println!("The quitient is {}", quotient);

    // 取余
    let remainder = 43 % 5;
    println!("The remainder is {}", remainder);


    // warning: unused variable: `f`
    let t: bool = true;

    let f = false;


    // Rust 的 char 类型是语言中最原生的字母类型
    // char为4个字节
    // 拼音字母（Accented letters），中文、日文、
    // 韩文等字符，emoji（绘文字）以及零长度的空白字符都是有效的 char 值 
    let c = 'c';
    let z = 'Z';

    println!("The char c is {}", c);

    let heart_eyed_cat = '😻';
    println!("The char heart_eyed_cat is {}", heart_eyed_cat);

    // 复合类型
    // 元组
    let tp: (f32, u32, f64) = (40.3, 40, 20.5);
    // 使用索引
    println!("The tp index 1 is {}", tp.1);
    // 为了从元组中获取单个值，
    //可以使用模式匹配（pattern matching）来解构（destructure）元组值，像这样：
    let (x1, y1, z1) = tp;
    println!("The Tuple of y1 is {}", x1);

    // 数组元素类型必须一样
    let a = [1, 2, 3, 4, 5];
    println!("The array a is {}", a[1]);

    // 另一种方式
    let a:[i32; 5] = [1, 2, 3, 4, 5];
    println!("The array a is {}", a[1]);


}
