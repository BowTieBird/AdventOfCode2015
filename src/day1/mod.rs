use std::fs;

pub fn main(file_path: &str) {
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    // let contents = contents.chars();

    let mut floor : isize = 0;
    let mut count : isize = 0;
    let mut found : bool = false;
    for c in contents.chars() {
        if c == '(' {
            floor+=1;
        } else if c == ')' {
            floor-=1;
        }
        if !found {
            count += 1;
            if floor == -1 {
                println!("{}", count);
                found = true;
            }
        }
    }
    println!("{}", floor);
}
