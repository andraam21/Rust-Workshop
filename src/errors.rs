// TODO 1 - fix this function
pub fn divide(a: i32, b: i32) -> Option<isize> {
    if b == 0 {
        None
    } else {
        Some((a / b).try_into().unwrap())
    }
}

// TODO 3 - fix this function
pub fn divide_error(a: i32, b: i32) -> Result<isize, String> {
    match b {
        0 => Err("Not permitted".to_string()),
        _ => Ok((a /b) as isize)
    }
}

// TODO 5 - list files in the diretcory
// - if the directory does not exist, return None

use std::fs; 

fn list_dir(path: &str) -> Option<Vec<String>> {

    let paths = fs::read_dir(path);

    let res1 = match paths {
        Ok(x) => x,
        Err(_) => return None,
    };

    let mut res2 = Vec::new();

    for p in res1 {
        res2.push(p.unwrap().path().display().to_string())
    }

    Some(res2)
}

pub fn run() {
    // TODO 2 - make the print work, use match and/or if let
    // println!("division: ", divide(a, b));

    // TODO 4 - make the print work, use match and/or if let
    // println!("division: ", divide_error(a, b));

    // TODO 6 - use the list_directory function to print the current directory
}
