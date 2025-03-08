
pub mod create_task {
    use std::{fs::File, io::Write};  
    use crate::modulos::config::ConfigFile;

    fn create_file(config_file : ConfigFile) {

        println!("**********");
        

        println!("**********");

    }

    pub fn create_task(id_task: u64) {
        
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

        let config_file = ConfigFile::new_file(input_name, input_task, id_task);
        
       // id += 1;

        println!();
        
        let serialized_file = toml::to_string(&config_file).unwrap();

        create_file(config_file);
    }
}

// pub mod get_task {
    
// }