//Calculadora
use std::io;
fn calc(){
    println!("Calculadora :3");
    loop {
        println!("1.Sumar");
        println!("2.Restar");
        println!("3.Multiplicar");
        println!("4.Dividir");
        println!("5.Salir");
        println!("Selecciona una opcion: ");
        let mut opcion = String::new();
        io::stdin()
            .read_line(&mut opcion)
            .expect("no");
        // Convierte String a Int
        let option: i32 = opcion.trim().parse().expect("Please enter a valid number");
        //Break Loop
        if option == 5{
            println!("Adios :3");
            break;
        }
        println!("Introduce un numero: ");
        let mut num1 = String::new();
        io::stdin()
            .read_line(&mut num1)
            .expect("no");
        // Convierte String a Int
        let num1: i32 = num1.trim().parse().expect("Please enter a valid number");
        println!("Introduce otro numero: ");
        let mut num2 = String::new();
        io::stdin()
            .read_line(&mut num2)
            .expect("no");
        // Convierte String a Int
        let num2: i32 = num2.trim().parse().expect("Please enter a valid number");
        //Inicio If
        if option == 1 {
            println!("El resultado es: {}", num1 + num2);
        } else if option == 2 {
            println!("El resultado es: {}", num1 - num2);
        } else if option == 3 {
            println!("El resultado es: {}", num1 * num2);
        } else if option == 4 {
            println!("El resultado es: {}", num1 / num2);
        } // Fin If
    } // Fin Loop
} // Fin fn

fn main() {
    calc();
}
