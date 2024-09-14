// 参数是按值传递的，意味着当参数传递给函数时，会发生所有权的转移
// 如果你想让参数在函数调用后仍然保留原始的所有权，你可以传递引用
fn add_one(x: &mut i32) {
    *x += 1;
}

fn main() {
    let mut num = 5;
    add_one(&mut num);
    println!("num is now: {}", num); // 输出: num is now: 6
}


fn square(x: i32) -> i32 {
    x * x // 隐式返回值
}

fn main() {
    let result = square(4);
    println!("Square of 4 is: {}", result);
}


// unit type单位类型，unit value单元值 ()
fn square(x: i32) -> () {
    let _ = x * x;
}

fn main() {
    let result = square(4);
    println!("Square of 4 is: {:?}", result);
}


// 函数指针
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn apply_operation(op: fn(i32, i32) -> i32, x: i32, y: i32) -> i32 {
    op(x, y)
}

fn main() {
    let result = apply_operation(add, 2, 3);
    println!("Result: {}", result);
}


// 高阶函数
fn is_even(n: &i32) -> bool {
    n & 1 == 0
}
/*
fn filter(arr: Vec<i32>, predicate: fn(&i32) -> bool) -> Vec<i32> {
    arr.into_iter().filter(predicate).collect()
}
*/
fn filter<F>(arr: Vec<i32>, predicate: F) -> Vec<i32>
where
    F: FnMut(&i32) -> bool,
{
    arr.into_iter().filter(predicate).collect()
}

fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6];
    let even_numbers = filter(numbers, is_even);
    println!("{:?}", even_numbers);
}


fn is_even(n: i32) -> bool {
    n & 1 == 0
}
// wrapper in filter
fn filter(arr: Vec<i32>, predicate: fn(i32) -> bool) -> Vec<i32> {
    arr.into_iter().filter(|arg0: &i32| predicate(*arg0)).collect()
}

fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6];
    let even_numbers = filter(numbers, is_even);
    println!("{:?}", even_numbers);
}


// 闭包的捕获方式

// 闭包可以以三种不同的方式捕获变量：

//     按值捕获：闭包获取变量的所有权。
//     按引用捕获：闭包获取变量的不可变引用。
//     按可变引用捕获：闭包获取变量的可变引用
fn main() {
    let x = 4;
    let square = |n: i32| n * n + x;
    println!("{}", square(2));
}


fn main() {
    let mut square = make();
    println!("{}", square(1));
    println!("{}", square(1));
    println!("{}", square(1));
}
fn make() -> impl FnMut(i32) -> i32 {
    let mut x = 9;
    move |n: i32| { // 使用 move 关键字让闭包按值捕获，获得所有权
        x -= 1;
        println!("{x}");
        n * n + x
    }
}


// 闭包的生命周期由它捕获的引用决定



fn standalone_function() {
    // code
}

pub fn public_thing(argument: bool) -> String {
    // code
}

struct Thing {
    foo: i32,
}

impl Thing {
    pub fn new() -> Self {
        Self {
            foo: 42,
        }
    }
}

fn generic_function<T: Clone>(x: T) -> (T, T, T) {
    (x.clone(), x.clone(), x.clone())
}

fn generic_where<T>(x: T) -> T
    where T: std::ops::Add<Output = T> + Copy
{
    x + x + x
}

fn main() {
    // let result = generic_where::<i32>(5);
    let result = generic_where(5); // 编译器自动确定泛型类型
    println!("Result: {}", result); // Result: 15
}
