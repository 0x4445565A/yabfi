use std::io;
use std::io::Write;

const MEMORY_BUFFER_SIZE: usize = 255;

fn main() {
    let program = ">>+++++[-<+++++>]<+<,>[-<.->]";
    run_bf_program(program);
}

fn run_bf_program(program: &str) {
    let program: &[u8] = program.as_bytes();

    let mut arr: [u8; MEMORY_BUFFER_SIZE] = [0; MEMORY_BUFFER_SIZE];
    let mut pointer: usize = 0;
    let mut cursor: usize = 0;
    let mut stack: Vec<usize> = vec![];

    while cursor < program.len() {
        match program[cursor] as char {
            '#' => println!(
                "Memory: {arr:?}\nPointer: {pointer}\nValue: {}",
                arr[pointer]
            ),
            '-' => arr[pointer] = arr[pointer].wrapping_sub(1),
            '+' => arr[pointer] = arr[pointer].wrapping_add(1),
            '>' => pointer = (pointer + 1) % MEMORY_BUFFER_SIZE,
            '<' => {
                pointer = if pointer == 0 {
                    MEMORY_BUFFER_SIZE - 1
                } else {
                    (pointer - 1) % MEMORY_BUFFER_SIZE
                }
            }
            '.' => print!("{}", arr[pointer] as char),
            '[' => stack.push(cursor),
            ']' => {
                let start = stack.pop().unwrap();
                // Set the cursor back to the start of the loop
                if arr[pointer] != 0 {
                    cursor = start;
                    continue;
                }
            }
            ',' => {
                print!("Input number 0-255: ");
                std::io::stdout()
                    .flush()
                    .expect("Failed to flush STDOUT buffer");
                let mut i = String::new();
                io::stdin().read_line(&mut i).unwrap_or(0);
                let i: u8 = i.trim().parse().unwrap_or(0);
                arr[pointer] = i;
            }
            _ => (),
        };

        // Advance if the loop didn't continue early
        cursor += 1;
    }
}
