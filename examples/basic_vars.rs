fn mut_vars(){
    let mut a: f64 = 1.0;
    let b = 2.0f32;

    //改变 a 的绑定
    a = 2.0;
    println!("{:?}", a);

    //重新绑定为不可变
    let a = a;

    //不能赋值
    //a = 3.0;

    //类型不匹配
    //assert_eq!(a, b);
}

fn main() {
    let a1 = 5;
    let a2:i32 = 5;
    assert_eq!(a1, a2);
    //let 绑定 整数变量默认类型推断是 i32

    let b1:u32 = 5;
    //assert_eq!(a1, b1);
    //去掉上面的注释会报错，因为类型不匹配
    //errer: mismatched types

    mut_vars();
}
