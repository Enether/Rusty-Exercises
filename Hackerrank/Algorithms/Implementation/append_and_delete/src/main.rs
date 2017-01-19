fn main() {
    let stdin = std::io::stdin();
    let first_str = &mut String::new();
    stdin.read_line(first_str);
    let first_str: &str = first_str.trim();

    let second_str = &mut String::new();
    stdin.read_line(second_str);
    let second_str: &str = second_str.trim();

    let operation_count_str = &mut String::new();
    stdin.read_line(operation_count_str);
    let operations_count: i32 = operation_count_str.trim().parse().unwrap();

    let mut equal_up_to_idx: i32 = 0;
    let mut first_str_chars = first_str.chars();
    let mut second_str_chars = second_str.chars();
    loop {
        let f_chr = first_str_chars.next();
        let s_chr = second_str_chars.next(); 
        if (f_chr != None && s_chr != None && f_chr == s_chr) {
            equal_up_to_idx += 1;
        } else {
            break;
        }
    }
    let count_to_remove: i32 = first_str.len() as i32 - equal_up_to_idx;
    
    if count_to_remove < 0 {
        println!("No");
        return;
    }

    let count_to_append: i32 = second_str.len() as i32 - equal_up_to_idx;
    
    let needed_operations: i32 = count_to_append + count_to_remove;
    if needed_operations == operations_count {
        println!("Yes");
    } else if needed_operations < operations_count {
        let difference: i32 = operations_count - needed_operations;
        if equal_up_to_idx == 0 {
            println!("Yes");
        } else if difference % 2 == 0{
            println!("Yes");
        }else if (difference <= equal_up_to_idx && difference % 2 != 0) {
            println!("No");
        }
        else if difference >= (equal_up_to_idx * 2) {
            println!("Yes");
        } else {
            println!("No");
        }
    } else {
        println!("No");
    }
}   