pub struct ConfigFile {
    id_task : i8,
    name_file : String,
    name_task : String,
    done : bool,
}

impl ConfigFile {
    
    pub fn new_file(name_file: String, name_task: String, id_task: i8) -> Self {
        
        Self{
            id_task,
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
    
    pub fn get_id(&self) -> i8 {
        self.id_task
    }

    pub fn set_done(mut self) {
        self.done = true;
    }
}