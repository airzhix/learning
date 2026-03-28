# Rust 基础语法示例

这是一个 Rust 编程语言的基础语法学习项目，涵盖了 Rust 的核心概念和常用语法。

## 项目结构

```
rust_basics/
├── Cargo.toml
├── src/
│   ├── main.rs          # 主程序入口
│   ├── variables.rs     # 变量和可变性示例
│   ├── data_types.rs    # 基本数据类型示例
│   ├── functions.rs     # 函数示例
│   ├── control_flow.rs  # 控制流示例
│   ├── ownership.rs     # 所有权和借用示例
│   ├── structs_enums.rs # 结构体和枚举示例
│   ├── error_handling.rs# 错误处理示例
│   └── generics_traits.rs# 泛型和 trait 示例
└── README.md
```

## 包含内容

1. **变量和可变性** - 变量声明、可变性、常量、变量隐藏
2. **基本数据类型** - 标量类型、复合类型、元组、数组
3. **函数** - 函数定义、参数、返回值、语句与表达式
4. **控制流** - if/else、loop、while、for 循环
5. **所有权和借用** - 所有权规则、引用、借用规则
6. **结构体和枚举** - 结构体定义、方法、枚举与模式匹配
7. **错误处理** - Option、Result、错误处理模式
8. **泛型和 trait** - 泛型函数、trait 定义与实现

## 运行方法

1. 确保已安装 Rust 工具链（rustc 和 cargo）
2. 在项目根目录下运行：
   ```bash
   cargo run
   ```
3. 程序将依次演示所有基础语法示例

## 学习要点

### 变量和可变性
- Rust 默认变量不可变，使用 `mut` 关键字使变量可变
- 常量使用 `const` 声明，必须注明类型
- 变量隐藏允许改变变量类型

### 基本数据类型
- 标量类型：整数、浮点数、布尔值、字符
- 复合类型：元组（tuple）、数组（array）

### 函数
- 函数使用 `fn` 关键字定义
- 语句不返回值，表达式返回值
- 函数返回值使用 `->` 指定类型

### 控制流
- `if` 是表达式，可以用于赋值
- `loop`、`while`、`for` 用于循环控制
- `break` 和 `continue` 控制循环流程

### 所有权和借用
- Rust 的核心概念，确保内存安全
- 每个值有且仅有一个所有者
- 引用允许使用值而不获取所有权
- 同一时间只能有一个可变引用或多个不可变引用

### 结构体和枚举
- 结构体用于创建自定义数据类型
- 枚举用于定义具有多种变体的类型
- 模式匹配用于处理枚举的不同变体

### 错误处理
- `Option<T>` 表示值可能存在或不存在
- `Result<T, E>` 表示操作可能成功或失败
- 使用 `?` 操作符简化错误处理

### 泛型和 trait
- 泛型允许编写适用于多种类型的代码
- trait 定义共享行为，类似于接口
- trait 可以有默认实现

## 参考资料

- [The Rust Programming Language](https://doc.rust-lang.org/book/)
- [Rust By Example](https://doc.rust-lang.org/rust-by-example/)
- [Rust 标准库文档](https://doc.rust-lang.org/std/)

## 许可证

本项目仅供学习使用。