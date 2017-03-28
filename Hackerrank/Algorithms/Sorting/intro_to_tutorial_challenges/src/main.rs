use std::io;

fn binary_search(array: &Vec<i32>, value: i32, mut start_index: usize, mut end_index: usize) -> Option<usize> {
    if start_index == end_index || end_index < 0 || start_index >= array.len() {
        return None;
    }
    let midIndex = start_index + ((end_index - start_index) / 2);
    
    let foundValue = array[midIndex];
    if foundValue == value {
        return Some(midIndex);
    } else if foundValue > value {
        // go left
        end_index = midIndex;
        return binary_search(array, value, start_index, end_index)
    } else {
        // go right
        start_index = midIndex + 1;
        return binary_search(array, value, start_index, end_index)
    }
}

fn main() {
    let mut stdin = io::stdin();
    let mut value_to_search_str = &mut String::new();
    stdin.read_line(&mut value_to_search_str);

    let mut arr_size_str = &mut String::new();
    stdin.read_line(&mut arr_size_str);

    let mut arr_str = &mut String::new();
    stdin.read_line(&mut arr_str);
    let arr_size: i32 = arr_size_str.trim().parse().unwrap();
    let value_to_search: i32 = value_to_search_str.trim().parse().unwrap();
    let arr: Vec<i32> = arr_str.split_whitespace().map(|x| x.parse().unwrap()).collect();
    
    println!("{:?}", binary_search(&arr, value_to_search, 0, arr.len()).unwrap());
}

#[cfg(test)]
mod tests {
    use super::binary_search;
    
    #[test]
    fn test_all_numbers() {
        let mut numbers = vec![1,2,3,4,5,6,7,8,9,10];
        for num in -10..11 {
            let el_position = numbers.iter().position(|x| x.clone()==num);
            match el_position {
                Some(x) => assert!(x == binary_search(&numbers, num, 0, numbers.len()).unwrap() ),
                None => assert!(None == binary_search(&numbers, num, 0, numbers.len())),
            }
        }
    }
}
