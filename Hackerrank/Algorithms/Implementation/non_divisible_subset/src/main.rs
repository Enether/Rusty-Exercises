use std::collections::HashMap;

fn build_occurences(numbers: Vec<i32>) -> HashMap<i32, i32> {
    /* Builds a hashmap holding the number of times we've found the number in the given vector */
    let mut occurences = HashMap::new();
    for num in numbers {
        let num_occ = occurences.entry(num).or_insert(0);
        *num_occ += 1;
    }

    occurences
}

fn get_subset_count(possible_remainders: Vec<i32>, occurences: HashMap<i32, i32>) -> i32{
    /* Get the maximum count of a subset which does not divide evenly by K
     * Choose the maximum of each opposite remainder (ex k=5, remainders=[1,2,3,4], choose between 1/4 and 2/3 (since 1+4==5, we want to have only one kind of
     *       numbers with such a remainder))
    */
    let remainders_len = possible_remainders.len();
    let mut count: i32 = 0;

    for i in (0..possible_remainders.len()/2) {
        let reversed_idx = (remainders_len - (i + 1)) as i32;
        let i_occ = occurences.get(&(possible_remainders[i] as i32)).unwrap_or(&0).clone();
        let ri_occ = occurences.get(&possible_remainders[reversed_idx as usize]).unwrap_or(&0).clone();
        count += std::cmp::max(i_occ, ri_occ);
    }
    
    count
}

fn main() {
    /* 1.Create a list of all the possible remainders and the number of numbers which have that remainder
     * 2.Choose the maximum of each opposite remainder (ex k=5, remainders=[1,2,3,4], choose between 1/4 and 2/3 (since 1+4==5, we want to have only one kind of
     *       numbers with such a remainder))
     * 3.If the number of remainders is odd, add ONE number of the middle remainder (ex k=6, remainders=[1,2,3,4,5], we can take 3 only once, since 3+3==6)
     * 4.We can also take ONE number whose remainder is 0, since he won't result in a number divisible by K when summed with any other number.
    */
    let mut line = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut line);
    let n_k: Vec<i32> = line.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    let k: i32 = n_k.last().unwrap().clone();
    line = String::new();
    stdin.read_line(&mut line);
    let numbers: Vec<i32> = line.split_whitespace().map(|x| x.parse::<i32>().unwrap() % k).collect();

    let mut occurences = build_occurences(numbers);
    /* a vector of the possible remainders that are of interest to us
    *  ex: k=5 - remainders: [1,2,3,4]
    */
    let possible_remainders: Vec<i32> = (1..k).collect();
    let remainders_len = possible_remainders.len();

    // 3.
    let mut subset_count = get_subset_count(possible_remainders, occurences.clone());

    // If there is a middle remainder, take one
    if remainders_len % 2 != 0 {
        let mid_remainder: &i32 = &((remainders_len/2 + 1) as i32);
        subset_count += if occurences.get(mid_remainder).unwrap_or(&0).clone() != 0 {1} else {0};
    }
    // Take one number whose remainder is 0, if possible
    subset_count += if occurences.get(&0).unwrap_or(&0).clone() != 0 {1} else {0};
    println!("{:?}", subset_count);
}