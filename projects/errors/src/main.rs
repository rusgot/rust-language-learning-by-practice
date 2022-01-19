use std::fs::File;
use std::io::ErrorKind;

fn main() {
    // panic!("crash and burn");

    let v = vec![1, 2, 3];
    // v[99];

    // Recoverable errors
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => {
                    println!("File created: hello.txt");
                    fc
                }
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };

    let result = get_item(false);
    let val = match result {
        Ok(msg) => {
            println!("Message: {}", msg);
            msg
        }
        Err(e) => {
            println!("Error: {:?}", e);
            panic!("Bad price, I need the price to go up")
        }
    };
    // println!("{}", val.unwrap());
}

fn get_item(v: bool) -> Result<String, ErrorKind> {
    if v {
        return Ok(String::from("Metaphysical Mechanism"));
    } else {
        return Err(ErrorKind::Unsupported);
    }
}
