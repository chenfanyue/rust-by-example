#![feature(coroutines, coroutine_trait, stmt_expr_attributes)]

use std::ops::{Coroutine, CoroutineState};
use std::pin::Pin;

fn main() {
    let mut coroutine = #[coroutine] || {
        yield 1;
        yield 2;
        yield 3;
    };

    // 使用 Pin 来固定coroutine的内存位置
    let mut pinned_coroutine = Pin::new(&mut coroutine);

    // 手动调用生成器
    assert_eq!(pinned_coroutine.as_mut().resume(()), CoroutineState::Yielded(1));
    assert_eq!(pinned_coroutine.as_mut().resume(()), CoroutineState::Yielded(2));
    assert_eq!(pinned_coroutine.as_mut().resume(()), CoroutineState::Yielded(3));
    assert_eq!(pinned_coroutine.as_mut().resume(()), CoroutineState::Complete(()));
}
