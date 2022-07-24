use std::io;
use std::env;

// Minimum and maximum braille code points
const CAECUS_MIN:u16 = 0x2800;
const CAECUS_MAX:u16 = 0x28FF;

// Position of each incoming bit on the braille character
const CAECUS_POS_ENCODE:[u8;8] = [0,1,2,6,3,4,5,7];

// Position of each bit when decoding a braille character
const CAECUS_POS_DECODE:[u8;8] = [0,1,2,4,5,6,3,7];

// Encodes an 8-bit value to its corresponding braille character
fn caecus_char(octo:u8) -> char {

    // Rearrange necessary bits
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

// Decodes a braille character to its corresponding 8-bit value
fn caecus_int(c:char) -> u8 {
    let number = c as u16 - CAECUS_MIN;
    
    // Rearrange neccessary bits
    let mut rearranged = 0u8;
    for i in 0..8 {
        if number & (1 << i) == (1 << i) {
            rearranged |= 1 << CAECUS_POS_DECODE[i];
        }
    }

    return rearranged;
}

fn main() {
   
    // If "-d" argument is passed, use decode mode
    let args: Vec<String> = env::args().collect();
    let decode = args.len() > 1 && &args[1] == "-d";

    // Read data from stdin
    for line in io::stdin().lines() {
        match line {
            Ok(mut line) => { 
                if decode {
                    // Strip all whitespace from line
                    line.retain(|c| !c.is_whitespace());
                    // Process input as octal triples
                    for i in (0..line.len()).step_by(3) {
                        print!("{}", caecus_char(match u8::from_str_radix(&line[i..i+3], 8) {
                            Ok(octo) => octo,
                            Err(error) => panic!("Error converting octal chunk: {}", error)
                        }));
                    }
                    println!();
                    continue;
                }

                // Decode each braille character
                for c in line.chars() {
                    print!(" {:03o}", caecus_int(c));
                }
                println!(" ");
            },
            Err(error) => panic!("Error reading line: {}", error)
        }
    }
}
