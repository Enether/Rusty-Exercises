pub fn last<T: Clone>(slice: &[T]) -> T {
    let a = slice[slice.len()-1].clone() as T;
    return a;
}
