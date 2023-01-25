use std::vec;

fn new_stack(maxsize: usize) -> Vec<String> {
    let vec: Vec<String> = Vec::with_capacity(maxsize);
    vec
}

fn pop(stack: &mut Vec<String>) -> Option<String> {
    let popped_val = stack.pop();
    // println!("The popped value is: {:?}", popped_val);
    popped_val
}

fn push(stack: &mut Vec<String>, item: String, maxsize: usize) {
    if stack.len() == maxsize {
        println!("Cannot add more");
    } else {
        stack.push(item);
        // println!("Stack: {:?}", stack)
    }
}

fn size(stack: &Vec<String>) -> usize {
    stack.len()
}

fn infix_to_postfix(input: Vec<String>) -> Vec<String> {
    let size_expr: usize = input.len();
    let mut stack: Vec<String> = new_stack(size_expr);
    let mut postfix: Vec<String> = Vec::new();

    for i in input {
        match i.as_str() {
            "+" | "-" | "*" | "/" | "^" => {
                if size(&stack) == 0 {
                    push(&mut stack, i, size_expr);
                } else {
                    if priority(&i) > priority(stack.last().unwrap()) {
                        push(&mut stack, i, size_expr)
                    } else {
                        while priority(&i) <= priority(stack.last().unwrap()) {
                            postfix.push(pop(&mut stack).unwrap());
                            if stack.last() == None {
                                break;
                            }
                        }

                        push(&mut stack, i, size_expr);
                    }
                }
            }

            "(" => push(&mut stack, i, size_expr),

            ")" => {
                while stack.last().unwrap() != "(" {
                    postfix.push(pop(&mut stack).unwrap());
                }
                pop(&mut stack).unwrap();
            }

            _ => postfix.push(i),
        };
    }

    while size(&stack) != 0 {
        postfix.push(pop(&mut stack).unwrap());
    }

    println!("{:?}", postfix);

    postfix
}

fn priority(x: &String) -> u8 {
    if ("+" == x) | ("-" == x) {
        1
    } else if ("*" == x) | ("/" == x) {
        2
    } else if "^" == x {
        3
    } else {
        0
    }
}

fn individual_symbols(input_expr: String) -> Vec<String> {
    let mut tokenized_input: Vec<String> = Vec::new();
    let input_chars: Vec<char> = input_expr.chars().collect();
    let mut temp: Vec<char> = Vec::new();

    for c in input_chars {
        if c != '+' && c != '-' && c != '/' && c != '*' && c != '^' && c != '(' && c != ')' {
            temp.push(c);
            continue;
        } else {
            if temp.len() == 0 {
                tokenized_input.push(c.to_string());
            } else {
                tokenized_input.push(temp.into_iter().collect());
                tokenized_input.push(c.to_string());
                temp = vec![];
            }
        }
    }

    if temp.len() != 0 {
        tokenized_input.push(temp.into_iter().collect());
    }

    println!("{:?}", tokenized_input);
    tokenized_input
}

fn postfix_evaluation(postfix: Vec<String>) -> f32 {
    let size_expr: usize = postfix.len();
    let mut result_stack: Vec<String> = new_stack(size_expr);
    for i in postfix {
        match i.as_str() {
            "+" | "-" | "*" | "/" | "^" => {
                let oper: String = i;
                let op2: String = pop(&mut result_stack).unwrap();
                let op1: String = pop(&mut result_stack).unwrap();
                let result: f32 = operation(op1, op2, oper);

                push(&mut result_stack, result.to_string(), size_expr);
            }

            _ => push(&mut result_stack, i.to_string(), size_expr),
        }
    }

    pop(&mut result_stack).unwrap().parse::<f32>().unwrap()
}

fn operation(op1: String, op2: String, oper: String) -> f32 {
    let op1: f32 = op1.parse::<f32>().unwrap();
    let op2: f32 = op2.parse::<f32>().unwrap();
    let result: f32 = match oper.as_str() {
        "+" => op1 + op2,
        "-" => op1 - op2,
        "*" => op1 * op2,
        "/" => op1 / op2,
        "^" => op1.powf(op2),
        _ => 0.0,
    };

    result
}

fn main() {
    let input_expr: String = String::from("(33+45/3*(2+9)-50)");
    println!("The original expression is {:?}", input_expr);
    let input_expr_tokenized: Vec<String> = individual_symbols(input_expr);
    let postfix: Vec<String> = infix_to_postfix(input_expr_tokenized);
    let result = postfix_evaluation(postfix);

    println!("Result: {}", result);
}
