use std::io::Stdin;

struct Range {
    min: u32,
    max: u32,
}

fn main() {
    let mut lines = vec!["".to_string()];
    let mut current_line = 0usize;

    let stdin = std::io::stdin();

    let mut running = true;
    while running {
        let mut input = String::new();
        if let Err(_) = stdin.read_line(&mut input) {
            println!("?");
            continue;
        }

        let command = input.trim();
        match command.chars().nth(0) {
            Some(c) => match c {
                'q' => {
                    if command.len() == 1 {
                        running = false;
                    } else {
                        println!("?");
                    }
                }
                'i' => {
                    if command.len() == 1 {
                        input_mode(&stdin, &mut lines, &mut current_line);
                    } else {
                        println!("?");
                    }
                }
                'p' => {
                    println!("{}", lines[current_line]);
                }
                'n' => {
                    println!("{}\t{}", current_line, lines[current_line]);
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

fn input_mode(p_stdin: &Stdin, p_lines: &mut Vec<String>, p_current_line: &mut usize) {
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

        p_lines.insert(
            if *p_current_line == 0 {
                *p_current_line + 1
            } else {
                *p_current_line
            },
            input.to_string(),
        );

        *p_current_line += 1;
    }
}

fn parse_range(p_range_str: &str, p_min: u32, p_max: u32) -> Option<Range> {
    if p_range_str == "," {
        return Some(Range {
            min: p_min,
            max: p_max,
        });
    }

    let components = p_range_str.split(',').collect::<Vec<&str>>();
    if components.len() != 2 {
        return None;
    }

    let min = unsafe { *components.as_ptr().add(0) };
    let max = unsafe { *components.as_ptr().add(1) };

    let min = match min.parse::<u32>() {
        Err(_) => p_min,
        Ok(x) => x,
    };

    let max = match max.parse::<u32>() {
        Err(_) => p_max,
        Ok(x) => x,
    };

    Some(Range { min, max })
}
