use std::io::Stdin;

#[derive(PartialEq, Eq, Debug)]
struct Range {
    min: u32,
    max: u32,
}

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

#[test]
fn range_parser_test() {
    let want = Range { min: 4, max: 5 };
    let got = parse_range("4,5", 0, 100).unwrap();
    assert_eq!(want, got);

    let want = Range { min: 5, max: 16 };
    let got = parse_range(",16", 5, 100).unwrap();
    assert_eq!(want, got);

    let want = Range { min: 14, max: 33 };
    let got = parse_range("14,", 0, 33).unwrap();
    assert_eq!(want, got);

    let want = Range { min: 9, max: 81 };
    let got = parse_range(",", 9, 81).unwrap();
    assert_eq!(want, got);
}
