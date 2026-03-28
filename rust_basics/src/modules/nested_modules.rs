/// 嵌套模块（Nested Modules）示例
/// 
/// 嵌套模块展示了如何在模块内部定义子模块，以及如何组织
/// 复杂的模块层次结构。

/// 游戏模块 - 展示嵌套模块的组织方式
pub mod game {
    /// 玩家模块
    pub mod player {
        use super::inventory::Inventory;
        use super::stats::Stats;

        /// 玩家结构体
        pub struct Player {
            pub name: String,
            pub level: u32,
            pub stats: Stats,
            pub inventory: Inventory,
        }

        impl Player {
            pub fn new(name: String) -> Self {
                Self {
                    name,
                    level: 1,
                    stats: Stats::new(100, 10, 5),
                    inventory: Inventory::new(),
                }
            }

            pub fn display_info(&self) {
                println!("玩家: {}", self.name);
                println!("  等级: {}", self.level);
                self.stats.display();
                self.inventory.display();
            }

            pub fn level_up(&mut self) {
                self.level += 1;
                self.stats.increase_max_hp(20);
                self.stats.restore_hp();
                println!("{} 升级了！当前等级: {}", self.name, self.level);
            }
        }
    }

    /// 统计模块
    pub mod stats {
        /// 角色统计信息
        pub struct Stats {
            pub max_hp: u32,
            pub current_hp: u32,
            pub attack: u32,
            pub defense: u32,
        }

        impl Stats {
            pub fn new(max_hp: u32, attack: u32, defense: u32) -> Self {
                Self {
                    max_hp,
                    current_hp: max_hp,
                    attack,
                    defense,
                }
            }

            pub fn take_damage(&mut self, damage: u32) {
                if damage >= self.current_hp {
                    self.current_hp = 0;
                    println!("角色倒下了！");
                } else {
                    self.current_hp -= damage;
                    println!("受到 {} 点伤害，剩余生命值: {}", damage, self.current_hp);
                }
            }

            pub fn heal(&mut self, amount: u32) {
                let new_hp = self.current_hp + amount;
                self.current_hp = new_hp.min(self.max_hp);
                println!("恢复了 {} 点生命值，当前生命值: {}", amount, self.current_hp);
            }

            pub fn restore_hp(&mut self) {
                self.current_hp = self.max_hp;
                println!("生命值已完全恢复！");
            }

            pub fn increase_max_hp(&mut self, amount: u32) {
                self.max_hp += amount;
                self.current_hp += amount;
                println!("最大生命值增加 {}，当前生命值: {}/{}", amount, self.current_hp, self.max_hp);
            }

            pub fn display(&self) {
                println!("  生命值: {}/{}", self.current_hp, self.max_hp);
                println!("  攻击力: {}", self.attack);
                println!("  防御力: {}", self.defense);
            }
        }
    }

    /// 背包模块
    pub mod inventory {
        use super::items::Item;

        /// 背包结构体
        pub struct Inventory {
            items: Vec<Item>,
            capacity: usize,
        }

        impl Inventory {
            pub fn new() -> Self {
                Self {
                    items: Vec::new(),
                    capacity: 10,
                }
            }

            pub fn add_item(&mut self, item: Item) -> Result<(), String> {
                if self.items.len() >= self.capacity {
                    return Err("背包已满".to_string());
                }
                self.items.push(item);
                Ok(())
            }

            pub fn remove_item(&mut self, name: &str) -> Option<Item> {
                let index = self.items.iter().position(|item| item.name == name)?;
                Some(self.items.remove(index))
            }

            pub fn list_items(&self) -> Vec<&Item> {
                self.items.iter().collect()
            }

            pub fn use_item(&mut self, name: &str) -> Result<String, String> {
                let index = self.items.iter().position(|item| item.name == name)
                    .ok_or_else(|| format!("物品 '{}' 不存在", name))?;
                
                let item = &self.items[index];
                let result = format!("使用了物品: {}", item.name);
                
                // 如果是消耗品，使用后移除
                if item.is_consumable {
                    self.items.remove(index);
                }
                
                Ok(result)
            }

