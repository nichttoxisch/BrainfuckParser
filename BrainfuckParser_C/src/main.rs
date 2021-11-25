use std::fs;
use std::io::Write;

fn brainfuck(code: String) -> String {
    let mut c_code = String::from(
        "
#include <stdio.h>
#include <stdlib.h>

int main(/*int argc, char const *argv[]*/)
{
    int b_index = 0;
    unsigned char buffer[3001] = {0};
    char c = 0;

",
    );

    let mut p = 0;
    let code_bytes = code.as_bytes();
    while p < code_bytes.len() {
        match code_bytes[p] as char {
            '+' => {
                let mut c = 0;
                while p < code_bytes.len() && code_bytes[p] == '+' as u8 {
                    c += 1;
                    p += 1;
                }
                p -= 1;
                c_code += &format!("buffer[b_index] += {};", c).to_string();
            }
            '-' => {
                let mut c = 0;
                while p < code_bytes.len() && code_bytes[p] == '-' as u8 {
                    c += 1;
                    p += 1;
                }
                p -= 1;
                c_code += &format!("buffer[b_index] -={};", c).to_string();
            }
            '>' => {
                let mut c = 0;
                while p < code_bytes.len() && code_bytes[p] == '>' as u8 {
                    c += 1;
                    p += 1;
                }
                p -= 1;
                c_code += &format!("b_index += {};", c).to_string();
            }
            '<' => {
                let mut c = 0;
                while p < code_bytes.len() && code_bytes[p] == '<' as u8 {
                    c += 1;
                    p += 1;
                }
                p -= 1;
                c_code += &format!("b_index -= {};", c).to_string();
            }
            '[' => {
                // Handle zeo case [-] for optimisation
                if p < code_bytes.len() - 2
                    && String::from_utf8_lossy(&code_bytes[p..p + 3]) == "[-]"
                {
                    c_code += "buffer[b_index] = 0;";
                    p += 2;
                } else {
                    c_code += "while (buffer[b_index] != 0) {";
                }
            }
            ']' => c_code += "};",
            '.' => c_code += "printf(\"%c\", buffer[b_index]);",
            ',' => {
                c_code += "c = getc(stdin);if (c != 10) {buffer[b_index] = c;}"
                // c_code += "buffer[b_index] = getchar()"
            }
            _ => print!(""),
        }

        p += 1;
    }

    c_code += "

    return 0;
}
";

    return c_code;
}
fn main() {
    let mut code = fs::read_to_string("hw.bf").expect("Something went wrong reading the file");
    code = code.replace("\n", "").replace("\t", "").replace(" ", "");

    let rust_code = brainfuck(code);

    let mut file_ref = std::fs::File::create("out/brainfuck.c").expect("create failed");

    file_ref
        .write_all(rust_code.as_bytes())
        .expect("write failed");

    println!("Text written into file successfully");
}
