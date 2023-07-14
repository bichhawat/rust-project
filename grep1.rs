use std::collections::HashMap;
use std::io::Read;
use std::io;

fn bad_char_heuristic(pat: &[u8]) -> HashMap<u8, usize> {
    let mut badchar: HashMap<u8, usize> = HashMap::new();
    for (i, &c) in pat.iter().enumerate() {
        badchar.insert(c, i);
    }
    badchar
}

pub fn search(pat: &[u8], txt: &[u8]) -> Option<usize> {
    let badchar = bad_char_heuristic(pat);
    let m = pat.len() as isize;
    let n = txt.len();
    let txt_bytes = txt.as_ptr();

    let mut s: isize = 0;

    while s <= (n as isize - m) {
        let mut j = m - 1;
        while j >= 0 && pat[j as usize] == unsafe { *txt_bytes.offset((s + j) as isize) } {
            j -= 1;
        }
        if j < 0 {
            return Some(s as usize);
        }
        s += if let Some(&idx) = badchar.get(&unsafe { *txt_bytes.offset((s + j) as isize) }) {
            (j - idx as isize).max(1)
        } else {
            j + 1
        };
    }
    None
}

fn main() {
    // pat is the pattern to be searched in input file
    println!("Enter pattern to be searched:");
    let mut pat = String::new();
    io::stdin().read_line(&mut pat).expect("failed to readline");
    let pat = pat.trim(); // trim the newline character from the input string

    println!("Enter file name :");
    let mut file_name = String::new();
    io::stdin().read_line(&mut file_name).expect("failed to readline");
    let file_name = file_name.trim(); // trim the newline character from the input string

    /* Here, the format!() macro is used to create a new String 
    that combines the file_name variable with the ".txt" extension. 
    The resulting string is then passed to the open() method. */

   let mut file = std::fs::File::open(format!("{}.txt", file_name)).unwrap();
   let mut contents = String::new();
   file.read_to_string(&mut contents).unwrap();
   // Content is now a string of text written inside the file


    let t = contents.as_bytes();
    let p = pat.as_bytes();
    if let Some(idx) = search(p, t) {
        println!("Pattern found at index {}", idx);
    } else {
        println!("Pattern not found");
    }
}


