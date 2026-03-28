use std::fs::File;
use std::io::ErrorKind;

pub fn demo() {
    // Option 枚举
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;

    println!("some_number: {:?}", some_number);
    println!("some_string: {:?}", some_string);
    println!("absent_number: {:?}", absent_number);

    // 使用 match 处理 Option
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y.unwrap_or(0);
    println!("5 + {:?} = {}", y, sum);

    // Result 枚举和错误处理
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => {
            println!("文件打开成功: {:?}", file);
            file
        }
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => {
                    println!("文件创建成功: {:?}", fc);
                    fc
                }
                Err(e) => {
                    panic!("创建文件失败: {:?}", e);
                }
            },
            other_error => {
                panic!("打开文件失败: {:?}", other_error);
            }
        },
    };

    println!("文件处理完成: {:?}", f);
}
