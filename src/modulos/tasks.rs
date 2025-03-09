
pub mod create_task {

    use std::*;

    use crate::modulos::config::ConfigFile;

    fn create_file(config_file : ConfigFile) {
        
        let path = format!("{}/qacer/save_file",dirs::document_dir().unwrap().display().to_string());

        let mut data = fs::read_to_string(&path).unwrap();

        let contents = format!("{}\n{}{}", config_file.get_id(), config_file.get_name(), config_file.get_task());

        data.push_str(&contents.as_str());

        let _ = fs::write(&path, &data);

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
    use std::fs;


    fn get_tasks(id_task: u64) -> Vec<String>  {
        
        let path = format!("{}/qacer/save_file",dirs::document_dir().unwrap().display().to_string());

        let data = fs::read_to_string(path).unwrap();

        let mut content: Vec<String> = Vec::new();

        for element in data.lines() {

            content.push(element.to_string());

        }

        content

    }

    pub fn get_task() {
        
        println!("{:?}",get_tasks(1));
    }

}