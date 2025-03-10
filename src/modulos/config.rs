
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigFile { 
    id_task: u64,   
    name_task : String,
    desc_task : String,
    done : bool,
}

impl ConfigFile {
    
    pub fn new_file(name_task: String, desc_task: String, id_task: u64) -> Self {
        
        Self{
            id_task,
            name_task,
            desc_task,
            done : false,
        }

    }

    pub fn get_name(&self) -> String {
        self.name_task.clone()
    }
    pub fn get_task(&self) -> String {
        self.desc_task.clone()
    }
    
    // pub fn get_done(&self) -> bool {
    //     self.done
    // }
    
     pub fn get_id(&self) -> u64 {
         self.id_task
    }

    // pub fn set_done(mut self) {
    //    self.done = true;
    // }
}