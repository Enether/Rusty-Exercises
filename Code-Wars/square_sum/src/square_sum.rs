pub fn square_sum(vec: Vec<i32>) -> i32 {
    let mut sum: i32 = 0;
    for el in vec {
        sum += el.pow(2);
    }
    return sum;
}