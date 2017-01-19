use std::io;


fn calculate_energy(clouds: Vec<i32>, clouds_count: i32, jump_distance: i32) -> i32 {
    let mut energy: i32 = 100;

    // first jump
    let mut position = jump_distance % clouds_count;
    energy = 99;

    while position != 0 {
        if clouds[position as usize] == 1 {
            energy -= 2;
        }
        energy -= 1;
        position = (position + jump_distance) % clouds_count;
    }
    if clouds[position as usize] == 1 {
            energy -= 2;
        }
    energy
}


fn main() {
    let stdin = io::stdin();

    let clouds_jump_dist_str = &mut String::new();
    stdin.read_line(clouds_jump_dist_str);
    let clouds_jump_dist: Vec<i32> = clouds_jump_dist_str.split_whitespace().map(|x| x.parse().unwrap()).collect();
    let clouds_count: i32 = clouds_jump_dist[0];
    let jump_distance: i32 = clouds_jump_dist[1];

    let clouds_str = &mut String::new();
    stdin.read_line(clouds_str);
    let clouds: Vec<i32> = clouds_str.split_whitespace().map(|x| x.parse().unwrap()).collect();
    let energy: i32 = calculate_energy(clouds, clouds_count, jump_distance);
    println!("{}", energy);
}

#[cfg(test)]
mod test {
    use calculate_energy;

    #[test]
    fn test_default() {
        assert!(calculate_energy(vec![0, 0, 1, 0, 0, 1, 1, 0], 8, 2) == 92);
    }
}