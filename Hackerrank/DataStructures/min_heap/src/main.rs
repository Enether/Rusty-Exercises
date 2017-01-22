#![allow(dead_code)]
#![allow(unused_variables)]


#[derive(Debug)]
struct Heap {
    values: Vec<i32>
}

impl Heap {
    fn add(&mut self, val: i32) {
        self.values.push(val);
        let a = self.values.len()-1;
        self.heapify_up(a);
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
}

fn main() {
    let mut heap: Vec<i32> = vec![5];
    let mut hp: Heap = Heap { values: heap};
    hp.add(3);
    hp.add(2);

    hp.add(10);
    hp.add(1);
    
    
    println!("{:?}", hp);
}
