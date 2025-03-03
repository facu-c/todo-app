use create_task::get_task;


mod create_task {
    use std::{fs::{self, File}, io::{self, Write}};

    fn create_file(input : &String) {
        
        let mut f = File::create("//home/facundo/Documentos/foo.txt").expect("No se pudo crear el archivo");

        let _ = f.write_all(&input.as_bytes());


    }

    pub fn get_task() {
        let mut input = String::new();

        println!("Ingrese la tarea a realizar: ");

        let _ = io::stdin().read_line(&mut input);

        create_file(&input);
    }
}


pub fn run_app() {

    println!("desde run_app");
    
    get_task();
}