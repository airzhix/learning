mod control_flow;
mod data_types;
mod error_handling;
mod functions;
mod generics_traits;
mod ownership;
mod structs_enums;
mod variables;

fn main() {
    println!("=== Rust 基础语法示例 ===\n");

    println!("1. 变量和可变性");
    variables::demo();

    println!("\n2. 基本数据类型");
    data_types::demo();

    println!("\n3. 函数");
    functions::demo();

    println!("\n4. 控制流");
    control_flow::demo();

    println!("\n5. 所有权和借用");
    ownership::demo();

    println!("\n6. 结构体和枚举");
    structs_enums::demo();

    println!("\n7. 错误处理");
    error_handling::demo();

    println!("\n8. 泛型和 trait");
    generics_traits::demo();
}
