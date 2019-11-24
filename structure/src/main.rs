#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username:String) -> User {
    return User{
        username,
        email,
        sign_in_count: 1,
        active: true,
    }    
}

fn main() {
    // 注意整个实例必须是可变的
    let mut user1 = User{
        username: String::from("someone"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
        active: true,
    };
    println!("user1: {}", user1.email);

    user1.email = String::from("someone123@example.com");
    println!("user1: {}", user1.email);

    // 使用工厂函数
    let user2 = build_user(String::from("shh@example.com"), String::from("shh"));
    println!("user2: {}", user2.username);

    // 使用其他实例创建新的实例
    let user3 = User{
        username: String::from("user3"),
        email: String::from("user3@example.com"),
        sign_in_count: user1.sign_in_count,
        active: user1.active,
    };
    println!("user3: {}", user3.username);

    // 简写
    let user4 = User {
        username: String::from("user4"),
        email: String::from("user4@example.com"),
        ..user1 // 这里使用user1的与未显示的字段相同的值, 注意到是不需要加逗号
    };

    println!("user4: {}", user4.username);

    // 使用没有明明字段的元组结构体来创建不同的类型
    // 元组结构体
    // 元组结构体有着结构体名称提供的含义，但没有具体的字段名，
    // 只有字段的类型
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);
    // 注意 black 和 origin 值的类型不同，
}
