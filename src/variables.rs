pub fn demo() {
    // 不可变变量
    let x = 5;
    println!("不可变变量 x = {}", x);

    // 可变变量
    let mut y = 10;
    println!("可变变量 y = {}", y);
    y = 20;
    println!("修改后 y = {}", y);

    // 常量
    const MAX_POINTS: u32 = 100_000;
    println!("常量 MAX_POINTS = {}", MAX_POINTS);

    // 变量隐藏 (Shadowing)
    let z = 5;
    let z = z + 1;
    let z = z * 2;
    println!("变量隐藏 z = {}", z);

    // 隐藏允许改变类型
    let spaces = "   ";
    let spaces = spaces.len();
    println!("空格数量 spaces = {}", spaces);
}
