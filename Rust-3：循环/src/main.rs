// 1.使用循环来求1~input之间的和
/* 
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("读取失败");

    let input: i32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("无效的输入");
            return;
        }
    };
    let mut sum = 0;
    for i in 1..=input {
        sum += i;
    }
    println!("1到{}之间的和是：{}", input, sum);
}
*/
// 这个好像挺简单的不需要笔记...



// 2.使用循环来求1~input之间的积
/* 
use std::io;
fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("读取失败");

    let input: i32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("无效的输入");
            return;
        }
    };
    
    let mut product = 1; // 初始化积为1
    for i in 1..=input {
        product *= i; // 累乘
    }

    println!("{}的阶乘是：{}", input, product);

}
*/

// 3.使用循环来求1~input之间的平均数
/*
use std::io;
fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("读取失败");

    let input: i32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("无效的输入");
            return;
        }
    };

    let mut sum = 0; // 初始化和为0
    for i in 1..=input {
        sum += i; // 累加
    }

    let average = sum as f64 / input as f64; // 使用64位浮点数的精度计算平均数
    println!("1到{}之间的平均数是：{:.2}", input, average);
    //                            ^^^^=>表示保留两位小数，与Python的 {:.2f} 类似
    // 注意：在Rust中，整数除法会自动向下取整，所以需要将整数转换为浮点数来得到精确的平均值，所以就有了后面的 as f64
}
*/

// 4.使用循环来求a~b之间的和 / 积 / 平均数
use std::io;
fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("读取失败");

    let input: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("无效的输入"))
        .collect();

    if input.len() != 2 {
        println!("请输入两个数字");
        return;
    }

    let a = input[0];
    let b = input[1];

    if a > b {
        println!("第一个数字应该小于或等于第二个数字");
        return;
    }

    // 求和
    let sum: i32 = (a..=b).sum();
    println!("从 {} 到 {} 的和是：{}", a, b, sum);

    // 求积
    let product: i32 = (a..=b).product();  // 累乘
    println!("从 {} 到 {} 的积是：{}", a, b, product);

    // 求平均数
    let count = (b - a + 1) as f64; // 计算范围内的数字个数
    let average = sum as f64 / count; // 使用64位浮点数的精度计算平均数
    println!("从 {} 到 {} 的平均数是：{:.2}", a, b, average);
}