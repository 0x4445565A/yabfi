use std::io::{self, BufRead, Write};
use std::process;

const MEMORY_BUFFER_SIZE: usize = 255;

fn main() -> io::Result<()> {
    let mut arr: [u8; MEMORY_BUFFER_SIZE] = [0; MEMORY_BUFFER_SIZE];
    let mut pointer: usize = 0;

    if atty::is(atty::Stream::Stdin) {
        println!(
            r###"
No STDIN detected, using interpreter CLI...
    __   _____  ____________ _____ 
    \ \ / / _ \ | ___ \  ___|_   _|
     \ V / /_\ \| |_/ / |_    | |  
      \ /|  _  || ___ \  _|   | |  
      | || | | || |_/ / |    _| |_ 
      \_/\_| |_/\____/\_|    \___/ 
    Yet Another Brain F*ck Interpreter
                         (q)uit (h)elp
        "###
        );
        loop {
            print!(">>> ");
            std::io::stdout()
                .flush()
                .expect("Failed to flush STDOUT buffer");
            let mut program = String::new();
            io::stdin().read_line(&mut program).unwrap_or(0);
            run_bf_program(program.as_str(), &mut arr, &mut pointer, true);
        }
    }

    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    let buf = stdin.fill_buf()?;
    let program = match std::str::from_utf8(buf) {
        Ok(p) => p,
        Err(error) => panic!("You goofed! {error}"),
    };

    run_bf_program(program, &mut arr, &mut pointer, false);
    Ok(())
}

fn run_bf_program(
    program: &str,
    arr: &mut [u8; MEMORY_BUFFER_SIZE],
    pointer: &mut usize,
    interpret_mode: bool,
) {
    let program: &[u8] = program.as_bytes();
    let mut cursor: usize = 0;
    let mut stack: Vec<usize> = vec![];
    let mut p = *pointer;

    while cursor < program.len() {
        match program[cursor] as char {
            '.' => print!("{}", arr[p] as char),
            '-' => arr[p] = arr[p].wrapping_sub(1),
            '+' => arr[p] = arr[p].wrapping_add(1),
            '>' => p = (p + 1) % MEMORY_BUFFER_SIZE,
            '<' => {
                p = if p == 0 {
                    MEMORY_BUFFER_SIZE - 1
                } else {
                    (p - 1) % MEMORY_BUFFER_SIZE
                }
            }
            '[' => stack.push(cursor),
            ']' => {
                let start = stack.pop().unwrap();
                // Set the cursor back to the start of the loop
                if arr[p] != 0 {
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
                arr[p] = i;
            }
            '#' => {
                if interpret_mode {
                    println!("Memory: {arr:?}\nPointer: {p}\nValue: {}", arr[p])
                }
            }
            'q' => {
                if interpret_mode {
                    process::exit(0)
                }
            }
            'h' => {
                if interpret_mode {
                    print_help()
                }
            }
            _ => (),
        };

        // Advance if the loop didn't continue early
        *pointer = p;
        cursor += 1;
    }
}

fn print_help() {
    println!(
        r###"Wiki: https://esolangs.org/wiki/Brainfuck

Non-standard BF symbols (Only in interpret mode)
---------
h: Print this help
q: Quit the program
#: Print the entire memory buffer

Standard BF symbols
---------
>: Moves pointer to the right
<: Moves pointer to the left
+: Increment the memory cell at the pointer
-: Decrement the memory cell at the pointer
.: Print the character signified by the cell at the pointer
,: Input a number and store it in the cell at the pointer
[: Jump past the matching ] if the cell at the pointer is 0
]: Jump back to the matching [ if the cell at the pointer is nonzero
        "###
    );
}
