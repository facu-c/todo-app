
use std::fs::{self, File};

pub fn init() {
    let document_dir: String = match dirs::document_dir() {
        Some(dir) => {
            format!("{}/qacer",dir.display().to_string())
        },
        None => {
            String::from("no se pudo encotrar el path correcto")
        }
    };

    println!("{:?}", &document_dir);

    match fs::create_dir(&document_dir) {
        Ok(_) => {
            let _ = File::create(format!("{}/save_file",&document_dir));
        },
        Err(msg) => { eprintln!("Hubo un problema con la creacion del directorio raiz: {} ", msg); }
    }
}
