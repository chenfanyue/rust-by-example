fn main() {
    let number = 5;

    if number < 10 {
        println!("The number is less than 10");
    } else if number == 10 {
        println!("The number is exactly 10");
    } else {
        println!("The number is greater than 10");
    }
}

fn main() {
    let condition = true;
    let number = if condition { 5 } else { 10 };

    println!("The value of number is: {number}");
}

fn main() {
    let some_value = Some(5);

    // 使用 if let 进行模式匹配
    if let Some(x) = some_value {
        println!("The value is: {}", x);
    } else {
        println!("No value found.");
    }
}

fn main() {
    let some_value = Some(5);

    match some_value {
        Some(x) => {
            println!("The value is: {}", x);
        }
        None => { // _ => {
            println!("No value found.");
        }
    }
}

fn main() {
    let some_value = Some(5);

    match some_value {
        Some(x) if x > 0 => println!("Positive number: {}", x),
        Some(x) if x < 0 => println!("Negative number: {}", x),
        Some(_) => println!("Zero"),
        None => println!("No value found."),
    }
}

fn main() {
    let number = 3;

    match number {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        4 | 5 | 6 => println!("Four, Five or Six"), // 可以匹配多个值
        7..=10 => println!("Seven to Ten"),        // 可以匹配一个范围
        _ => println!("Some other number"),       // 默认分支
    }
}

fn main() {
    let some_value = Some(7);

    // 仅当值为 Some 且满足额外条件时执行
    if let Some(x) = some_value && x > 5 {
        println!("The value is greater than 5: {}", x);
    }
}


fn main() {
    let mut counter = 0;

    loop {
        counter += 1;
        println!("Counter: {}", counter);

        if counter == 5 {
            break;
        }
    }
}

fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }

    println!("Liftoff!");
}

fn main() {
    for number in 1..4 { // 1..=4 闭区间
        println!("{}!", number);
    }

    println!("Liftoff!");
}

fn main() {
    let a = [10, 20, 30, 40, 50];

    // 借用的例子
    for element in a.iter() { // binding element: &i32
        println!("The value is: {}", element);
    }
    println!("{:?}", a);
}

fn main() {
    let a = [10, 20, 30, 40, 50];

    // 所有权的例子
    for element in a.into_iter() { // element: i32
        println!("The value is: {}", element);
    }
    println!("{:?}", a);
}

fn main() {
    // Vector 向量是动态数组
    let mut v: Vec<i32> = vec![10, 20, 30, 40, 50];
    v[0] = 1;        // 修改元素
    v.push(60);      // 添加新元素
    let Some(elem) = v.pop() else { todo!() }; // let else
    println!("{:?}", v);
}

fn main() {
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("Popped value: {}", top);
    }
}
