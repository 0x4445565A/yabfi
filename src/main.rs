const MEMORY_BUFFER_SIZE: usize = 255;

fn main() {
    let program = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";
    println!("Program: {program}");
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
                if arr[pointer] != 0 {
                    cursor = start;
                    continue;
                }
            }
            _ => (),
        };

        cursor += 1;
    }
}
