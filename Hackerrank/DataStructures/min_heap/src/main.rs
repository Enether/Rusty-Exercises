#[derive(Debug)]
struct Heap {
    values: Vec<i32>
}

enum CommandTypes {
    ADD,
    REMOVE,
    PRINT_MIN,
}

fn get_command_type(i: i32) -> CommandTypes {
    match i {
        1 => CommandTypes::ADD,
        2 => CommandTypes::REMOVE,
        3 => CommandTypes::PRINT_MIN,
        _ => panic!("Invalid Command Type!"),
    }
}

impl Heap {
    fn add(&mut self, val: i32) {
        self.values.push(val);
        let index: usize = self.values.len()-1;
        self.heapify_up(index);
    }

    fn remove(&mut self, val: i32) {
        // find the index of the value
        let index: usize = self.values.iter().position(|&x| x == val).unwrap();
        self.values.swap_remove(index);  // remove and replace with the last value
        self.heapify_down(index);
    }

    fn heapify_up(&mut self, original_index: usize) {
        if original_index == 0 {
            return;
        }
        let mut index = original_index;
        let mut parent_index: usize = (index - 1) / 2;

        while index != 0 && self.values[parent_index] > self.values[index] {
            
            // switch them up
            let temp: i32 = self.values[index];
            self.values[index] = self.values[parent_index];
            self.values[parent_index] = temp;

            index = parent_index;
            if index == 0 {
                break;
            }
            parent_index = (index - 1) / 2;
        }
    }

    fn heapify_down(&mut self, original_index: usize) {
        let mut index: usize = original_index;
        let max_len = self.values.len();
        let mut first_child_idx: usize = (2 * index) + 1;
        let mut second_child_idx: usize = (2 * index) + 2;
        
        loop {
            if first_child_idx < max_len && second_child_idx < max_len {
                let min_el_idx = self.get_min_element(first_child_idx, second_child_idx);
                if self.values[index] < self.values[min_el_idx] {
                    break;
                }
                self.swap_elements(index, min_el_idx);
                index = min_el_idx;
                first_child_idx = (2 * index) + 1;
                second_child_idx = (2 * index) + 2;
            } else if first_child_idx >= max_len {
                // no children
                break;
            } else {
                // only 1 child
                if self.values[index] < self.values[first_child_idx] {
                    break;
                }
                self.swap_elements(index, first_child_idx);
                break;
            }
        }
    }

    fn swap_elements(&mut self, idx_one: usize, idx_two: usize) {
        let temp: i32 = self.values[idx_one];
        self.values[idx_one] = self.values[idx_two];
        self.values[idx_two] = temp;
    }

    fn get_min_element(&self, idx_one: usize, idx_two: usize) -> usize {
        let first_is_smaller: bool = self.values[idx_one] < self.values[idx_two];
        match first_is_smaller {
            true => idx_one,
            false => idx_two,
        }
    }
}

fn main() {
    let stdin = std::io::stdin();
    let query_count_str = &mut String::new();
    stdin.read_line(query_count_str);
    let query_count: i32 = query_count_str.trim().parse().unwrap();

    let mut heap: Heap = Heap {values: vec![]};
    
    for _ in 0..query_count {
        let command_str = &mut String::new();
        stdin.read_line(command_str);
        let command_args: Vec<i32> = command_str.trim().split_whitespace().map(|a| a.parse().unwrap()).collect();

        let command_type = get_command_type(command_args[0]);

        match command_type {
            CommandTypes::PRINT_MIN => println!("{}", heap.values[0]),
            CommandTypes::ADD => heap.add(command_args[1]),
            CommandTypes::REMOVE => heap.remove(command_args[1]),
        }
    }
}

#[cfg(test)]
mod tests {
    use Heap;

    #[test]
    fn test_heap_default_input() {
        heap.add(4);
        let mut heap: Heap = Heap {values: vec![]};
        heap.add(9);
        heap.remove(4);
        assert!(heap.values[0] == 9);
    }
}