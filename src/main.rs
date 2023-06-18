const MEMORY_BUFFER_SIZE: usize = 255;
fn main() {
    let program = "++>++->";
    let mut arr: [u8; MEMORY_BUFFER_SIZE] = [0; MEMORY_BUFFER_SIZE];
    let mut pointer = 0;
    for c in program.chars() {
        match c {
            '-' => arr[pointer] -= 1,
            '+' => arr[pointer] += 1,
            '>' => pointer += 1,
            _ => println!("Skip"),
        };
    }
    println!("{program}\n{arr:?}");
}
