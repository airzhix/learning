pub fn demo() {
    another_function(5, 'h');

    let result = five();
    println!("函数返回值: {}", result);

    let sum = add(3, 7);
    println!("3 + 7 = {}", sum);
}

fn another_function(x: i32, unit_label: char) {
    println!("参数值: {}{}", x, unit_label);
}

fn five() -> i32 {
    5 // 表达式，没有分号
}

fn add(a: i32, b: i32) -> i32 {
    a + b // 表达式，没有分号
}
