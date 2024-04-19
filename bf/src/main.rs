use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::stdin;
use std::io::stdout;
use std::io::Read;
use std::io::Write;

fn easy_input(prompt: &String) -> String {
    stdout().flush().expect("Failed to flush stdout");

    print!("{}", prompt);
    let mut s = String::new();
    stdin()
        .read_line(&mut s)
        .expect("Couldn't read line")
        .to_string();
    return s;
}

fn read_file(p: &str) -> String {
    let mut file = fs::File::open(p).expect("Couldn't open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Can't read file");
    contents
}

fn main() {
    let file_name: String;
    if env::args().len() <= 1 {
        file_name = format!("{}{}", "", easy_input(&String::from("File name:\n")));
    } else {
        file_name = env::args().nth(1).unwrap();
    }
    let program = read_file(&file_name.trim());

    // Parsing [] using a stack
    let mut loop_table = HashMap::<usize, usize>::new();
    let mut stack: Vec<usize> = vec![];
    for ip in 0..program.len() {
        let instruction = program.chars().nth(ip).unwrap();
        if instruction == '[' {
            stack.push(ip);
        } else if instruction == ']' {
            let loop_begin_index = stack
                .pop()
                .expect(format!("Bracket mismatch at {}", ip).as_str());
            loop_table.insert(loop_begin_index.try_into().unwrap(), ip.try_into().unwrap());
            loop_table.insert(ip.try_into().unwrap(), loop_begin_index.try_into().unwrap());
        }
    }
    assert!(stack.len() == 0);

    let mut tape: Vec<i16> = vec![0];
    let mut cell_index = 0;
    let mut ip = 0;

    while ip < program.len() {
        let instruction = program.chars().nth(ip).unwrap();

        if instruction == '+' {
            tape[cell_index] += 1;
            if tape[cell_index] >= 256 {
                tape[cell_index] = 0;
            }
        } else if instruction == '-' {
            tape[cell_index] -= 1;
            if tape[cell_index] < 0 {
                tape[cell_index] = 255;
            }
        } else if instruction == '<' {
            if i32::try_from(cell_index).unwrap() - 1 < 0 {
                cell_index = 0
            } else {
                cell_index -= 1;
            }
        } else if instruction == '>' {
            cell_index += 1;
            while cell_index >= tape.len() {
                tape.push(0);
            }
        } else if instruction == '.' {
            let a: u8 = tape[cell_index].try_into().unwrap();
            let c: char = a.try_into().unwrap();
            print!("{}", c)
        } else if instruction == ',' {
            let inp = easy_input(&String::from(""));
            let c = inp.chars().nth(0).unwrap();
            tape[cell_index] = u8::try_from(c).unwrap().try_into().unwrap();
        } else if instruction == '[' {
            if tape[cell_index] == 0 {
                ip = loop_table[&ip];
            }
        } else if instruction == ']' {
            if tape[cell_index] != 0 {
                ip = loop_table[&ip];
            }
        }

        ip += 1;
    }
}
