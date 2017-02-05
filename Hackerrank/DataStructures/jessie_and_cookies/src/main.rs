#[derive(Debug)]
struct Heap {
    values: Vec<i32>
}

impl Heap {
    fn add(&mut self, val: i32) {
        self.values.push(val);
        let index: usize = self.values.len()-1;
        self.heapify_up(index);
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

    fn pop(&mut self) -> i32 {
        let min_element: i32 = self.values[0];

        self.values.swap_remove(0);  // remove and replace with the last value
        self.heapify_down(0);

        min_element
    }

    fn peek_min_element(&mut self) -> i32 {
        self.values[0]
    }

    fn len(&mut self) -> usize {
        self.values.len()
    }
}

fn read_input() -> (i32, Vec<i32>) {
    let stdin = std::io::stdin();
    let n_k_str = &mut String::new();
    stdin.read_line(n_k_str);
    let n_k: Vec<i32> = n_k_str.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
    let k: i32 = n_k[1];

    let cookies_str = &mut String::new();
    stdin.read_line(cookies_str);
    let cookies: Vec<i32> = cookies_str.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();

   (k, cookies)
}

fn reach_sweetness(mut heap: Heap, k: i32) -> (bool, i32) {
    /* Run a loop while the min element is below the desired sweetness
     * Get the min_element and check if there are any left
     *    (if there are not, we can't mix the cookie and therefore can't reach the desired sweetness)
     * If there are left, get the second_min_element, create the new cookie and add it back
     */
    let mut reached_sweetness: bool = true;
    let mut steps: i32 = 0;
    while heap.peek_min_element() < k {
        // remove cookies to increase the sweetness
        let least_sweet_cookie: i32 = heap.pop();
        if heap.len() == 0 {
            // heap is empty, therefore we can't reach the desired sweetness
            reached_sweetness = false;
            break;
        }
        let second_least_sweet_cookie: i32 = heap.pop();
        let new_cookie: i32 = least_sweet_cookie + (2 * second_least_sweet_cookie);

        heap.add(new_cookie);
        steps += 1;
    }

    (reached_sweetness, steps)
}

fn main() {
    let (k, cookies) = read_input();

    let mut heap: Heap = Heap { values: vec![]};
    // add to the heap
    for i in cookies {
        heap.add(i);
    }

    let (reached_sweetness, steps) = reach_sweetness(heap, k);
    println!("{}", if reached_sweetness  {steps} else {-1});
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
