use std::io;

/// 测试函数：用于演示用户输入和数字处理
pub fn number_test() {
    println!("请输入一个数字进行测试：");
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("读取失败");
    let number: i32 = match input.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("请输入有效的数字");
            return;
        }
    };
    
    // 使用 number 变量
    println!("你输入的数字是：{}", number);
    println!("数字的两倍是：{}", number * 2);
    println!("数字的平方是：{}", number * number);
}
