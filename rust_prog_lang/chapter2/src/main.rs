use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Adivine el número!");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    let mut tries = 0;    
    loop {
        println!("Ingrese su adivinanza:");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Fallo al leer línea.");
        let guess:u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Ingrese un número.");
                continue;
            }
        };
            
        
        println!("Usted adivinó {}", guess);

        tries = tries + 1;

        match guess.cmp(&secret_number){
            Ordering::Less => println!("Demasiado pequeño!"),
            Ordering::Greater => println!("Demasiado grande!"),
            Ordering::Equal => {
                println!("Ha acertado!");
                println!("Le ha llevado {} intentos.", tries);
                break;
            }
        }
    }

}

