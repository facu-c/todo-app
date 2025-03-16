
use std::*;
use crate::modulos::config::ConfigFile;

fn create_file(config_file : ConfigFile) {
    
    let path = format!("{}/qacer/save_file",dirs::document_dir().unwrap().display());

    let mut data = fs::read_to_string(&path).unwrap();

    let contents = format!("{}\n{}{}", config_file.get_id(), config_file.get_name(), config_file.get_task());

    data.push_str(contents.as_str());

    let _ = fs::write(path.as_str(), data.as_str());

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