            pub fn display(&self) {
                println!("  背包内容:");
                if self.items.is_empty() {
                    println!("    (空)");
                } else {
                    for item in &self.items {
                        println!("    - {}", item.name);
                    }
                }
            }
        }
    }

    /// 物品模块
    pub mod items {
        /// 游戏物品
        #[derive(Clone)]
        pub struct Item {
            pub name: String,
            pub description: String,
            pub is_consumable: bool,
        }

        impl Item {
            pub fn new(name: String, description: String, is_consumable: bool) -> Self {
                Self {
                    name,
                    description,
                    is_consumable,
                }
            }

            pub fn use_item(&self) -> String {
                if self.is_consumable {
                    format!("使用了消耗品: {}", self.name)
                } else {
                    format!("装备了物品: {}", self.name)
                }
            }
        }

        /// 预定义物品
        pub mod presets {
            use super::Item;

            pub fn health_potion() -> Item {
                Item::new(
                    "生命药水".to_string(),
                    "恢复 50 点生命值".to_string(),
                    true,
                )
            }

            pub fn sword() -> Item {
                Item::new(
                    "铁剑".to_string(),
                    "基础近战武器".to_string(),
                    false,
                )
            }

            pub fn shield() -> Item {
                Item::new(
                    "木盾".to_string(),
                    "提供基础防御".to_string(),
                    false,
                )
            }
        }
    }

    /// 战斗模块
    pub mod combat {
        use super::player::Player;
        use super::stats::Stats;

        /// 敌人结构体
        pub struct Enemy {
            pub name: String,
            pub stats: Stats,
        }

        impl Enemy {
            pub fn new(name: String, hp: u32, attack: u32, defense: u32) -> Self {
                Self {
                    name,
                    stats: Stats::new(hp, attack, defense),
                }
            }

            pub fn attack(&self, target: &mut Player) -> u32 {
                let damage = self.stats.attack;
                println!("{} 攻击了 {}，造成 {} 点伤害", self.name, target.name, damage);
                target.stats.take_damage(damage);
                damage
            }
        }

        /// 战斗系统
        pub struct CombatSystem;

        impl CombatSystem {
            pub fn battle(player: &mut Player, enemy: &mut Enemy) -> bool {
                println!("\n=== 战斗开始 ===");
                println!("对手: {}", enemy.name);

                loop {
                    // 玩家回合
                    println!("\n--- 玩家回合 ---");
                    player.stats.display();
                    
                    // 简单的玩家攻击
                    let player_damage = player.stats.attack;
                    enemy.stats.take_damage(player_damage);
                    
                    if enemy.stats.current_hp == 0 {
                        println!("{} 获胜！", player.name);
                        return true;
                    }

                    // 敌人回合
                    println!("\n--- 敌人回合 ---");
                    enemy.attack(player);
                    
                    if player.stats.current_hp == 0 {
                        println!("{} 获胜！", enemy.name);
                        return false;
                    }
                }
            }
        }
    }
}

/// 工具模块 - 展示另一种嵌套结构
pub mod tools {
    /// 数学工具
    pub mod math {
        /// 几何计算
        pub mod geometry {
            #[derive(Debug)]
            pub struct Point {
                pub x: f64,
                pub y: f64,
            }

            #[derive(Debug)]
            pub struct Rectangle {
                pub width: f64,
                pub height: f64,
            }

            impl Point {
                pub fn new(x: f64, y: f64) -> Self {
                    Self { x, y }
                }

                pub fn distance_to(&self, other: &Point) -> f64 {
                    ((other.x - self.x).powi(2) + (other.y - self.y).powi(2)).sqrt()
                }
            }

            impl Rectangle {
                pub fn new(width: f64, height: f64) -> Self {
                    Self { width, height }
                }

                pub fn area(&self) -> f64 {
                    self.width * self.height
                }

                pub fn perimeter(&self) -> f64 {
                    2.0 * (self.width + self.height)
                }
            }
        }

