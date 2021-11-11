use std::fs;
use std::io::Write;

fn brainfuck(code: String) -> String {
    let mut rust_code = String::from(
        "
use std::num::Wrapping;

const BUFFER_SIZE: usize = 3001;

fn main() {
    let mut b_index: usize = 0;
    let mut buffer = [0u8; BUFFER_SIZE];
    ",
    );

    let mut p = 0;
    let code_bytes = code.as_bytes();
    while p < code_bytes.len() {
        match code_bytes[p] as char {
            '+' => {
                let mut c = 0;
                while code_bytes[p] == '+' as u8 {
                    c += 1;
                    p += 1;
                }
                p -= 1;
                rust_code += &format!(
                    "buffer[b_index] = (Wrapping(buffer[b_index]) + Wrapping({}u8)).0;",
                    c
                )
                .to_string();
            }
            '-' => {
                let mut c = 0;
                while code_bytes[p] == '-' as u8 {
                    c += 1;
                    p += 1;
                }
                p -= 1;
                rust_code += &format!(
                    "buffer[b_index] = (Wrapping(buffer[b_index]) - Wrapping({}u8)).0;",
                    c
                )
                .to_string();
            }
            '>' => {
                let mut c = 0;
                while code_bytes[p] == '>' as u8 {
                    c += 1;
                    p += 1;
                }
                p -= 1;
                rust_code += &format!("b_index += {};", c).to_string();
            }
            '<' => {
                let mut c = 0;
                while code_bytes[p] == '<' as u8 {
                    c += 1;
                    p += 1;
                }
                p -= 1;
                rust_code += &format!("b_index -= {};", c).to_string();
            }
            '[' => {
                // Handle zeo case [-] for optimisation
                if String::from_utf8_lossy(&code_bytes[p..p + 3]) == "[-]" {
                    rust_code += "buffer[b_index] = 0;";
                    p += 2;
                } else {
                    rust_code += "while buffer[b_index] != 0 {";
                }
            }
            ']' => rust_code += "};",
            '.' => rust_code += "print!(\"{}\", buffer[b_index] as char);",
            ',' => rust_code += "print!(\"\nNot Implemented\n\");",
            _ => print!(""),
        }

        p += 1;
    }

    rust_code += "\n}";

    return rust_code;
}
fn main() {
    let mut code = fs::read_to_string("hw.bf").expect("Something went wrong reading the file");
    code = code.replace("\n", "").replace("\t", "").replace(" ", "");

    let rust_code = brainfuck(code);

    let mut file_ref = std::fs::File::create("out/brainfuck.rs").expect("create failed");

    file_ref
        .write_all(rust_code.as_bytes())
        .expect("write failed");

    println!("Text written into file successfully");
}
