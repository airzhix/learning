pub fn demo() {
    // 所有权规则示例
    let s1 = String::from("hello");
    let s2 = s1; // s1 的所有权移动到 s2
    println!("s2 = {}", s2);
    // println!("s1 = {}", s1); // 这行会编译错误，因为 s1 已无效

    // 克隆
    let s3 = String::from("world");
    let s4 = s3.clone();
    println!("s3 = {}, s4 = {}", s3, s4);

    // 引用和借用
    let s5 = String::from("hello");
    let len = calculate_length(&s5);
    println!("字符串 '{}' 的长度是 {}", s5, len);

    // 可变引用
    let mut s6 = String::from("hello");
    change(&mut s6);
    println!("修改后: {}", s6);

    // 不可变引用可以同时有多个
    let s7 = String::from("hello");
    let r1 = &s7;
    let r2 = &s7;
    println!("r1 = {}, r2 = {}", r1, r2);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", world");
}
