use std::io;

fn calculate_gcd(a: i32, b: i32) -> i32 {
    /* Find the greater common denominator between two numbers. */
    let mut x = a;
    let mut y = b;
    while y > 0 {
        let temp = x % y;
        x = y;
        y = temp;
    }

    x
}

fn calculate_lcm(a: i32, b: i32) -> i32 {
    a * b / calculate_gcd(a, b)
}

fn calculate_vector_lcm(vec: Vec<i32>) -> i32 {
    if vec.len() == 1 {
        return vec[0];
    }
    let mut lcm = calculate_lcm(vec[0], vec[1]);
    for i in 2..vec.len() {
        lcm = calculate_lcm(lcm, vec[i]);
    }
    lcm
}

fn calculate_vector_gcd(vec: Vec<i32>) -> i32 {
    if vec.len() == 1 {
        return vec[0];
    }
    let mut gcd = calculate_gcd(vec[0], vec[1]);
    for i in 2..vec.len() {
        gcd = calculate_gcd(gcd, vec[i]);
    }
    gcd
}

fn calculate_count_multiples_that_divide_evenly_by_x(start_number: i32, x: i32) -> i32 {
    let mut count = 0;
    let mut start = start_number;
    while start <= x {
        if x % start == 0 {
            count += 1;
        }
        start += start_number;
    }
    count
}

fn calculate_count_of_x_between_two_sets(set_a: Vec<i32>, set_b: Vec<i32>) -> i32 {
    /*
    *    We can get the amount of X by :
    *    1. Getting the Least Common Multiple of the
    *    vector with the smaller values and the Greater Common Divisor of the vector
    *    with the greater values.
    *    2. Finding the number of multiples of the LCM that divide evenly with the GCD
    */
    let a_lcm: i32 = calculate_vector_lcm(set_a);
    let b_gcd: i32 = calculate_vector_gcd(set_b);
    
     calculate_count_multiples_that_divide_evenly_by_x(a_lcm, b_gcd)
}

fn main() {
    let stdin = io::stdin();
    let mut _useless_ = &mut String::new();
    stdin.read_line(_useless_);

    let mut set_a_str = &mut String::new();
    stdin.read_line(set_a_str);
    let mut set_a: Vec<i32> = set_a_str.split_whitespace().map(|x| x.parse().unwrap()).collect();

    let mut set_b_str = &mut String::new();
    stdin.read_line(set_b_str);
    let mut set_b: Vec<i32> = set_b_str.split_whitespace().map(|x| x.parse().unwrap()).collect();

    println!("{}", calculate_count_of_x_between_two_sets(set_a, set_b));
}

#[cfg(test)]
mod test {
    use calculate_count_of_x_between_two_sets;
    #[test]
    fn default_test() {
        assert!(calculate_count_of_x_between_two_sets(vec![2, 4], vec![16, 32, 96]) == 3);
    }
}
