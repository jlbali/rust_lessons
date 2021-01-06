use std::fs::File;
use std::io::ErrorKind;
use std::io;
use std::io::Read;
use std::fs;

fn main() {
    /*
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("Problem opening the file {:?}", error)
        },
    };*/

    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file {:?}", e),
            },
            other_error => panic!("Problem opening the file {:?}", other_error),
        },
    };

    // Quicker way... unwrap returns the Ok part if Result is ok
    // otherwise it panics.
    let f = File::open("hello.txt").unwrap();

    // Similar to unwrap, but let us choose a message for the panic.
    let f = File::open("hello.txt").expect("Could not open hello.txt");

    // Propagating errors.    

}


fn read_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// ? returns the Ok object if Ok otherwise it finishes the
// function returning an Error propagated.

fn shorter_read_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn even_shorter_read_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s);
}

fn convenient_read_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

