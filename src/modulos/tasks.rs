
pub mod create_task {

    use std::{fs::{self, File}, io::Read};

    use crate::modulos::config::ConfigFile;

    fn create_file(config_file : ConfigFile) {

        let path = format!("{}/qacer/save_file", dirs::document_dir().unwrap().display().to_string());

        let mut data = String::new();

        let _ = match File::open(&path) {
            Ok(mut file) => {
                file.read_to_string(&mut data)
            },
            Err(_) => {
                return;
            }
        };

        let contents = toml::to_string(&config_file).unwrap();

        data.push_str(&contents);

        let _ = fs::write(&path, &data);

        println!("{}", data);

    }

    pub fn create_task(id_task: u64) {
        
        use std::io;
        use crate::modulos::config::ConfigFile;
        
        let mut input_desc_task = String::new();
        let mut input_name_task = String::new();
        
        println!("Ingrese de la tarea: ");
        let _ = io::stdin().read_line(&mut input_name_task);
        println!();
        println!("Agrege una descripcion de la tarea: ");
        let _ = io::stdin().read_line(&mut input_desc_task);
    

        let config_file = ConfigFile::new_file(input_name_task, input_desc_task, id_task);
        
        println!();
        
        create_file(config_file);
    }
}


pub mod get_task {
    
}