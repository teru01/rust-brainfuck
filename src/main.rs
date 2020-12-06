const BUFFER_SIZE: usize = 30000;
fn main() {
    println!("Hello, world!");
}

fn eval(input: &[char]) {
    let mut buffer = [0; BUFFER_SIZE];
    let mut buffer_ptr = 0;
    let mut input_ptr = 0;

    while input_ptr < input.len() {
        match input[input_ptr] {
            '>' => {
                buffer_ptr += 1;
            }
            '<' => {
                buffer_ptr -= 1;
            }
            '+' => {
                buffer[buffer_ptr] += 1;
            }
            '-' => {}
            '[' => {
                if buffer[buffer_ptr] == 0 {
                    while input_ptr < input.len() && input[input_ptr] == ']' {
                        input_ptr += 1;
                    }
                }
            }
            _ => {
                unimplemented!();
            }
        }
        input_ptr += 1;
    }
}