        /// 数值计算
        pub mod calculations {
            pub fn factorial(n: u32) -> u64 {
                if n <= 1 {
                    1
                } else {
                    (1..=n as u64).product()
                }
            }

            pub fn fibonacci(n: u32) -> u64 {
                match n {
                    0 => 0,
                    1 => 1,
                    _ => {
                        let mut a = 0;
                        let mut b = 1;
                        for _ in 2..=n {
                            let temp = a + b;
                            a = b;
                            b = temp;
                        }
                        b
                    }
                }
            }
        }
    }

    /// 字符串工具
    pub mod strings {
        /// 文本处理
        pub mod text_processing {
            pub fn reverse_string(s: &str) -> String {
                s.chars().rev().collect()
            }

            pub fn count_words(text: &str) -> usize {
                text.split_whitespace().count()
            }

            pub fn capitalize_first_letter(s: &str) -> String {
                let mut chars = s.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => first.to_uppercase().collect::<String>() + &chars.as_str(),
                }
            }
        }

        /// 格式化工具
        pub mod formatting {
            pub fn format_currency(amount: f64) -> String {
                format!("¥{:.2}", amount)
            }

            pub fn format_percentage(value: f64) -> String {
                format!("{:.2}%", value * 100.0)
            }
        }
    }
}

pub fn demo() {
    println!("--- 嵌套模块 ---");

    // 游戏模块演示
    println!("\n1. 游戏模块演示:");
    
    // 创建玩家
    let mut player = game::player::Player::new("英雄".to_string());
    player.display_info();

    // 添加物品到背包
    let potion = game::items::presets::health_potion();
    let sword = game::items::presets::sword();
    
    if let Err(e) = player.inventory.add_item(potion) {
        println!("错误: {}", e);
    }
    if let Err(e) = player.inventory.add_item(sword) {
        println!("错误: {}", e);
    }

    player.display_info();

    // 升级
    player.level_up();
    player.display_info();

    // 战斗演示
    let mut enemy = game::combat::Enemy::new("哥布林".to_string(), 50, 8, 2);
    let battle_won = game::combat::CombatSystem::battle(&mut player, &mut enemy);
    
    if battle_won {
        println!("战斗胜利！获得经验。");
    } else {
        println!("战斗失败！");
    }

    // 工具模块演示
    println!("\n2. 工具模块演示:");

    // 几何计算
    let point1 = tools::math::geometry::Point::new(0.0, 0.0);
    let point2 = tools::math::geometry::Point::new(3.0, 4.0);
    println!("点距离: {:.2}", point1.distance_to(&point2));

    let rect = tools::math::geometry::Rectangle::new(5.0, 3.0);
    println!("矩形面积: {}, 周长: {}", rect.area(), rect.perimeter());

    // 数值计算
    println!("5! = {}", tools::math::calculations::factorial(5));
    println!("斐波那契数列第10项: {}", tools::math::calculations::fibonacci(10));

    // 字符串处理
    let text = "hello world";
    println!("原文: {}", text);
    println!("反转: {}", tools::strings::text_processing::reverse_string(text));
    println!("单词数: {}", tools::strings::text_processing::count_words(text));
    println!("首字母大写: {}", tools::strings::text_processing::capitalize_first_letter(text));

    // 格式化
    println!("货币格式: {}", tools::strings::formatting::format_currency(1234.56));
    println!("百分比格式: {}", tools::strings::formatting::format_percentage(0.75));

    println!("\n嵌套模块演示完成！");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_modules() {
        let mut player = game::player::Player::new("测试玩家".to_string());
        assert_eq!(player.name, "测试玩家");
        assert_eq!(player.level, 1);
        
        player.level_up();
        assert_eq!(player.level, 2);
    }

    #[test]
    fn test_geometry() {
        let rect = tools::math::geometry::Rectangle::new(4.0, 3.0);
        assert_eq!(rect.area(), 12.0);
        assert_eq!(rect.perimeter(), 14.0);
    }

    #[test]
    fn test_calculations() {
        assert_eq!(tools::math::calculations::factorial(5), 120);
        assert_eq!(tools::math::calculations::fibonacci(10), 55);
    }
}