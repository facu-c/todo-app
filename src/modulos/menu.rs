
use std::io;
use crate::modulos::tasks::{create_task, list_task, load_task, make_done, remove_task};

pub fn start_menu() {

    let mut tasks = load_task();
    let mut id_number = 0;

    loop {
        println!();
        println!("Ingrese el numero de tarea a realizar: ");
        println!("A. Ingresar nueva tarea.");
        println!("B. Listar tareas");
        println!("D. Borrar tarea");
        println!("Y. Marcar Hecha");
        println!("X. Salir de Qacer");

        let mut number_option = String::new();        
        
        match io::stdin().read_line(&mut number_option) {
            Ok(_) => {},
            Err(_) => eprintln!("No se logro identificar la accion solicitada"),
        }
        match number_option.trim().to_uppercase().as_str() {
            
            "X" => break,
            "A" => {
                    create_task(&mut tasks, id_number);
                    id_number += 1;
                }
            "B" => list_task(),
            "D" => remove_task(&tasks),
            "Y" => make_done(&tasks),
            _ => {
                println!("No se logro identificar la accion solicitada, por favor, intente de nuevo");
                continue;
            },
            //TODO: Agregar "recuperar tareas"
        }
    }
    println!("¡Hasta luego!");
}
