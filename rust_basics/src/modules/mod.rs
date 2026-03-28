/// 模块系统（Modules）示例
/// 
/// 模块系统是 Rust 中组织代码的核心特性，它帮助我们将代码分组、
/// 控制可见性，并提供命名空间管理。

pub mod visibility;
pub mod use_statements;
pub mod nested_modules;
pub mod reexports;

pub fn demo() {
    println!("=== 模块系统演示 ===\n");

    println!("1. 可见性控制");
    visibility::demo();

    println!("\n2. use 语句和路径导入");
    use_statements::demo();

    println!("\n3. 嵌套模块");
    nested_modules::demo();

    println!("\n4. 重新导出");
    reexports::demo();

    println!("\n模块系统演示完成！");
}

/// 一个简单的模块示例
pub mod example_module {
    /// 公有结构体
    pub struct PublicStruct {
        pub name: String,
        value: i32,  // 私有字段
    }

    impl PublicStruct {
        /// 公有关联函数
        pub fn new(name: String, value: i32) -> Self {
            Self { name, value }
        }

        /// 公有方法
        pub fn get_value(&self) -> i32 {
            self.value
        }

        /// 私有方法
        fn private_method(&self) {
            println!("这是私有方法，只能在模块内部调用");
        }

        /// 公有方法调用私有方法
        pub fn public_interface(&self) {
            println!("公有接口: {}", self.name);
            self.private_method();
        }
    }

    /// 私有枚举
    enum PrivateEnum {
        Option1,
        Option2,
    }

    /// 公有函数
    pub fn public_function() {
        println!("这是公有函数");
        let _private = PrivateEnum::Option1;  // 可以在模块内部使用私有项
    }
}

/// 另一个模块示例 - 展示模块层次
pub mod library {
    pub mod books {
        pub struct Book {
            pub title: String,
            pub author: String,
            pub isbn: String,
        }

        impl Book {
            pub fn new(title: String, author: String, isbn: String) -> Self {
                Self { title, author, isbn }
            }
        }
    }

    pub mod readers {
        use super::books::Book;

        pub struct Reader {
            pub name: String,
            borrowed_books: Vec<Book>,
        }

        impl Reader {
            pub fn new(name: String) -> Self {
                Self {
                    name,
                    borrowed_books: Vec::new(),
                }
            }

            pub fn borrow_book(&mut self, book: Book) {
                self.borrowed_books.push(book);
                println!("{} 借阅了一本书", self.name);
            }

            pub fn list_books(&self) -> Vec<&Book> {
                self.borrowed_books.iter().collect()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_public_struct() {
        let book = example_module::PublicStruct::new("Test".to_string(), 42);
        assert_eq!(book.get_value(), 42);
        assert_eq!(book.name, "Test");
    }

    #[test]
    fn test_library_system() {
        let mut reader = library::readers::Reader::new("Alice".to_string());
        let book = library::books::Book::new(
            "Rust Programming".to_string(),
            "Someone".to_string(),
            "123456".to_string(),
        );
        
        reader.borrow_book(book);
        let books = reader.list_books();
        assert_eq!(books.len(), 1);
    }
}