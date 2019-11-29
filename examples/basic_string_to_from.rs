// 要把任何类型转换成 String，只需要实现那个类型的 ToString trait。
use std::string::ToString;

struct Circle {
    radius: i32
}

impl ToString for Circle {
    fn to_string(&self) -> String {
        format!("Circle of radius {:?}", self.radius)
    }
}

fn main() {
    let circle = Circle { radius: 6 };
    println!("{}", circle.to_string());

    // 只要对目标类型实现了 FromStr trait，就可以用 parse 把字符串转换成目标类型。
    // 标准库中已经给无数种类型实现了 FromStr。如果要转换到用户定义类型，
    // 只要手动实现 FromStr 就行。
    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();

    let sum = parsed + turbo_parsed;
    println!{"Sum: {:?}", sum};
}
