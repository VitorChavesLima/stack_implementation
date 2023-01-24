fn new_stack(maxsize: usize) -> Vec<char> {
    let vec: Vec<char> = Vec::with_capacity(maxsize);
    vec
}

fn pop(stack: &mut Vec<char>) -> Option<char> {
    let popped_val = stack.pop();
    // println!("The popped value is: {:?}", popped_val);
    popped_val
}

fn push(stack: &mut Vec<char>, item: char, maxsize: usize) {
    if stack.len() == maxsize {
        // println!("Cannot add more");
    } else {
        stack.push(item);
        // println!("Stack: {:?}", stack)
    }
}

fn size(stack: &Vec<char>) -> usize {
    stack.len()
}

fn input() -> u32 {
    let mut n: String = String::new();
    std::io::stdin()
        .read_line(&mut n)
        .expect("Failed to read input");

    let n: u32 = n.trim().parse().expect("Invalid input");
    n
}

fn main() {
    let input_string: String = String::from("Roma é amor");
    let size_stack = input_string.len();
    let mut stack = new_stack(size_stack);

    let mut rev_string = String::new();

    for c in input_string.chars() {
        push(&mut stack, c, size_stack);
    }

    for _ in 0..size(&stack) {
        let popped_char: char = pop(&mut stack).unwrap();
        rev_string.push(popped_char);
    }

    println!("Original string: {:?}", input_string);
    println!("Reversed string: {:?}", rev_string);
}
