
mod create_task {
    use std::{fs::File, io::{self, Write}};

    fn create_file(input_task : &String, input_name: &String) {

        println!("**********");
        
        let mut f = File::create(format!("//home/facundo/Documentos/{}.txt",input_name)).expect("No se pudo crear el archivo");
        match f.write_all(&input_task.as_bytes()) {
            Ok(_) => {
                println!("Archivo creado con exito");
            },
            Err(msg) => {
                eprintln!("No se pudo crear el archivo: {}", msg);
            }
        }

        println!("**********");

    }

    pub fn get_task() {
        
        let mut input_task = String::new();
        let mut input_name = String::new();
        
        print!("Ingrese el nombre con el que guardar el archivo: ");
        let _ = io::stdin().read_line(&mut input_name);
        println!();
        print!("Ingrese la tarea a realizar: ");
        let _ = io::stdin().read_line(&mut input_task);
        
        println!();

        create_file(&input_task, &input_name);
    }
}

pub fn run_app() {
    
    use create_task::get_task;

    println!();
    
    get_task();
}