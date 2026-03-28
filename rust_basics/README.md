# Rust 基础语法示例

[![CI](https://github.com/airzhix/learning/actions/workflows/ci.yml/badge.svg)](https://github.com/airzhix/learning/actions/workflows/ci.yml)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

这是一个 Rust 编程语言的基础语法学习项目，涵盖了 Rust 的核心概念和常用语法。

## 🚀 快速开始

### 前置要求
- Rust 1.70+ (推荐使用最新稳定版)
- Cargo (Rust 包管理器)

### 安装和运行
```bash
# 克隆项目
git clone https://github.com/airzhix/learning.git
cd rust_basics

# 运行项目
cargo run

# 运行测试
cargo test

# 检查代码格式
cargo fmt --check

# 运行代码检查
cargo clippy
```

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
8. **闭包** - 闭包语法、捕获变量、闭包类型、实际应用
9. **迭代器** - 迭代器基础、链式操作、适配器、实用示例
10. **泛型和 trait** - 泛型函数、trait 定义与实现
11. **模块系统** - 模块定义、可见性控制、use 语句、嵌套模块、重新导出

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

### 闭包
- 闭包可以捕获环境中的变量
- 闭包有三种类型：Fn、FnMut、FnOnce
- 闭包可以作为函数参数传递
- 闭包在函数式编程中非常有用

### 迭代器
- 迭代器提供高效的数据处理方式
- 支持链式操作和惰性求值
- 常用方法：map、filter、fold、collect
- 迭代器与闭包结合使用非常强大

### 泛型和 trait
- 泛型允许编写适用于多种类型的代码
- trait 定义共享行为，类似于接口
- trait 可以有默认实现

## 参考资料

- [The Rust Programming Language](https://doc.rust-lang.org/book/)
- [Rust By Example](https://doc.rust-lang.org/rust-by-example/)
- [Rust 标准库文档](https://doc.rust-lang.org/std/)

## 🤝 贡献指南

我们欢迎贡献！请在提交 Pull Request 之前：

1. Fork 本仓库
2. 创建特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 打开 Pull Request

### 代码规范

- 使用 `cargo fmt` 格式化代码
- 使用 `cargo clippy` 检查代码质量
- 确保所有测试通过 (`cargo test`)

## 🐛 报告问题

如果您发现了 bug 或者有功能建议，请：

1. 检查是否已有相关 issue
2. 使用 [Issue 模板](https://github.com/airzhix/learning/issues/new/choose) 创建新 issue
3. 提供详细的复现步骤和环境信息

## 📄 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情。

## 🙏 致谢

感谢所有为 Rust 生态做出贡献的开发者们！
