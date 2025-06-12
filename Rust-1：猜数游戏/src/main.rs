use std::io;               // 引入io模块来处理输入输出
use std::cmp::Ordering;    // 引入Ordering枚举来比较两个值的大小
use rand::{self, Rng};     // 引入rand库来生成随机数

// 声明自定义模块
// mod utils;

fn main() {
    println!("标题：请猜测一个你想猜测的范围之间的随机数");
    println!("请输入你想猜测的开始范围");
    let mut range1 = String::new();
    io::stdin()
        .read_line(&mut range1)
        .expect("读取失败");

    let range1: i32 = match range1.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("无效的输入");
            return;
        }
    };
    println!("请输入你想猜测的结束范围");
    let mut range2: String = String::new();
    io::stdin()
        .read_line(&mut range2)
        .expect("读取失败");

    let range2: i32 = match range2.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("无效的输入");
            return;
        }
    };
    let secret_num: i32 = rand::rng().random_range(range1..=range2);  // 生成range1到range2之间的随机数
    loop {
        println!("请输入你的猜测：");
        let mut guess: String = String::new();  // 创建一个新的字符串变量来存储用户的输入

        io::stdin()
            .read_line(&mut guess)  // 从键盘输入读取一行数据
            .expect("读取失败");
        // 将输入的字符串转换为整数
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,  // 将成功解析的数字赋值给guess
            Err(_) => continue,
        };
        // 比较用户的猜测和生成的随机数 
        // 其中"guess >= range1 && guess <= range2" 意思就是 "range1 <= guess <= range2",只不过Rust里面不能这样写
        if guess >= range1 && guess <= range2 { 
            println!("你猜测的数是：{}", guess);
        } else {
            println!("请在{}到{}之间猜测", range1, range2);
            continue;
        }
        // 这里的match就是进行猜测结果的判断
        match guess.cmp(&secret_num) {
            Ordering::Less => println!("猜小了"),
            Ordering::Greater => println!("猜大了"),
            Ordering::Equal => {  
                println!("猜对了");

                break;
            }
        }
    }
    println!("游戏结束");
    
    // 调用测试函数
    // println!("\n=== 测试函数 ===");
    // utils::number_test();
}






/*
Python
input = input("请输入一个数字：")
try:
    number = int(input)
except ValueError:
    print("无效的输入")
    exit()
*/