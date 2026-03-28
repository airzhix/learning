/// 可见性控制（Visibility）示例
/// 
/// 在 Rust 中，我们可以使用 pub 关键字来控制项的可见性。
/// 这是模块系统的核心概念之一。

/// 这是一个公有结构体，可以从模块外部访问
pub struct PublicStruct {
    /// 公有字段，可以从外部直接访问
    pub name: String,
    /// 私有字段，只能在当前模块内访问
    value: i32,
}

impl PublicStruct {
    /// 公有构造函数
    pub fn new(name: String, value: i32) -> Self {
        Self { name, value }
    }

    /// 公有方法，可以从外部调用
    pub fn get_value(&self) -> i32 {
        self.value
    }

    /// 私有方法，只能在当前模块内调用
    fn private_method(&self) {
        println!("私有方法: {} 的值是 {}", self.name, self.value);
    }

    /// 公有方法调用私有方法
    pub fn public_interface(&self) {
        println!("公有接口: {}", self.name);
        self.private_method();
    }
}

/// 私有结构体，只能在当前模块内访问
struct PrivateStruct {
    data: String,
}

impl PrivateStruct {
    /// 私有结构体的公有方法
    pub fn new(data: String) -> Self {
        Self { data }
    }

    pub fn get_data(&self) -> &str {
        &self.data
    }
}

/// 公有枚举
pub enum PublicEnum {
    /// 公有变体
    OptionA(String),
    /// 另一个公有变体
    OptionB { x: i32, y: i32 },
}

/// 私有枚举
enum PrivateEnum {
    Hidden,
}

/// 公有函数
pub fn public_function() {
    println!("这是公有函数");
    
    // 在模块内部可以访问私有项
    let private = PrivateStruct::new("私有数据".to_string());
    println!("私有数据: {}", private.get_data());
    
    let _hidden = PrivateEnum::Hidden;
}

/// 私有函数
fn private_function() {
    println!("这是私有函数，只能在模块内部调用");
}

/// 展示不同级别的可见性
pub mod visibility_levels {
    /// 公有结构体
    pub struct OuterStruct {
        pub public_field: i32,
        private_field: String,
    }

    impl OuterStruct {
        /// 公有方法
        pub fn new(value: i32, text: String) -> Self {
            Self {
                public_field: value,
                private_field: text,
            }
        }

        /// 私有方法
        fn get_private(&self) -> &str {
            &self.private_field
        }

        /// 公有方法
        pub fn display(&self) {
            println!("公有字段: {}, 私有字段: {}", 
                    self.public_field, self.get_private());
        }
    }

    /// 使用 pub(crate) 限定符 - 只在当前 crate 内可见
    pub(crate) struct CrateVisibleStruct {
        pub data: String,
    }

    /// 使用 pub(super) 限定符 - 只在父模块内可见
    pub(super) fn super_visible_function() {
        println!("这个函数只在父模块中可见");
    }

    /// 使用 pub(in path) 限定符 - 在指定模块路径内可见
    pub(in super) fn module_visible_function() {
        println!("这个函数在指定模块路径内可见");
    }
}

/// 展示模块内的可见性规则
pub mod inner_module {
    use super::visibility_levels;

    pub fn use_crate_visible() {
        // 可以访问 crate 可见的结构体
        let _struct = visibility_levels::CrateVisibleStruct {
            data: "crate 可见".to_string(),
        };
        
        // 可以访问父模块中的 super 可见函数
        visibility_levels::super_visible_function();
        visibility_levels::module_visible_function();
    }
}

pub fn demo() {
    println!("--- 可见性控制 ---");

    // 创建公有结构体实例
    let mut public_struct = PublicStruct::new("测试结构体".to_string(), 100);
    
    // 访问公有字段
    println!("公有字段 name: {}", public_struct.name);
    
    // 调用公有方法
    println!("公有方法 get_value: {}", public_struct.get_value());
    
    // 调用公有接口（内部会调用私有方法）
    public_struct.public_interface();

    // 调用公有函数
    public_function();

    // 使用不同可见性级别的结构体
    let outer = visibility_levels::OuterStruct::new(42, "私有文本".to_string());
    outer.display();

    // 在子模块中使用
    inner_module::use_crate_visible();

    println!("可见性控制演示完成！");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_public_struct_access() {
        let mut struct_instance = PublicStruct::new("test".to_string(), 42);
        
        // 可以访问公有字段
        assert_eq!(struct_instance.name, "test");
        
        // 可以调用公有方法
        assert_eq!(struct_instance.get_value(), 42);
        
        // 可以修改公有字段
        struct_instance.name = "modified".to_string();
        assert_eq!(struct_instance.name, "modified");
    }

    #[test]
    fn test_visibility_levels() {
        let outer = visibility_levels::OuterStruct::new(10, "test".to_string());
        outer.display(); // 这应该能正常工作
    }
}