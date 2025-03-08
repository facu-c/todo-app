

pub mod create_task {
    use std::{fs::File, io::Write};  
    use crate::modulos::config::ConfigFile; 

    fn create_file(config_file : ConfigFile) {

        println!("**********");

        let save_file = format!("{} - {} - /",config_file.get_done(), config_file.get_task().trim());
        
        let mut f = File::create(format!("//home/facundo/Documentos/{}.txt",config_file.get_name())).expect("No se pudo crear el archivo");
        match f.write_all(&save_file.as_bytes()) {
            Ok(_) => {
                println!("Archivo creado con exito");
            },
            Err(msg) => {
                eprintln!("No se pudo crear el archivo: {}", msg);
            }
        }

        println!("**********");

    }

    pub fn create_task() {
        
        use std::io;
        use crate::modulos::config::ConfigFile;
        
        let mut input_task = String::new();
        let mut input_name = String::new();
        
        println!("Ingrese el nombre con el que guardar el archivo: ");
        let _ = io::stdin().read_line(&mut input_name);
        println!();
        println!("Ingrese la tarea a realizar: ");
        let _ = io::stdin().read_line(&mut input_task);
    
        //let mut id = 0;

        let config_file = ConfigFile::new_file(input_name, input_task);
        
       // id += 1;

        println!();
    
        create_file(config_file);
    }
}

pub mod get_task {
    
}