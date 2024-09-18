trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // 提供默认方法
    fn collect(&mut self) -> Vec<Self::Item> {
        let mut collection = Vec::new();
        while let Some(item) = self.next() {
            collection.push(item);
        }
        collection
    }
}

struct Counter {
    count: i32,
}

impl Iterator for Counter {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

fn main() {
    let mut counter = Counter { count: 0 };
    let collected = counter.collect();
    println!("{:?}", collected); // 输出: [1, 2, 3, 4, 5]
}



/* another implementation */
trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // 提供默认方法
    fn collect(mut self) -> Vec<Self::Item> where Self: Sized {
        let mut collection = Vec::new();
        while let Some(item) = self.next() {
            collection.push(item);
        }
        collection
    }
}

struct Counter {
    count: i32,
}

impl Iterator for Counter {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

fn main() {
    let counter = Counter { count: 0 };
    let collected = counter.collect();
    println!("{:?}", collected); // 输出: [1, 2, 3, 4, 5]
}
