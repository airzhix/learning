/// 重新导出（Re-exports）示例
/// 
/// 重新导出是 Rust 模块系统的一个重要特性，它允许我们将内部模块
/// 的项重新导出到外部，从而创建清晰的公共 API。

/// 内部模块 - 包含实际实现
mod internal {
    /// 内部结构体（私有）
    pub struct InternalStruct {
        pub data: String,
        value: i32,
    }

    impl InternalStruct {
        pub fn new(data: String, value: i32) -> Self {
            Self { data, value }
        }

        pub fn get_data(&self) -> &str {
            &self.data
        }

        pub fn get_value(&self) -> i32 {
            self.value
        }
    }

    /// 内部枚举（私有）
    pub enum InternalEnum {
        OptionA(String),
        OptionB { x: i32, y: i32 },
    }

    impl InternalEnum {
        pub fn process(&self) -> String {
            match self {
                InternalEnum::OptionA(text) => format!("处理 A: {}", text),
                InternalEnum::OptionB { x, y } => format!("处理 B: x={}, y={}", x, y),
            }
        }
    }

    /// 内部函数（私有）
    pub fn internal_function(input: &str) -> String {
        format!("内部处理: {}", input)
    }

    /// 内部模块
    pub mod helpers {
        pub fn helper_a() -> &'static str {
            "辅助函数 A"
        }

        pub fn helper_b() -> &'static str {
            "辅助函数 B"
        }
    }
}

/// 重新导出内部项，创建公共 API
pub use internal::InternalStruct as PublicStruct;
pub use internal::InternalEnum as PublicEnum;
pub use internal::helpers::helper_a;
pub use internal::helpers::helper_b as helper_b_function;

/// 重新导出函数
pub use internal::internal_function as process_data;

/// 创建别名，简化使用
pub type DataProcessor = PublicStruct;
pub type ProcessingResult = PublicEnum;

/// 演示重新导出的用途
pub mod api_design {
    use super::internal;

    /// 公共接口结构体
    pub struct PublicAPI {
        processor: internal::InternalStruct,
    }

    impl PublicAPI {
        /// 公共构造函数
        pub fn new(data: String, value: i32) -> Self {
            Self {
                processor: internal::InternalStruct::new(data, value),
            }
        }

        /// 公共方法
        pub fn get_info(&self) -> String {
            format!("数据: {}, 值: {}", 
                   self.processor.get_data(), 
                   self.processor.get_value())
        }

        /// 使用内部功能
        pub fn process(&self) -> String {
            let data = self.processor.get_data();
            internal::internal_function(data)
        }
    }
}

/// 演示模块重构时的重新导出
pub mod legacy_api {
    // 假设我们重构了模块结构，但想保持旧的 API
    use super::internal;

    // 旧的模块路径
    pub mod old_module {
        pub use super::super::internal::InternalStruct;
        pub use super::super::internal::InternalEnum;
    }

    // 新的模块路径
    pub mod new_module {
        pub use super::super::internal::InternalStruct;
        pub use super::super::internal::InternalEnum;
    }

    // 为了向后兼容，重新导出旧的路径
    pub use old_module::InternalStruct as LegacyStruct;
    pub use old_module::InternalEnum as LegacyEnum;
}

/// 演示条件重新导出
#[cfg(feature = "advanced")]
pub mod advanced_features {
    use super::internal;

    /// 高级功能结构体
    pub struct AdvancedProcessor {
        inner: internal::InternalStruct,
    }

    impl AdvancedProcessor {
        pub fn new() -> Self {
            Self {
                inner: internal::InternalStruct::new("高级处理器".to_string(), 999),
            }
        }

        pub fn advanced_process(&self) -> String {
            format!("高级处理: {}", self.inner.get_data())
        }
    }
}

/// 移除条件编译，直接定义高级功能
pub mod advanced_features {
    use super::internal;

    /// 高级功能结构体
    pub struct AdvancedProcessor {
        inner: internal::InternalStruct,
    }

    impl AdvancedProcessor {
        pub fn new() -> Self {
            Self {
                inner: internal::InternalStruct::new("高级处理器".to_string(), 999),
            }
        }

        pub fn advanced_process(&self) -> String {
            format!("高级处理: {}", self.inner.get_data())
        }
    }
}

/// 演示重新导出的最佳实践
pub mod best_practices {
    use super::internal;

    /// 清晰的公共 API 设计
    pub mod public_api {
        pub use super::super::internal::InternalStruct as Processor;
        pub use super::super::internal::InternalEnum as ResultType;
        
