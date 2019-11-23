fn main() {
    let mut s1 = String::from("hello");

    let len = calculate_length(&mut s1);

    // error[E0502]: cannot borrow `s1` as immutable because it is also borrowed as mutable
    // let r1 = &mut s1;
    // let len2 = calculate_length(r1);
    
    println!("The length of '{}' is {}.", s1, len);
    
    // correct, create a new block
    {
        let r1 = &mut s1;
        // let len2 = calculate_length(r1);
        println!("The length of '{}' is {}.", r1, len);
    }


    {
        let mut s = String::from("hello");

        let r1 = &s; // 没问题
        let r2 = &s; // 没问题
        println!("{} and {}", r1, r2);
        // 此位置之后 r1 和 r2 不再使用

        let r3 = &mut s; // 没问题
        // error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
        // println!("s1: {}, s2: {}, s3: {}", r1, r2, r3);
        println!("{}", r3);
    }

    /*
    {
        let reference_to_nothing = dangle();
        // error: help: this function's return type contains a borrowed value, but there is
        //   no value for it to be borrowed from
        fn dangle() -> &String {
            let s = String::from("hello");
        
            &s
        }
    }
    */

    // println!("The length of '{}' is {}.", r1, len2);
}

// borrowing
// 如果我们尝试修改借用的变量呢？尝试示例 4-6 中的代码。剧透：这行不通！
fn calculate_length(s:  &mut String) -> usize {
    // err: s.push_str(", world.")
    // 正如变量默认是不可变的，引用也一样。（默认）不允许修改引用的值。

    // 修改为可变引用即可
    s.push_str(", world.");
    s.len()

    // 不过可变引用有一个很大的限制：在特定作用域中的特定数据有且只有一个可变引用。这些代码会失败：
    // 这个限制的好处是 Rust 可以在编译时就避免数据竞争。
    // 两个或更多指针同时访问同一数据。
    // 至少有一个指针被用来写入数据。
    // 没有同步数据访问的机制。

}// 这里，s 离开了作用域。但因为它并不拥有引用值的所有权，
  // 所以什么也不会发生
