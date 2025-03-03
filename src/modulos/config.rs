pub struct ConfigFile {
    name_file : String,
    name_task : String,
    done : bool,
}

impl ConfigFile {
    
    pub fn new_file(name_file: String, name_task: String) -> Self {
        
        Self{
            name_file,
            name_task,
            done : false,
        }
    }

    pub fn get_name(&self) -> String {
        self.name_file.clone()
    }
    pub fn get_task(&self) -> String {
        self.name_task.clone()
    }
    
    pub fn get_done(&self) -> bool {
        self.done
    }
}