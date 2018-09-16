
use std::io;
use std::io::Read;
use std::fs::File;
use std::io::ErrorKind;


fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt").unwrap();// si todo sale bien retorna lo que contenga Ok..si no llama a panic!
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file2() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;// the ? operator permite capturar el error como si de un match se tratase

    Ok(s)
}

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => {
                    panic!(
                        "Tried to create file but there was a problem: {:?}",
                        e
                    )
                },
            }
        },
        Err(error) => {
            panic!(
                "There was a problem opening the file: {:?}",
                error
            )
        },
    };
}
