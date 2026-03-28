/// use 语句和路径导入示例
/// 
/// use 语句是 Rust 中导入模块和项的主要方式，它可以让我们
/// 更方便地使用其他模块中的内容。

/// 一个简单的模块，用于演示 use 语句
pub mod utils {
    /// 工具函数
    pub fn format_message(msg: &str) -> String {
        format!("处理后的消息: {}", msg)
    }

    /// 工具结构体
    pub struct Formatter {
        prefix: String,
    }

    impl Formatter {
        pub fn new(prefix: String) -> Self {
            Self { prefix }
        }

        pub fn format(&self, text: &str) -> String {
            format!("{}: {}", self.prefix, text)
        }
    }

    /// 内部模块
    pub mod internal {
        pub fn helper_function() -> &'static str {
            "内部辅助函数"
        }
    }
}

/// 另一个模块，用于演示跨模块导入
pub mod data {
    pub struct DataProcessor {
        pub data: Vec<i32>,
    }

    impl DataProcessor {
        pub fn new() -> Self {
            Self { data: Vec::new() }
        }

        pub fn add_value(&mut self, value: i32) {
            self.data.push(value);
        }

        pub fn sum(&self) -> i32 {
            self.data.iter().sum()
        }
    }
}

/// 演示不同的 use 语句用法
pub mod use_examples {
    // 1. 基本的 use 语句
    use super::utils::format_message;

    // 2. 使用 as 关键字重命名
    use super::utils::Formatter as TextFormatter;

    // 3. 使用 pub use 重新导出
    pub use super::data::DataProcessor;

    // 4. 使用 glob 模式导入所有项（谨慎使用）
    // use super::utils::internal::*;

    pub fn demonstrate_basic_use() {
        println!("1. 基本 use 语句:");
        
        // 直接使用导入的函数
        let message = format_message("Hello World");
        println!("   {}", message);
    }

    pub fn demonstrate_rename() {
        println!("2. 使用 as 重命名:");
        
        // 使用重命名后的类型
        let formatter = TextFormatter::new("重命名示例".to_string());
        let result = formatter.format("测试文本");
        println!("   {}", result);
    }

    pub fn demonstrate_absolute_paths() {
        println!("3. 绝对路径导入:");
        
        // 使用绝对路径（从 crate 根开始）
        let absolute_path = crate::modules::use_statements::utils::format_message("绝对路径");
        println!("   {}", absolute_path);
    }

    pub fn demonstrate_relative_paths() {
        println!("4. 相对路径导入:");
        
        // 使用相对路径（从当前模块开始）
        let relative_path = super::utils::internal::helper_function();
        println!("   {}", relative_path);
    }

    pub fn demonstrate_nested_imports() {
        println!("5. 嵌套导入:");
        
        // 可以导入模块中的多个项
        use super::utils::{Formatter, internal::helper_function};
        
        let formatter = Formatter::new("嵌套导入".to_string());
        let formatted = formatter.format("测试");
        let helper = helper_function();
        
        println!("   格式化结果: {}", formatted);
        println!("   辅助函数: {}", helper);
    }

    pub fn demonstrate_use_with_structs() {
        println!("6. 结构体的 use:");
        
        // 导入结构体
        use super::data::DataProcessor;
        
        let mut processor = DataProcessor::new();
        processor.add_value(10);
        processor.add_value(20);
        
        println!("   数据总和: {}", processor.sum());
    }
}

/// 演示路径的相对性和绝对性
pub mod path_examples {
    // 绝对路径 - 从 crate 根开始
    use crate::modules::use_statements::utils::Formatter;

    // 相对路径 - 从当前模块开始
    use super::data::DataProcessor;

    pub fn demonstrate_path_differences() {
        println!("7. 路径差异演示:");
        
        // 使用绝对路径导入的类型
        let absolute_formatter = Formatter::new("绝对路径".to_string());
        println!("   {}", absolute_formatter.format("消息"));
        
        // 使用相对路径导入的类型
        let mut processor = DataProcessor::new();
        processor.add_value(100);
        println!("   数据处理器结果: {}", processor.sum());
    }
}

/// 演示 use 语句的最佳实践
pub mod best_practices {
    // 好的做法：明确导入需要的项
    use super::utils::format_message;
    use super::data::DataProcessor;

    // 避免使用 glob 导入，除非确实需要
    // use super::utils::*;  // 不推荐

    pub fn good_practices() {
        println!("8. use 语句最佳实践:");
        
        // 明确导入使代码更清晰
        let message = format_message("明确导入");
        println!("   {}", message);
        
        let mut processor = DataProcessor::new();
        processor.add_value(42);
        println!("   处理器结果: {}", processor.sum());
    }

    // 使用 pub use 创建清晰的公共 API
    pub use super::utils::Formatter;
    pub use super::data::DataProcessor as Processor;
}

pub fn demo() {
    println!("--- use 语句和路径导入 ---");

    // 演示基本用法
    use_examples::demonstrate_basic_use();
    use_examples::demonstrate_rename();
    use_examples::demonstrate_absolute_paths();
    use_examples::demonstrate_relative_paths();
    use_examples::demonstrate_nested_imports();
    use_examples::demonstrate_use_with_structs();

    // 演示路径差异
    path_examples::demonstrate_path_differences();

    // 演示最佳实践
    best_practices::good_practices();

    // 展示重新导出
    println!("9. 重新导出示例:");
    let formatter = best_practices::Formatter::new("重新导出".to_string());
    println!("   {}", formatter.format("测试"));
    
    let mut processor = best_practices::Processor::new();
    processor.add_value(99);
    println!("   重新导出的处理器: {}", processor.sum());

    println!("use 语句演示完成！");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_use_statements() {
        // 测试基本的 use 语句
        let message = utils::format_message("test");
        assert!(message.contains("处理后的消息"));
        
        // 测试结构体使用
        let mut processor = data::DataProcessor::new();
        processor.add_value(1);
        processor.add_value(2);
        assert_eq!(processor.sum(), 3);
    }

    #[test]
    fn test_reexported_items() {
        // 测试重新导出的项
        let formatter = best_practices::Formatter::new("test".to_string());
        let result = formatter.format("hello");
        assert!(result.contains("test: hello"));
    }
}