        // 重新导出相关功能
        pub use super::super::internal::internal_function as process;
        pub use super::super::internal::helpers::helper_a;
        pub use super::super::internal::helpers::helper_b as helper_b_function;
    }

    /// 内部实现（不应该被外部使用）
    mod implementation {
        use super::super::internal;

        pub struct Implementation {
            data: internal::InternalStruct,
        }

        impl Implementation {
            pub fn new() -> Self {
                Self {
                    data: internal::InternalStruct::new("实现数据".to_string(), 42),
                }
            }
        }
    }
}

/// 演示复杂的重新导出模式
pub mod complex_reexports {
    use super::internal;

    /// 重新导出并重命名
    pub use internal::InternalStruct as CoreProcessor;
    pub use internal::InternalEnum as ProcessingMode;

    /// 重新导出模块
    pub use internal::helpers;

    /// 重新导出并添加文档
    /// 
    /// 这是一个重新导出的函数，用于处理输入数据
    pub use internal::internal_function as handle_input;

    /// 重新导出类型别名
    pub type Handler = CoreProcessor;
    pub type Mode = ProcessingMode;
}

pub fn demo() {
    println!("--- 重新导出 ---");

    // 使用重新导出的结构体
    println!("\n1. 重新导出的结构体:");
    let public_struct = PublicStruct::new("测试数据".to_string(), 123);
    println!("  数据: {}", public_struct.get_data());
    println!("  值: {}", public_struct.get_value());

    // 使用重新导出的枚举
    println!("\n2. 重新导出的枚举:");
    let enum_a = PublicEnum::OptionA("选项 A".to_string());
    let enum_b = PublicEnum::OptionB { x: 10, y: 20 };
    
    println!("  枚举 A 处理: {}", enum_a.process());
    println!("  枚举 B 处理: {}", enum_b.process());

    // 使用重新导出的函数
    println!("\n3. 重新导出的函数:");
    let result = process_data("测试输入");
    println!("  处理结果: {}", result);

    // 使用重新导出的辅助函数
    println!("\n4. 重新导出的辅助函数:");
    println!("  辅助 A: {}", helper_a());
    println!("  辅助 B: {}", helper_b_function());

    // 使用类型别名
    println!("\n5. 类型别名:");
    let processor: DataProcessor = DataProcessor::new("别名测试".to_string(), 456);
    println!("  别名处理器: {}", processor.get_data());

    // 使用公共 API
    println!("\n6. 公共 API 设计:");
    let api = api_design::PublicAPI::new("API 测试".to_string(), 789);
    println!("  API 信息: {}", api.get_info());
    println!("  API 处理: {}", api.process());

    // 使用遗留 API（向后兼容）
    println!("\n7. 遗留 API（向后兼容）:");
    let legacy_struct = legacy_api::LegacyStruct::new("遗留数据".to_string(), 111);
    println!("  遗留结构体: {}", legacy_struct.get_data());

    // 使用复杂重新导出
    println!("\n8. 复杂重新导出:");
    let core_processor = complex_reexports::CoreProcessor::new("核心处理器".to_string(), 222);
    println!("  核心处理器: {}", core_processor.get_data());
    
    let mode = complex_reexports::ProcessingMode::OptionA("模式测试".to_string());
    println!("  处理模式: {}", mode.process());
    
    let handle_result = complex_reexports::handle_input("处理输入");
    println!("  处理输入: {}", handle_result);

    // 使用最佳实践示例
    println!("\n9. 最佳实践:");
    let processor = best_practices::public_api::Processor::new("最佳实践".to_string(), 333);
    println!("  处理器: {}", processor.get_data());
    
    let result = best_practices::public_api::process("最佳实践输入");
    println!("  处理结果: {}", result);

    println!("\n重新导出演示完成！");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reexports() {
        // 测试重新导出的结构体
        let struct_instance = PublicStruct::new("test".to_string(), 42);
        assert_eq!(struct_instance.get_data(), "test");
        assert_eq!(struct_instance.get_value(), 42);
    }

    #[test]
    fn test_reexported_enum() {
        let enum_instance = PublicEnum::OptionA("test".to_string());
        let result = enum_instance.process();
        assert!(result.contains("处理 A: test"));
    }

    #[test]
    fn test_reexported_function() {
        let result = process_data("test");
        assert!(result.contains("内部处理: test"));
    }

    #[test]
    fn test_type_aliases() {
        let processor: DataProcessor = DataProcessor::new("alias".to_string(), 100);
        assert_eq!(processor.get_data(), "alias");
    }
}