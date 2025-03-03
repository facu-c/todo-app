

mod create_task {
    use std::{fs::File, io::Write};  
    use crate::modulos::config::ConfigFile; 

    pub fn create_file(config_file : ConfigFile) {

        println!("**********");
        
        let mut f = File::create(format!("//home/facundo/Documentos/{}.txt",config_file.get_name())).expect("No se pudo crear el archivo");
        match f.write_all(&config_file.get_task().as_bytes()) {
            Ok(_) => {
                println!("Archivo creado con exito");
            },
            Err(msg) => {
                eprintln!("No se pudo crear el archivo: {}", msg);
            }
        }

        println!("**********");

    }

}
pub fn get_task() {

    use std::io;
    use crate::modulos::config::ConfigFile;     
    
    let mut input_task = String::new();
    let mut input_name = String::new();
    
    println!("Ingrese el nombre con el que guardar el archivo: ");
    let _ = io::stdin().read_line(&mut input_name);
    println!();
    println!("Ingrese la tarea a realizar: ");
    let _ = io::stdin().read_line(&mut input_task);

    let config_file = ConfigFile::new_file(input_name, input_task);
    println!();

    create_task::create_file(config_file);
}