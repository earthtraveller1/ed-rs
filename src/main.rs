use std::io::Stdin;

fn main() {
    let mut lines = Vec::<String>::new();
    let mut current_line = 0;

    let stdin = std::io::stdin();

    let mut running = true;
    while running {
        let mut input = String::new();
        if let Err(_) = stdin.read_line(&mut input) {
            println!("?");
            continue;
        }

        match input.trim().chars().nth(0) {
            Some(c) => match c {
                'q' => {
                    running = false;
                }
                'i' => {
                    input_mode(&stdin, &mut lines, &mut current_line);
                }
                _ => {
                    println!("?");
                }
            },
            None => {
                println!("?");
            }
        }
    }
}

fn input_mode(p_stdin: &Stdin, p_lines: &mut Vec<String>, p_current_line: &mut u32) {
    let mut is_insert_mode = true;
    while is_insert_mode {
        let mut input = String::new();
        if let Err(_) = p_stdin.read_line(&mut input) {
            println!("?");
            continue;
        }

        let input = input.trim();

        if input == "." {
            is_insert_mode = false;
            continue;
        }

        if let Ok(current_line) = TryInto::<usize>::try_into(*p_current_line) {
            p_lines.insert(current_line, input.to_string());
            *p_current_line += 1;
        } else {
            println!("?");
            continue;
        }
    }
}
