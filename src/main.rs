#[cfg(test)]
mod test;

use std::collections::HashMap;

fn memoization<F, U, V>(f: F) -> impl FnMut(U) -> V
where
    F: Fn(U) -> V,
    U: std::fmt::Display + std::cmp::Eq + std::hash::Hash + std::marker::Copy,
    V: std::marker::Copy,
{
    let mut cache: HashMap<U, V> = HashMap::new();
    return move |x| {
        println!("New ({})", x);

        match cache.get(&x) {
            Some(&result) => result,
            _ => {
                let result = f(x);
                cache.insert(x, result);
                result
            }
        }
    };
}

fn f(x: i32) -> i32 {
    println!("Base({})", x);
    x
}

fn j(x: i32) -> bool {
    println!("Base({})", x);
    x % 2 == 0
}

fn main() {
    let mut g = memoization(f);

    println!("function g :\n");

    g(1);
    g(1);
    g(0);
    g(1);
    g(0);

    let mut i = memoization(j);

    println!("\nfunction j :\n");
    i(1);
    i(1);
    i(0);
}
