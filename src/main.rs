use std::io::Stdin;

fn main() {
    let mut lines = Vec::<String>::new();
    let mut current_line = -1i64;

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
                'p' => {
                    println!(
                        "{}",
                        lines
                            .get(TryInto::<usize>::try_into(current_line).unwrap_or(0usize))
                            .unwrap_or(&("?".to_string()))
                    );
                }
                'n' => match TryInto::<usize>::try_into(current_line) {
                    Ok(line_number) => match lines.get(line_number) {
                        Some(line) => {
                            println!("{}\t{}", line_number, line);
                        }
                        None => {
                            println!("?");
                        }
                    },
                    Err(_) => {
                        println!("?");
                    }
                },
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

fn input_mode(p_stdin: &Stdin, p_lines: &mut Vec<String>, p_current_line: &mut i64) {
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

        *p_current_line += 1;
        if let Ok(current_line) = TryInto::<usize>::try_into(*p_current_line) {
            p_lines.insert(current_line, input.to_string());
        } else {
            println!("?");
            continue;
        }
    }
}
