use rand::Rng;
use std::cmp::Ordering;
use std::io;

// 1.获取用户猜测并打印
// 2.match进行比较
// 3.循环，多次比较
// 4.猜对后退出
// 5.用户输入非数字，忽略并继续
fn main() {
    // ！ 宏
    println!("Guess the num!");

    // 生成一个number
    let secret_num = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Plz input your guess.");

        // :: 关联函数，针对类型实现
        let mut guess = String::new();

        // read_line 输入追加到一个字符串中; 返回一个io::Result,
        io::stdin()
            .read_line(&mut guess)
            .expect("fail to read line");

        // 转换guess类型为u32，使其能比较;  shadow,转换类型，
        // trim去除空白符，parse解析数字，返回Result,处理错误
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("you guessed: {}", guess);

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You Win");
                break;
            }
        }
    }
}
