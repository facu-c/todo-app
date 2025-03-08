use std::io;
use crate::modulos::tasks::create_task::create_task;


pub fn run_app() {

    loop {
                
        println!();
        println!("Ingrese el numero de tarea a realizar: ");
        println!("A. Ingresar nueva tarea.");
        println!("X. Salir de Qacer");

        let mut number_task = String::new();

        match io::stdin().read_line(&mut number_task) {
            Ok(_) => {},
            Err(_) => eprintln!("No se logro identificar la accion solicitada"),
        }

        match number_task.trim().to_uppercase().as_str() {
            
            "X" => break,
            "A" => create_task(),
            _ => {
                println!("No se logro identificar la accion solicitada, por favor, intente de nuevo");
                continue;
            },
        }
    }
    println!("Hasta luego!");
}