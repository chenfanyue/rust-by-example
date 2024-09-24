use std::collections::BTreeSet;

fn main() {
    // 创建一个空的 BTreeSet
    let mut set = BTreeSet::new();

    // 插入元素
    set.insert(3);
    set.insert(1);
    set.insert(4);
    set.insert(1);  // 插入重复元素不会改变集合
    set.insert(2);

    // 打印集合，元素将按升序排列
    println!("BTreeSet: {:?}", set);

    // 检查某个元素是否存在
    if set.contains(&3) {
        println!("集合包含元素 3");
    }

    // 删除元素
    set.remove(&2);
    println!("删除元素 2 后的集合: {:?}", set);

    // 遍历集合中的元素
    for x in &set {
        println!("遍历元素: {}", x);
    }

    // 范围查询，找到 1 到 3 之间的元素（包括 3）
    let range: Vec<_> = set.range(1..=3).cloned().collect();
    println!("范围查询结果: {:?}", range);
}
