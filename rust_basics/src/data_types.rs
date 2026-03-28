pub fn demo() {
    // 标量类型
    let integer: i32 = 42;
    let float: f64 = 3.14;
    let boolean: bool = true;
    let character: char = 'A';

    println!(
        "整数: {}, 浮点数: {}, 布尔值: {}, 字符: {}",
        integer, float, boolean, character
    );

    // 复合类型 - 元组
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("元组解构: x={}, y={}, z={}", x, y, z);
    println!(
        "元组索引: tup.0={}, tup.1={}, tup.2={}",
        tup.0, tup.1, tup.2
    );

    // 复合类型 - 数组
    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
    println!("数组元素: first={}, second={}", first, second);

    // 带类型的数组声明
    let b: [i32; 5] = [1, 2, 3, 4, 5];
    let c = [3; 5]; // [3, 3, 3, 3, 3]
    println!("数组 b: {:?}, 数组 c: {:?}", b, c);
}
