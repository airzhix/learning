pub fn demo() {
    // if-else
    let number = 3;
    if number < 5 {
        println!("条件为真");
    } else {
        println!("条件为假");
    }

    // else if
    let number = 6;
    if number % 4 == 0 {
        println!("能被4整除");
    } else if number % 3 == 0 {
        println!("能被3整除");
    } else {
        println!("不能被4或3整除");
    }

    // if 作为表达式
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("if 表达式结果: {}", number);

    // loop 循环
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("loop 结果: {}", result);

    // while 循环
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");

    // for 循环
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("数组元素: {}", element);
    }

    // 范围 for 循环
    for number in 1..4 {
        println!("范围数字: {}!", number);
    }
}
