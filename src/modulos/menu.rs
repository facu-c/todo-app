
use std::io;
use crate::modulos::tasks::create_task;

pub fn start_menu() {
    loop {
        println!();
        println!("Ingrese el numero de tarea a realizar: ");
        println!("A. Ingresar nueva tarea.");
        println!("X. Salir de Qacer");

        let mut number_option = String::new();
        //  id_task += 1;
        match io::stdin().read_line(&mut number_option) {
            Ok(_) => {},
            Err(_) => eprintln!("No se logro identificar la accion solicitada"),
        }

        match number_option.trim().to_uppercase().as_str() {
            
            "X" => break,
            "A" => create_task(0),
            _ => {
                println!("No se logro identificar la accion solicitada, por favor, intente de nuevo");
                continue;
            },
            //TODO: Agregar "recuperar tareas"
        }
    }
    println!("¡Hasta luego!");
}
