/// 闭包（Closures）示例
/// 
/// 闭包是 Rust 中的一个重要特性，它们可以捕获环境中的变量，
/// 并且可以作为函数参数传递，这使得函数式编程成为可能。

use std::thread;
use std::time::Duration;

pub fn demo() {
    println!("--- 闭包基础 ---");
    
    // 1. 简单闭包
    let simple_closure = |x: i32| x + 1;
    let result = simple_closure(5);
    println!("简单闭包结果: {}", result);
    
    // 2. 捕获变量的闭包
    let mut counter = 0;
    let mut increment = || {
        counter += 1;
        println!("计数器: {}", counter);
    };
    
    increment();
    increment();
    increment();
    
    // 3. 闭包作为函数参数
    let numbers = vec![1, 2, 3, 4, 5];
    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    println!("数字翻倍: {:?}", doubled);
    
    // 4. 带有返回值的闭包
    let add = |a: i32, b: i32| -> i32 {
        a + b
    };
    let sum = add(10, 20);
    println!("闭包求和: {}", sum);
    
    // 5. 捕获外部变量的闭包
    let multiplier = 3;
    let multiply_by = |x: i32| x * multiplier;
    let result = multiply_by(7);
    println!("乘以 {}: {}", multiplier, result);
    
    println!("\n--- 闭包与所有权 ---");
    
    // 6. 移动闭包 (move closure)
    let data = vec![1, 2, 3, 4, 5];
    let sum_closure = move || {
        data.iter().sum::<i32>()
    };
    
    let total = sum_closure();
    println!("移动闭包求和: {}", total);
    // 注意：data 在这里已经被移动到闭包中，无法再使用
    
    println!("\n--- 闭包在实际应用中的使用 ---");
    
    // 7. 排序时使用闭包
    let mut words = vec!["rust", "programming", "language", "is", "awesome"];
    words.sort_by(|a, b| a.len().cmp(&b.len()));
    println!("按长度排序: {:?}", words);
    
    // 8. 过滤数据
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let evens: Vec<i32> = numbers.into_iter().filter(|x| x % 2 == 0).collect();
    println!("偶数: {:?}", evens);
    
    // 9. 累积计算
    let numbers = vec![1, 2, 3, 4, 5];
    let product = numbers.iter().fold(1, |acc, x| acc * x);
    println!("累积乘积: {}", product);
    
    println!("\n--- 闭包与线程 ---");
    
    // 10. 在线程中使用闭包
    let handle = thread::spawn(|| {
        for i in 1..=3 {
            println!("线程中的消息 {}", i);
            thread::sleep(Duration::from_millis(100));
        }
    });
    
    // 等待线程完成
    handle.join().unwrap();
    
    println!("\n--- 闭包类型 ---");
    
    // 11. 不同类型的闭包
    let closure1 = |x: i32| x;           // FnOnce - 可以消费捕获的变量
    let closure2 = |x: &i32| *x;    // Fn - 只读访问捕获的变量
    let closure3 = |x: &mut i32| *x += 1; // FnMut - 可变访问捕获的变量
    
    let mut value = 10;
    println!("原始值: {}", value);
    closure3(&mut value);
    println!("修改后: {}", value);
    
    println!("\n闭包演示完成！");
}

/// 高阶函数示例 - 使用闭包作为参数
pub fn apply_operation<F>(data: Vec<i32>, operation: F) -> Vec<i32>
where
    F: Fn(i32) -> i32,
{
    data.into_iter().map(operation).collect()
}

/// 条件过滤函数
pub fn filter_data<F>(data: Vec<i32>, predicate: F) -> Vec<i32>
where
    F: Fn(&i32) -> bool,
{
    data.into_iter().filter(predicate).collect()
}

/// 累积操作函数
pub fn accumulate<F>(data: Vec<i32>, initial: i32, operation: F) -> i32
where
    F: Fn(i32, i32) -> i32,
{
    data.into_iter().fold(initial, operation)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_closure() {
        let add_one = |x: i32| x + 1;
        assert_eq!(add_one(5), 6);
    }

    #[test]
    fn test_map_closure() {
        let numbers = vec![1, 2, 3];
        let doubled = apply_operation(numbers, |x| x * 2);
        assert_eq!(doubled, vec![2, 4, 6]);
    }

    #[test]
    fn test_filter_closure() {
        let numbers = vec![1, 2, 3, 4, 5];
        let evens = filter_data(numbers, |x| x % 2 == 0);
        assert_eq!(evens, vec![2, 4]);
    }

    #[test]
    fn test_accumulate_closure() {
        let numbers = vec![1, 2, 3, 4];
        let sum = accumulate(numbers, 0, |acc, x| acc + x);
        assert_eq!(sum, 10);
    }
}