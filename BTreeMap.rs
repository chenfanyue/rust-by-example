use std::collections::BTreeMap;

fn main() {
    // 创建一个空的 BTreeMap
    let mut map = BTreeMap::new();

    // 插入键值对
    map.insert(3, "three");
    map.insert(1, "one");
    map.insert(4, "four");
    map.insert(2, "two");

    // 打印 BTreeMap，键会按升序排列
    println!("BTreeMap: {:?}", map);

    // 查找某个键对应的值
    if let Some(value) = map.get(&3) {
        println!("键 3 对应的值是: {}", value);
    }

    // 删除一个键值对
    map.remove(&2);
    println!("删除键 2 后的 BTreeMap: {:?}", map);

    // 遍历所有键值对
    for (key, value) in &map {
        println!("遍历: 键: {}, 值: {}", key, value);
    }

    // 使用范围查询，找到键从 1 到 3 之间的所有键值对
    let range: Vec<_> = map.range(1..=3).collect();
    println!("范围查询结果: {:?}", range);
}
