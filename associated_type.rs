/** 具名字段结构体 */
trait Container {
    type Item;
    fn add(&mut self, item: Self::Item);
    fn remove(&mut self) -> Self::Item;
}

struct IntContainer {
    items: Vec<i32>,
}

impl Container for IntContainer {
    type Item = i32;

    fn add(&mut self, item: Self::Item) {
        self.items.push(item);
    }

    fn remove(&mut self) -> Self::Item {
        self.items.pop().unwrap()
    }
}

fn main() {
    let mut a = IntContainer {
        items: vec![],
    };
    a.add(1001);
    a.remove();
}


/** 元组结构体 */
trait Container {
    type Item;
    fn add(&mut self, item: Self::Item);
    fn remove(&mut self) -> Self::Item;
}

struct IntContainer(Vec<i32>);

impl Container for IntContainer {
    type Item = i32;

    fn add(&mut self, item: Self::Item) {
        self.0.push(item);
    }

    fn remove(&mut self) -> Self::Item {
        self.0.pop().unwrap()
    }
}

fn main() {
    let mut a = IntContainer(vec![1, 2, 3]);
    a.add(1001);
    a.remove();
}

