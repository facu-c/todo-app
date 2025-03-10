use std::fs::{self, File};
use std::io;

use crate::modulos::tasks::create_task::create_task;

extern crate dirs;

pub fn run_app() {

    let document_dir = match dirs::document_dir() {
        Some(dir) => {
            format!("{}/qacer",dir.display().to_string())
        },
        None => {
            String::from("no se pudo encotrar el path correcto")
        }
    };

    println!("{:?}", &document_dir);

    match fs::create_dir(&document_dir) {
        Ok(_) => {
            let _ = File::create(format!("{}/save_file",&document_dir));
        },
        Err(msg) => { eprintln!("Hubo un problema con la creacion del directorio raiz: {} ", msg); }
    }

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
        }
    }
    println!("Hasta luego!");
}