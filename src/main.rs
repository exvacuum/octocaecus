use std::io;
use std::env;

const CAECUS_MIN:u16 = 0x2800;
const CAECUS_MAX:u16 = 0x28FF;
const CAECUS_POS_ENCODE:[u8;8] = [0,1,2,6,3,4,5,7];
const CAECUS_POS_DECODE:[u8;8] = [0,1,2,4,5,6,3,7];

fn caecus_char(octo:u8) -> char {
    let mut rearranged = 0u8;
    for i in 0..8 {
        if octo & (1 << i) == (1 << i) {
            rearranged |= 1 << CAECUS_POS_ENCODE[i];
        }
    }

    return match char::from_u32((CAECUS_MIN + rearranged as u16) as u32) {
        Some(character) => { 
            if character as u16 > CAECUS_MAX {
                panic!("Character too large!");
            }

            character
        },
        None => panic!("Invalid character!")
    }
}

fn caecus_int(c:char) -> u8 {
    let number = c as u16 - CAECUS_MIN;
    
    let mut rearranged = 0u8;
    for i in 0..8 {
        if number & (1 << i) == (1 << i) {
            rearranged |= 1 << CAECUS_POS_DECODE[i];
        }
    }

    return rearranged;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let decode = args.len() > 1 && &args[1] == "-d"; 
    for line in io::stdin().lines() {
        match line {
            Ok(mut line) => { 
                if decode {
                    line.retain(|c| !c.is_whitespace());
                    for i in (0..line.len()).step_by(3) {
                        print!("{}", caecus_char(match u8::from_str_radix(&line[i..i+3], 8) {
                            Ok(octo) => octo,
                            Err(error) => panic!("Error converting octal chunk: {}", error)
                        }));
                    }
                    println!();
                    continue;
                }

                for c in line.chars() {
                    print!(" {:03o}", caecus_int(c));
                }
                println!(" ");
            },
            Err(error) => panic!("Error reading line: {}", error)
        }
    }
}
