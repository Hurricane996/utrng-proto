use std::io::{stdout, stdin, Write};

pub fn get_int(question: &str) -> u32 {
    loop {
        print!("{}", question);
        let mut s = String::new();
        let _ = stdout().flush();
        stdin().read_line(&mut s).expect("Could not read from stdin");

        match s.trim().parse::<u32>() {
            Ok(res) => {
                return res;
            },
            Err(_) => {
                print!("Not a valid int!");
                continue;
            },
        }
    }
}

pub fn get_yn(question: &str) -> bool {
    print!("{}", question);

    let mut s = String::new();
    let _ = stdout().flush();
    stdin().read_line(&mut s).expect("Could not read from stdin");

    s.trim().to_lowercase() == "y"
}
