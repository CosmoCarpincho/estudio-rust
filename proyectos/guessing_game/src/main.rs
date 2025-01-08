use std::{cmp::Ordering, io};
use rand::Rng;


fn main() {
    println!("Adivine el número:");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("El número secreto es: {secret_number}");

    println!("Ingrese su suposición:");

    let mut guess = String::new();


    io::stdin()
        .read_line(&mut guess)
        .expect("Error al leer la línea");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    println!("Su suposición es: {guess}");

    match guess.cmp(&secret_number){
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
