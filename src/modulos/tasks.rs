use std::*;

use crate::modulos::config::ConfigFile;
use std::io;

pub fn create_task(tasks: &mut Vec<ConfigFile>, id_task: u64) {
    
    let path = format!("{}/qacer/save_file",dirs::document_dir().unwrap().display());
    let mut input_desc_task = String::new();
    let mut input_name_task = String::new();
    
    println!("Ingrese de la tarea: ");
    let _ = io::stdin().read_line(&mut input_name_task);
    println!();
    println!("Agrege una descripcion de la tarea: ");
    let _ = io::stdin().read_line(&mut input_desc_task);

    tasks.push(ConfigFile { id_task, name_task: input_name_task.trim().to_string(), desc_task: input_desc_task.trim().to_string(), done: false} );

    let json = serde_json::to_string_pretty(tasks).unwrap();
    fs::write(path, json).unwrap();

    println!();
}

pub fn load_task() -> Vec<ConfigFile> {
    let path = format!("{}/qacer/save_file",dirs::document_dir().unwrap().display());
    let data = fs::read_to_string(path).unwrap_or_else(|_| "[]".to_string());
    serde_json::from_str(&data).unwrap_or_default()
    
}

pub fn list_task() {
    let path = format!("{}/qacer/save_file",dirs::document_dir().unwrap().display());
    let data = fs::read_to_string(path).unwrap_or_else(|_| "[]".to_string());
    let tasks: Vec<ConfigFile> = serde_json::from_str(&data).unwrap_or_default();

    for element in tasks {
        println!("{}: {} - {} - {}", element.id_task, element.name_task, element.desc_task, element.done);
        println!("---------------");
    }
}

pub fn remove_task( tasks: &[ConfigFile]) {
    let path = format!("{}/qacer/save_file",dirs::document_dir().unwrap().display());
    let mut new_data: Vec<ConfigFile> = vec![];
    let mut id_delete = String::new();

    println!("Ingresa el id a eliminar");
    io::stdin().read_line(&mut id_delete).expect("Error");

    let id_delete: u64 = id_delete.trim().parse().expect("error");

    for element in tasks {
        if element.id_task != id_delete {
            new_data.push(element.clone());
        }
    }

    let json = serde_json::to_string_pretty(&new_data).unwrap();

    fs::write(path, json).unwrap();
}

pub fn make_done(tasks: &[ConfigFile]) {
    let path = format!("{}/qacer/save_file",dirs::document_dir().unwrap().display());
    let mut new_data: Vec<ConfigFile> = vec![];
    let mut id_done = String::new();

    println!("Ingresa el id de la tarea a terminar");
    io::stdin().read_line(&mut id_done).expect("Error");

    let id_delete: u64 = id_done.trim().parse().expect("error");

    for mut element in tasks.iter().cloned() {
        if element.id_task == id_delete {
            element.done = true;
        }
        new_data.push(element);
    }

    let json = serde_json::to_string_pretty(&new_data).unwrap();

    fs::write(path, json).unwrap();
}