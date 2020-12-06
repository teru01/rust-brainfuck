use anyhow::Result;
use std::io::{self, prelude::*};
const BUFFER_SIZE: usize = 30000;

fn main() -> Result<()> {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input_chars: Vec<char> = input.chars().collect();
        exec(&input_chars)?;
        print!("\n");
    }
}

fn exec(input: &[char]) -> Result<()> {
    let mut buffer = [0; BUFFER_SIZE];
    let mut buffer_ptr = 0;
    let mut input_ptr = 0;
    let mut prev_paren = Vec::new();

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
            '-' => {
                buffer[buffer_ptr] -= 1;
            }
            '.' => {
                print!("{}", char::from(buffer[buffer_ptr]));
            }
            ',' => {
                buffer[buffer_ptr] = io::stdin().bytes().next().and_then(|r| r.ok()).unwrap();
            }
            '[' => match buffer[buffer_ptr] {
                0 => {
                    while input_ptr < input.len() && input[input_ptr] == ']' {
                        input_ptr += 1;
                    }
                }
                _ => prev_paren.push(input_ptr),
            },
            ']' => match buffer[buffer_ptr] {
                0 => {
                    prev_paren.pop().ok_or(anyhow::anyhow!("invalid ]"))?;
                }
                _ => {
                    if prev_paren.len() <= 0 {
                        anyhow::bail!("no previous paren")
                    }
                    input_ptr = prev_paren[prev_paren.len() - 1];
                }
            },
            _ => {}
        }
        input_ptr += 1;
    }
    Ok(())
}
