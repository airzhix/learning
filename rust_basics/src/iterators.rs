/// 迭代器（Iterators）与闭包结合使用示例
/// 
/// 迭代器是 Rust 的另一个强大特性，它们与闭包结合使用可以实现
/// 高效、函数式的编程模式。

pub fn demo() {
    println!("--- 迭代器基础 ---");
    
    // 1. 基本迭代器
    let numbers = vec![1, 2, 3, 4, 5];
    
    println!("原始数据: {:?}", numbers);
    
    // 2. map - 转换数据
    let squared: Vec<i32> = numbers.iter().map(|x| x * x).collect();
    println!("平方: {:?}", squared);
    
    // 3. filter - 过滤数据
    let evens: Vec<&i32> = numbers.iter().filter(|x| *x % 2 == 0).collect();
    println!("偶数: {:?}", evens);
    
    // 4. filter_map - 过滤并转换
    let numbers_with_option = vec![Some(1), Some(2), None, Some(4), None, Some(6)];
    let valid_doubled: Vec<i32> = numbers_with_option
        .into_iter()
        .filter_map(|x| x)
        .map(|x| x * 2)
        .collect();
    println!("有效数字翻倍: {:?}", valid_doubled);
    
    println!("\n--- 高级迭代器操作 ---");
    
    // 5. fold - 累积操作
    let sum = numbers.iter().fold(0, |acc, x| acc + x);
    println!("求和: {}", sum);
    
    let product = numbers.iter().fold(1, |acc, x| acc * x);
    println!("乘积: {}", product);
    
    // 6. reduce - 简化的累积（需要 itertools crate，这里用 fold 演示）
    let max = numbers.iter().fold(i32::MIN, |acc, x| if *x > acc { *x } else { acc });
    println!("最大值: {}", max);
    
    // 7. enumerate - 带索引的迭代
    for (index, value) in numbers.iter().enumerate() {
        println!("索引 {}: 值 {}", index, value);
    }
    
    println!("\n--- 迭代器链式操作 ---");
    
    // 8. 复杂的链式操作
    let result: Vec<i32> = (1..=20)
        .filter(|x| x % 2 == 0)      // 过滤偶数
        .map(|x| x * x)              // 计算平方
        .filter(|x| *x > 50)         // 过滤大于 50 的
        .take(5)                     // 取前 5 个
        .collect();
    
    println!("复杂链式操作结果: {:?}", result);
    
    // 9. zip - 合并两个迭代器
    let letters = vec!['a', 'b', 'c', 'd'];
    let numbers = vec![1, 2, 3, 4];
    
    let pairs: Vec<(char, i32)> = letters.into_iter().zip(numbers.into_iter()).collect();
    println!("配对结果: {:?}", pairs);
    
    println!("\n--- 迭代器适配器 ---");
    
    // 10. take 和 skip
    let numbers: Vec<i32> = (1..=10).collect();
    let first_five: Vec<i32> = numbers.iter().take(5).cloned().collect();
    let skip_first_three: Vec<i32> = numbers.iter().skip(3).cloned().collect();
    
    println!("前五个数字: {:?}", first_five);
    println!("跳过前三个: {:?}", skip_first_three);
    
    // 11. chain - 连接迭代器
    let first = vec![1, 2, 3];
    let second = vec![4, 5, 6];
    let chained: Vec<i32> = first.into_iter().chain(second.into_iter()).collect();
    println!("连接后的向量: {:?}", chained);
    
    // 12. flatten - 展平嵌套结构
    let nested = vec![vec![1, 2], vec![3, 4], vec![5, 6]];
    let flattened: Vec<i32> = nested.into_iter().flatten().collect();
    println!("展平后的向量: {:?}", flattened);
    
    println!("\n--- 迭代器与闭包的实用示例 ---");
    
    // 13. 数据处理管道
    let data = vec![
        ("Alice", 25, "Engineer"),
        ("Bob", 30, "Designer"),
        ("Charlie", 35, "Manager"),
        ("Diana", 28, "Engineer"),
    ];
    
    // 找出所有工程师并按年龄排序
    let engineers: Vec<_> = data
        .into_iter()
        .filter(|(_, _, title)| *title == "Engineer")
        .map(|(name, age, title)| (name.to_string(), age, title.to_string()))
        .collect();
    
    println!("工程师列表: {:?}", engineers);
    
    // 14. 统计分析
    let scores = vec![85, 92, 78, 96, 88, 73, 91, 87];
    
    let average = scores.iter().sum::<i32>() as f32 / scores.len() as f32;
    let above_average: Vec<i32> = scores.iter().filter(|&&score| score as f32 > average).cloned().collect();
    
    println!("平均分: {:.2}", average);
    println!("高于平均分的分数: {:?}", above_average);
    
    // 15. 分组统计
    let words = vec!["apple", "banana", "cherry", "date", "elderberry", "fig"];
    
    let word_lengths: std::collections::HashMap<usize, Vec<&str>> = words
        .iter()
        .fold(std::collections::HashMap::new(), |mut acc, word| {
            let length = word.len();
            acc.entry(length).or_insert_with(Vec::new).push(*word);
            acc
        });
    
    println!("按长度分组的单词:");
    for (length, words) in word_lengths {
        println!("  长度 {}: {:?}", length, words);
    }
    
    println!("\n迭代器演示完成！");
}

/// 实用的迭代器工具函数
pub fn process_numbers<F, G>(numbers: Vec<i32>, filter_fn: F, transform_fn: G) -> Vec<i32>
where
    F: Fn(&i32) -> bool,
    G: Fn(i32) -> i32,
{
    numbers
        .into_iter()
        .filter(filter_fn)
        .map(transform_fn)
        .collect()
}

/// 查找最大值和最小值
pub fn find_min_max(numbers: Vec<i32>) -> Option<(i32, i32)> {
    if numbers.is_empty() {
        None
    } else {
        let min = numbers.iter().fold(i32::MAX, |acc, x| if *x < acc { *x } else { acc });
        let max = numbers.iter().fold(i32::MIN, |acc, x| if *x > acc { *x } else { acc });
        Some((min, max))
    }
}

/// 计算中位数
pub fn calculate_median(mut numbers: Vec<f32>) -> Option<f32> {
    if numbers.is_empty() {
        None
    } else {
        numbers.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let len = numbers.len();
        if len % 2 == 0 {
            Some((numbers[len / 2 - 1] + numbers[len / 2]) / 2.0)
        } else {
            Some(numbers[len / 2])
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_filter() {
        let numbers = vec![1, 2, 3, 4, 5];
        let result = process_numbers(numbers, |x| x % 2 == 0, |x| x * 2);
        assert_eq!(result, vec![4, 8]);
    }

    #[test]
    fn test_find_min_max() {
        let numbers = vec![3, 1, 4, 1, 5, 9, 2, 6];
        let (min, max) = find_min_max(numbers).unwrap();
        assert_eq!(min, 1);
        assert_eq!(max, 9);
    }

    #[test]
    fn test_empty_min_max() {
        let result = find_min_max(vec![]);
        assert_eq!(result, None);
    }
}