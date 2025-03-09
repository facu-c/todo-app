use std::fs::{self, File};
use std::io;

use crate::modulos::tasks::create_task::create_task;
use crate::modulos::tasks::get_task::get_task;

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

    match fs::create_dir(&document_dir) {
        Ok(_) => {
            let _ = File::create(format!("{}/save_file",&document_dir));
        },
        Err(msg) => { eprintln!("Hubo un problema con la creacion del directorio raiz: {} ", msg); }
    }

    let mut v_data = Vec::new();

    let last_id = match fs::read_to_string(format!("{}/save_file",document_dir)) {
        Ok(data) => {

            for element in data.lines() {
                v_data.push(element.to_string());
            }

            v_data.get(v_data.len()-3).unwrap()

        },
        Err(_) => { panic!("Not found") }
    };

    println!("{}",last_id);


    let mut id_task:u64 = last_id.parse().unwrap();



    loop {

        println!();
        println!("Ingrese el numero de tarea a realizar: ");
        println!("A. Ingresar nueva tarea.");
        println!("X. Salir de Qacer");

        let mut number_option = String::new();

        id_task += 1;

        match io::stdin().read_line(&mut number_option) {
            Ok(_) => {},
            Err(_) => eprintln!("No se logro identificar la accion solicitada"),
        }

        match number_option.trim().to_uppercase().as_str() {
            
            "X" => break,
            "A" => create_task(id_task),
            _ => {
                println!("No se logro identificar la accion solicitada, por favor, intente de nuevo");
                continue;
            },
        }
    }
    println!("Hasta luego!");

    get_task();
}